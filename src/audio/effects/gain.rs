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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doubles_amplitude() {
        let mut e = GainEffect::new(2.0);
        let mut s = vec![0.5, -0.25, 0.0];
        e.process(&mut s, 48_000);
        assert!((s[0] - 1.0).abs() < 1e-6);
        assert!((s[1] + 0.5).abs() < 1e-6);
        assert!(s[2].abs() < 1e-6);
    }

    #[test]
    fn unity_gain_is_passthrough() {
        let mut e = GainEffect::new(1.0);
        let mut s = vec![0.3, -0.7, 0.1];
        let orig = s.clone();
        e.process(&mut s, 48_000);
        for (a, b) in s.iter().zip(orig.iter()) {
            assert!((a - b).abs() < 1e-6);
        }
    }
}
