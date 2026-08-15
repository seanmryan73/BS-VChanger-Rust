use std::collections::VecDeque;
use nnnoiseless::DenoiseState;
use super::AudioEffect;

const FRAME_SIZE: usize = DenoiseState::FRAME_SIZE;
const SCALE: f32 = 32_768.0;

pub struct NoiseSuppressionEffect {
    /// 0.0 = bypass (dry), 1.0 = full denoising.
    pub strength: f32,
    /// VAD probability below which the frame is gated to silence (0.0 = gate off).
    pub threshold: f32,
    state:   Box<DenoiseState<'static>>,
    in_buf:  Vec<f32>,
    out_buf: VecDeque<f32>,
    /// Reusable scratch for the dry copy. `samples.to_vec()` allocated this on
    /// **every** callback — a `malloc` in the highest-priority thread in the
    /// process, ~100 times a second. `malloc` can block on the allocator's lock,
    /// which defeats the `try_lock` discipline the engine is otherwise careful
    /// about.
    dry:     Vec<f32>,
    /// Reusable frame scratch, replacing a `drain(..).collect::<Vec<f32>>()` that
    /// allocated once **per frame** — so several times per callback, not once.
    frame:   [f32; FRAME_SIZE],
}

impl NoiseSuppressionEffect {
    pub fn new(strength: f32, threshold: f32) -> Self {
        Self {
            strength:  strength.clamp(0.0, 1.0),
            threshold: threshold.clamp(0.0, 1.0),
            state:     DenoiseState::new(),
            in_buf:    Vec::with_capacity(FRAME_SIZE * 2),
            out_buf:   VecDeque::with_capacity(FRAME_SIZE * 2),
            dry:       Vec::with_capacity(FRAME_SIZE * 2),
            frame:     [0.0; FRAME_SIZE],
        }
    }
}

impl AudioEffect for NoiseSuppressionEffect {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32) {
        if sample_rate != 48_000 {
            return;
        }
        if self.strength <= 0.001 && self.threshold <= 0.001 {
            return;
        }

        // Keep the dry input so we can fall back to it rather than inserting
        // silence when out_buf hasn't accumulated a full frame yet (e.g. on
        // the first callback after the chain is built). `clear` + `extend`
        // reuses the existing capacity; `to_vec()` allocated every callback.
        self.dry.clear();
        self.dry.extend_from_slice(samples);

        self.in_buf.extend(self.dry.iter().map(|&s| s * SCALE));

        while self.in_buf.len() >= FRAME_SIZE {
            self.frame.copy_from_slice(&self.in_buf[..FRAME_SIZE]);
            self.in_buf.drain(..FRAME_SIZE);

            let mut denoised = [0.0f32; FRAME_SIZE];
            let vad = self.state.process_frame(&mut denoised, &self.frame);

            let gate = if vad >= self.threshold { 1.0f32 } else { 0.0 };

            for (&orig, &den) in self.frame.iter().zip(denoised.iter()) {
                let mixed = (orig * (1.0 - self.strength) + den * self.strength) * gate;
                self.out_buf.push_back(mixed / SCALE);
            }
        }

        for (s, &fallback) in samples.iter_mut().zip(self.dry.iter()) {
            *s = self.out_buf.pop_front().unwrap_or(fallback);
        }
    }

    fn name(&self) -> &'static str { "Noise Suppression" }

    fn reset(&mut self) {
        self.in_buf.clear();
        self.out_buf.clear();
        self.dry.clear();
        self.state = DenoiseState::new();
    }
}
