use super::AudioEffect;

pub struct CompressorEffect {
    pub threshold: f32, // linear amplitude
    pub ratio:     f32,
    pub attack:    f32, // seconds
    pub release:   f32, // seconds
    envelope:      f32,
}

impl CompressorEffect {
    pub fn new(threshold: f32, ratio: f32, attack: f32, release: f32) -> Self {
        Self { threshold, ratio, attack, release, envelope: 0.0 }
    }
}

impl AudioEffect for CompressorEffect {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32) {
        let attack_coef  = (-1.0 / (self.attack  * sample_rate as f32)).exp();
        let release_coef = (-1.0 / (self.release * sample_rate as f32)).exp();
        for s in samples.iter_mut() {
            let level = s.abs();
            let coef = if level > self.envelope { attack_coef } else { release_coef };
            self.envelope = level + coef * (self.envelope - level);
            if self.envelope > self.threshold {
                let gain_reduction = self.threshold
                    + (self.envelope - self.threshold) / self.ratio;
                *s *= gain_reduction / self.envelope;
            }
        }
    }

    fn name(&self) -> &'static str { "Compressor" }

    fn reset(&mut self) { self.envelope = 0.0; }
}
