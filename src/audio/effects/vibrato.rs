use super::AudioEffect;

pub struct VibratoEffect {
    pub rate:  f32, // Hz
    pub depth: f32, // seconds (pitch modulation depth via delay)
    buffer:    Vec<f32>,
    write_pos: usize,
    lfo_phase: f32,
}

impl VibratoEffect {
    pub fn new(rate: f32, depth: f32) -> Self {
        Self { rate, depth, buffer: Vec::new(), write_pos: 0, lfo_phase: 0.0 }
    }

    fn init_buffer(&mut self, sample_rate: u32) {
        let max_delay = (self.depth * 2.0 * sample_rate as f32) as usize + 2;
        if self.buffer.len() != max_delay {
            self.buffer = vec![0.0; max_delay];
        }
    }
}

impl AudioEffect for VibratoEffect {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32) {
        self.init_buffer(sample_rate);
        let len = self.buffer.len();
        let lfo_inc = self.rate / sample_rate as f32;
        for s in samples.iter_mut() {
            self.buffer[self.write_pos] = *s;
            let delay = ((self.depth * sample_rate as f32)
                * (1.0 + (self.lfo_phase * 2.0 * std::f32::consts::PI).sin()) / 2.0) as usize;
            let read_pos = (self.write_pos + len - delay.min(len - 1)) % len;
            *s = self.buffer[read_pos];
            self.write_pos = (self.write_pos + 1) % len;
            self.lfo_phase = (self.lfo_phase + lfo_inc).fract();
        }
    }

    fn name(&self) -> &'static str { "Vibrato" }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
        self.lfo_phase = 0.0;
    }
}
