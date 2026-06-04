use super::AudioEffect;

pub struct NoiseSuppressionEffect {
    // nnnoiseless integration added in Stage 3
}

impl NoiseSuppressionEffect {
    pub fn new() -> Self {
        Self {}
    }
}

impl AudioEffect for NoiseSuppressionEffect {
    fn process(&mut self, samples: &mut [f32], _sample_rate: u32) {
        // TODO Stage 3: integrate nnnoiseless DenoiseState
        // Requires 48 kHz input, 480-sample frames
    }

    fn name(&self) -> &'static str { "Noise Suppression" }

    fn reset(&mut self) {}
}
