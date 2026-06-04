use std::collections::VecDeque;
use nnnoiseless::DenoiseState;
use super::AudioEffect;

// RNNoise requires exactly 480 samples per frame at 48 kHz.
const FRAME_SIZE: usize = DenoiseState::FRAME_SIZE;

// nnnoiseless operates in the 16-bit integer amplitude range.
const SCALE: f32 = 32_768.0;

pub struct NoiseSuppressionEffect {
    state:   Box<DenoiseState<'static>>,
    in_buf:  Vec<f32>,          // accumulates scaled input until a full frame is ready
    out_buf: VecDeque<f32>,     // holds processed output waiting to be drained
}

impl NoiseSuppressionEffect {
    pub fn new() -> Self {
        Self {
            state:   DenoiseState::new(),
            in_buf:  Vec::with_capacity(FRAME_SIZE * 2),
            out_buf: VecDeque::new(),
        }
    }
}

impl AudioEffect for NoiseSuppressionEffect {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32) {
        // RNNoise only works at 48 kHz — pass through at other rates.
        // Most WASAPI shared-mode configs default to 48 kHz on Windows.
        if sample_rate != 48_000 {
            return;
        }

        // Scale f32 [-1, 1] → i16 range for the RNNoise model.
        self.in_buf.extend(samples.iter().map(|s| s * SCALE));

        // Drain full 480-sample frames through the denoiser.
        while self.in_buf.len() >= FRAME_SIZE {
            let frame: Vec<f32> = self.in_buf.drain(..FRAME_SIZE).collect();
            let mut output_frame = [0.0f32; FRAME_SIZE];
            self.state.process_frame(&mut output_frame, &frame);
            // Scale back to [-1, 1] and queue output.
            self.out_buf.extend(output_frame.iter().map(|s| s / SCALE));
        }

        // Fill the caller's buffer from processed output.
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
