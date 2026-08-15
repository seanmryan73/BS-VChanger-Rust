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
        // A ratio of zero divides by zero in the gain computation. 1.0 is the
        // identity — no compression — which is the sane reading of "no ratio".
        Self { threshold, ratio: if ratio > 0.0 { ratio } else { 1.0 }, attack, release, envelope: 0.0 }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loud_signal_is_reduced() {
        // threshold 0.5, ratio 4:1
        let mut e = CompressorEffect::new(0.5, 4.0, 0.001, 0.1);
        // Loud steady signal well above threshold
        let mut loud = vec![0.9f32; 4800];
        e.process(&mut loud, 48_000);
        let avg: f32 = loud.iter().map(|x| x.abs()).sum::<f32>() / loud.len() as f32;
        assert!(avg < 0.9, "compressor did not reduce loud signal");
    }

    #[test]
    fn quiet_signal_passes_through() {
        let mut e = CompressorEffect::new(0.5, 4.0, 0.001, 0.1);
        let mut quiet = vec![0.1f32; 480];
        let orig: Vec<f32> = quiet.clone();
        e.process(&mut quiet, 48_000);
        for (a, b) in quiet.iter().zip(orig.iter()) {
            assert!((a - b).abs() < 0.01, "quiet signal should not be compressed");
        }
    }
}
