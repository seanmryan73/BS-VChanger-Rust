use super::AudioEffect;

pub struct EchoEffect {
    pub delay_secs: f32,
    pub feedback:   f32,
    pub wet:        f32,
    buffer:         Vec<f32>,
    write_pos:      usize,
}

impl EchoEffect {
    pub fn new(delay_secs: f32, feedback: f32, wet: f32) -> Self {
        Self { delay_secs, feedback, wet, buffer: Vec::new(), write_pos: 0 }
    }

    fn init_buffer(&mut self, sample_rate: u32) {
        let len = (self.delay_secs * sample_rate as f32) as usize + 1;
        if self.buffer.len() != len {
            self.buffer = vec![0.0; len];
            self.write_pos = 0;
        }
    }
}

impl AudioEffect for EchoEffect {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32) {
        self.init_buffer(sample_rate);
        let len = self.buffer.len();
        for s in samples.iter_mut() {
            let read_pos = (self.write_pos + 1) % len;
            let delayed  = self.buffer[read_pos];
            self.buffer[self.write_pos] = *s + delayed * self.feedback;
            self.write_pos = (self.write_pos + 1) % len;
            *s = *s * (1.0 - self.wet) + delayed * self.wet;
        }
    }

    fn name(&self) -> &'static str { "Echo" }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
    }
}
