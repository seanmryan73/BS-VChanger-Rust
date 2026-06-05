use super::AudioEffect;

pub struct CleanMicEffect {
    pub cutoff_hz: f32,
    x_prev: f32,
    y_prev: f32,
}

impl CleanMicEffect {
    pub fn new(cutoff_hz: f32) -> Self {
        Self {
            cutoff_hz: cutoff_hz.clamp(20.0, 400.0),
            x_prev: 0.0,
            y_prev: 0.0,
        }
    }
}

impl AudioEffect for CleanMicEffect {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32) {
        // y[n] = x[n] - x[n-1] + R * y[n-1]  (single-pole high-pass)
        let r = (1.0 - 2.0 * std::f32::consts::PI * self.cutoff_hz / sample_rate as f32)
            .clamp(0.0, 0.9999);
        for s in samples.iter_mut() {
            let y = *s - self.x_prev + r * self.y_prev;
            self.x_prev = *s;
            self.y_prev = y;
            *s = y;
        }
    }

    fn name(&self) -> &'static str { "Clean Mic" }

    fn reset(&mut self) {
        self.x_prev = 0.0;
        self.y_prev = 0.0;
    }
}
