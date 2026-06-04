use super::AudioEffect;

pub struct RingModEffect {
    pub carrier_freq: f32, // Hz
    phase:            f32,
}

impl RingModEffect {
    pub fn new(carrier_freq: f32) -> Self {
        Self { carrier_freq, phase: 0.0 }
    }
}

impl AudioEffect for RingModEffect {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32) {
        let inc = self.carrier_freq / sample_rate as f32;
        for s in samples.iter_mut() {
            *s *= (self.phase * 2.0 * std::f32::consts::PI).sin();
            self.phase = (self.phase + inc).fract();
        }
    }

    fn name(&self) -> &'static str { "Ring Mod" }

    fn reset(&mut self) { self.phase = 0.0; }
}
