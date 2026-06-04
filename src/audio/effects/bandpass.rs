use super::AudioEffect;

pub struct BandpassFilterEffect {
    pub center_freq: f32,
    pub q: f32,
    x1: f32, x2: f32, y1: f32, y2: f32,
}

impl BandpassFilterEffect {
    pub fn new(center_freq: f32, q: f32) -> Self {
        Self { center_freq, q, x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0 }
    }

    fn compute_coeffs(&self, sample_rate: u32) -> (f32, f32, f32, f32, f32) {
        let w0 = 2.0 * std::f32::consts::PI * self.center_freq / sample_rate as f32;
        let alpha = w0.sin() / (2.0 * self.q);
        let b0 = alpha;
        let b1 = 0.0f32;
        let b2 = -alpha;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * w0.cos();
        let a2 = 1.0 - alpha;
        (b0 / a0, b1 / a0, b2 / a0, a1 / a0, a2 / a0)
    }
}

impl AudioEffect for BandpassFilterEffect {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32) {
        let (b0, b1, b2, a1, a2) = self.compute_coeffs(sample_rate);
        for s in samples.iter_mut() {
            let x0 = *s;
            let y0 = b0 * x0 + b1 * self.x1 + b2 * self.x2
                             - a1 * self.y1 - a2 * self.y2;
            self.x2 = self.x1; self.x1 = x0;
            self.y2 = self.y1; self.y1 = y0;
            *s = y0;
        }
    }

    fn name(&self) -> &'static str { "Bandpass Filter" }

    fn reset(&mut self) {
        self.x1 = 0.0; self.x2 = 0.0; self.y1 = 0.0; self.y2 = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rms(s: &[f32]) -> f32 {
        (s.iter().map(|x| x * x).sum::<f32>() / s.len() as f32).sqrt()
    }

    fn sine_wave(freq: f32, sample_rate: u32, n: usize) -> Vec<f32> {
        (0..n)
            .map(|i| (2.0 * std::f32::consts::PI * freq * i as f32 / sample_rate as f32).sin())
            .collect()
    }

    #[test]
    fn passes_center_frequency() {
        let sr = 48_000u32;
        let center = 1_000.0f32;
        let mut e = BandpassFilterEffect::new(center, 1.0);
        let mut s = sine_wave(center, sr, 4800);
        let before = rms(&s);
        e.process(&mut s, sr);
        let after = rms(&s);
        // Center frequency should pass through with reasonable energy
        assert!(after > before * 0.5, "center frequency attenuated too much");
    }

    #[test]
    fn attenuates_out_of_band() {
        let sr = 48_000u32;
        let mut e = BandpassFilterEffect::new(1_000.0, 1.0);
        // Far-out-of-band signal: 10 kHz
        let mut s = sine_wave(10_000.0, sr, 4800);
        e.process(&mut s, sr);
        assert!(rms(&s) < 0.2, "out-of-band signal not attenuated");
    }
}
