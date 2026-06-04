use super::AudioEffect;

pub struct RobotModulationEffect {
    pub pitch_hz:  f32, // fundamental comb frequency
    buffer:        Vec<f32>,
    write_pos:     usize,
}

impl RobotModulationEffect {
    pub fn new(pitch_hz: f32) -> Self {
        Self { pitch_hz, buffer: Vec::new(), write_pos: 0 }
    }

    fn init_buffer(&mut self, sample_rate: u32) {
        let delay = (sample_rate as f32 / self.pitch_hz) as usize + 1;
        if self.buffer.len() != delay {
            self.buffer = vec![0.0; delay];
            self.write_pos = 0;
        }
    }
}

impl AudioEffect for RobotModulationEffect {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32) {
        self.init_buffer(sample_rate);
        let len = self.buffer.len();
        for s in samples.iter_mut() {
            // Full-wave rectify → comb filter at pitch_hz
            let rect = s.abs();
            let delayed = self.buffer[self.write_pos];
            self.buffer[self.write_pos] = rect;
            self.write_pos = (self.write_pos + 1) % len;
            *s = rect - delayed * 0.8;
        }
    }

    fn name(&self) -> &'static str { "Robot" }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
    }
}
