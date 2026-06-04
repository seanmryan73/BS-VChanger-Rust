use super::AudioEffect;

pub struct TremoloEffect {
    pub rate:      f32, // Hz
    pub depth:     f32, // 0.0–1.0
    lfo_phase:     f32,
}

impl TremoloEffect {
    pub fn new(rate: f32, depth: f32) -> Self {
        Self { rate, depth, lfo_phase: 0.0 }
    }
}

impl AudioEffect for TremoloEffect {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32) {
        let lfo_inc = self.rate / sample_rate as f32;
        for s in samples.iter_mut() {
            let lfo = 1.0 - self.depth * (1.0 + (self.lfo_phase * 2.0 * std::f32::consts::PI).sin()) / 2.0;
            *s *= lfo;
            self.lfo_phase = (self.lfo_phase + lfo_inc).fract();
        }
    }

    fn name(&self) -> &'static str { "Tremolo" }

    fn reset(&mut self) { self.lfo_phase = 0.0; }
}
