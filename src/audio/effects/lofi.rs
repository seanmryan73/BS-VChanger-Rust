use super::AudioEffect;

pub struct LoFiEffect {
    pub bit_depth:       f32, // 4.0–16.0
    pub downsample_rate: u32, // keep every Nth sample
    counter:             u32,
    held:                f32,
}

impl LoFiEffect {
    pub fn new(bit_depth: f32, downsample_rate: u32) -> Self {
        // `2^bit_depth` overflows f32 to inf past ~128, and every sample then
        // becomes NaN. `downsample_rate` of 0 quantises every sample — a silent
        // no-op that says nothing about why the effect did nothing.
        Self {
            bit_depth:       bit_depth.clamp(1.0, 24.0),
            downsample_rate: downsample_rate.max(1),
            counter:         0,
            held:            0.0,
        }
    }
}

impl AudioEffect for LoFiEffect {
    fn process(&mut self, samples: &mut [f32], _sample_rate: u32) {
        let levels = (2.0f32).powf(self.bit_depth) / 2.0;
        for s in samples.iter_mut() {
            self.counter += 1;
            if self.counter >= self.downsample_rate {
                self.counter = 0;
                self.held = (*s * levels).round() / levels;
            }
            *s = self.held;
        }
    }

    fn name(&self) -> &'static str { "Lo-Fi" }

    fn reset(&mut self) { self.counter = 0; self.held = 0.0; }
}
