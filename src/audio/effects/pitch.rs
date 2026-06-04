use super::AudioEffect;

pub struct PitchResampleEffect {
    pub semitones: f32,
    // rubato resampler wired in Stage 3
}

impl PitchResampleEffect {
    pub fn new(semitones: f32) -> Self {
        Self { semitones }
    }
}

impl AudioEffect for PitchResampleEffect {
    fn process(&mut self, samples: &mut [f32], _sample_rate: u32) {
        // TODO Stage 3: rubato async resampler for pitch shift
    }

    fn name(&self) -> &'static str { "Pitch Shift" }

    fn reset(&mut self) {}
}
