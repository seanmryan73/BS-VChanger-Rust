use super::AudioEffect;

pub struct DeEsserEffect {
    pub threshold: f32,
    pub reduction: f32,
    // Biquad state for the sibilance detector band (4–8 kHz)
    x1: f32, x2: f32, y1: f32, y2: f32,
    envelope: f32,
}

impl DeEsserEffect {
    pub fn new(threshold: f32, reduction: f32) -> Self {
        Self { threshold, reduction, x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0, envelope: 0.0 }
    }
}

impl AudioEffect for DeEsserEffect {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32) {
        // Bandpass 6 kHz to detect sibilance
        let w0 = 2.0 * std::f32::consts::PI * 6000.0 / sample_rate as f32;
        let alpha = w0.sin() / (2.0 * 1.5);
        let b0 = alpha / (1.0 + alpha);
        let b2 = -alpha / (1.0 + alpha);
        let a1 = -2.0 * w0.cos() / (1.0 + alpha);
        let a2 = (1.0 - alpha) / (1.0 + alpha);

        for s in samples.iter_mut() {
            let x0 = *s;
            let detect = b0 * x0 + b2 * self.x2 - a1 * self.y1 - a2 * self.y2;
            self.x2 = self.x1; self.x1 = x0;
            self.y2 = self.y1; self.y1 = detect;

            self.envelope = detect.abs().max(self.envelope * 0.999);
            if self.envelope > self.threshold {
                *s *= self.reduction;
            }
        }
    }

    fn name(&self) -> &'static str { "De-Esser" }

    fn reset(&mut self) {
        self.x1 = 0.0; self.x2 = 0.0; self.y1 = 0.0; self.y2 = 0.0;
        self.envelope = 0.0;
    }
}
