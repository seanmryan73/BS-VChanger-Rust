use super::AudioEffect;

#[derive(Clone, Copy, PartialEq)]
pub enum ClipMode { Soft, Hard }

pub struct DistortionEffect {
    pub drive: f32,
    pub mode:  ClipMode,
}

impl DistortionEffect {
    pub fn new(drive: f32, mode: ClipMode) -> Self {
        Self { drive, mode }
    }
}

impl AudioEffect for DistortionEffect {
    fn process(&mut self, samples: &mut [f32], _sample_rate: u32) {
        for s in samples.iter_mut() {
            let driven = *s * self.drive;
            *s = match self.mode {
                ClipMode::Soft => driven.tanh(),
                ClipMode::Hard => driven.clamp(-1.0, 1.0),
            };
        }
    }

    fn name(&self) -> &'static str { "Distortion" }

    fn reset(&mut self) {}
}
