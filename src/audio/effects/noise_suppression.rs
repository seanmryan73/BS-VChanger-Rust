use std::collections::VecDeque;
use nnnoiseless::DenoiseState;
use super::AudioEffect;

const FRAME_SIZE: usize = DenoiseState::FRAME_SIZE;
const SCALE: f32 = 32_768.0;

pub struct NoiseSuppressionEffect {
    /// 0.0 = bypass (dry), 1.0 = full denoising. Wet/dry mix of the RNNoise output.
    pub strength: f32,
    state:        Box<DenoiseState<'static>>,
    in_buf:       Vec<f32>,
    out_buf:      VecDeque<f32>,
}

impl NoiseSuppressionEffect {
    pub fn new(strength: f32) -> Self {
        Self {
            strength: strength.clamp(0.0, 1.0),
            state:    DenoiseState::new(),
            in_buf:   Vec::with_capacity(FRAME_SIZE * 2),
            out_buf:  VecDeque::new(),
        }
    }
}

impl AudioEffect for NoiseSuppressionEffect {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32) {
        if sample_rate != 48_000 {
            return; // RNNoise requires 48 kHz
        }
        if self.strength <= 0.001 {
            return; // fully bypassed — skip the expensive FFT
        }

        self.in_buf.extend(samples.iter().map(|s| s * SCALE));

        while self.in_buf.len() >= FRAME_SIZE {
            let frame: Vec<f32> = self.in_buf.drain(..FRAME_SIZE).collect();
            let mut denoised = [0.0f32; FRAME_SIZE];
            self.state.process_frame(&mut denoised, &frame);

            // Wet/dry mix: blend denoised with original at the configured strength
            for (orig, den) in frame.iter().zip(denoised.iter()) {
                let mixed = orig * (1.0 - self.strength) + den * self.strength;
                self.out_buf.push_back(mixed / SCALE);
            }
        }

        for s in samples.iter_mut() {
            *s = self.out_buf.pop_front().unwrap_or(0.0);
        }
    }

    fn name(&self) -> &'static str { "Noise Suppression" }

    fn reset(&mut self) {
        self.in_buf.clear();
        self.out_buf.clear();
        self.state = DenoiseState::new();
    }
}
