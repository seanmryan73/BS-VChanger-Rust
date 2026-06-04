use super::AudioEffect;

pub struct CleanMicEffect {
    prev: f32,
}

impl CleanMicEffect {
    pub fn new() -> Self {
        Self { prev: 0.0 }
    }
}

impl AudioEffect for CleanMicEffect {
    fn process(&mut self, samples: &mut [f32], _sample_rate: u32) {
        // Simple single-pole high-pass to remove DC offset
        for s in samples.iter_mut() {
            let out = *s - self.prev + 0.995 * self.prev;
            self.prev = *s;
            *s = out;
        }
    }

    fn name(&self) -> &'static str { "Clean Mic" }

    fn reset(&mut self) {
        self.prev = 0.0;
    }
}
