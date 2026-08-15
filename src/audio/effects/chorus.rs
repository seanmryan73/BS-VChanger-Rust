use super::AudioEffect;

pub struct ChorusEffect {
    pub rate:  f32, // LFO Hz
    pub depth: f32, // seconds
    pub wet:   f32,
    buffer:    Vec<f32>,
    write_pos: usize,
    lfo_phase: f32,
}

impl ChorusEffect {
    pub fn new(rate: f32, depth: f32, wet: f32) -> Self {
        Self { rate, depth, wet, buffer: Vec::new(), write_pos: 0, lfo_phase: 0.0 }
    }

    fn init_buffer(&mut self, sample_rate: u32) {
        let max_delay = (self.depth * 2.0 * sample_rate as f32) as usize + 2;
        if self.buffer.len() != max_delay {
            self.buffer = vec![0.0; max_delay];
        }
    }
}

impl AudioEffect for ChorusEffect {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32) {
        self.init_buffer(sample_rate);
        let len = self.buffer.len();
        let lfo_inc = self.rate / sample_rate as f32;
        for s in samples.iter_mut() {
            self.buffer[self.write_pos] = *s;
            let delay_samples = (self.depth * sample_rate as f32
                * (1.0 + (self.lfo_phase * 2.0 * std::f32::consts::PI).sin()) / 2.0) as usize;
            // Clamped: a delay longer than the buffer underflows
            // `write_pos + len - delay` and panics. Flanger and vibrato already
            // clamped; chorus did not.
            let read_pos = (self.write_pos + len - delay_samples.min(len - 1)) % len;
            let delayed = self.buffer[read_pos];
            self.write_pos = (self.write_pos + 1) % len;
            self.lfo_phase = (self.lfo_phase + lfo_inc).fract();
            *s = *s * (1.0 - self.wet) + delayed * self.wet;
        }
    }

    fn name(&self) -> &'static str { "Chorus" }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
        self.lfo_phase = 0.0;
    }
}
