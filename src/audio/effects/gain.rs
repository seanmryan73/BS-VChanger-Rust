use super::AudioEffect;

pub struct GainEffect {
    pub gain: f32,
}

impl GainEffect {
    pub fn new(gain: f32) -> Self {
        Self { gain }
    }
}

impl AudioEffect for GainEffect {
    fn process(&mut self, samples: &mut [f32], _sample_rate: u32) {
        for s in samples.iter_mut() {
            *s *= self.gain;
        }
    }

    fn name(&self) -> &'static str { "Gain" }

    fn reset(&mut self) {}
}
