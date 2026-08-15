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
        // `.max(1)`: a zero-length buffer makes the `% len` in `process` a
        // divide-by-zero panic, inside the audio callback.
        let len = ((self.delay_secs * sample_rate as f32) as usize + 1).max(1);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delayed_signal_appears() {
        // 1-second delay at 100 Hz = 100-sample delay; buffer len = 101.
        // Impulse at index 0 → echo surfaces at index 100 (after one full loop).
        let mut e = EchoEffect::new(1.0, 0.5, 1.0);
        let mut s = vec![0.0f32; 200];
        s[0] = 1.0;
        e.process(&mut s, 100);
        assert!(s[100].abs() > 0.5, "echo not audible at delay position");
    }

    #[test]
    fn reset_clears_delay_line() {
        let mut e = EchoEffect::new(0.01, 0.5, 1.0);
        let mut s = vec![1.0f32; 48];
        e.process(&mut s, 48_000);
        e.reset();
        let mut silence = vec![0.0f32; 48];
        e.process(&mut silence, 48_000);
        // After reset, no residual echo on the first buffer
        assert!(silence.iter().all(|&x| x.abs() < 1e-6));
    }
}
