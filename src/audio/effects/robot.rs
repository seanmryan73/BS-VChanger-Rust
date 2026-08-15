use super::AudioEffect;

/// Corner frequency of the internal DC blocker, in Hz.
const DC_BLOCK_HZ: f32 = 20.0;

/// Rectify-and-comb "robot" voice, with the DC blocker **inside the effect**.
///
/// `s.abs()` is strictly non-negative, so the output rides on a large DC offset.
/// Into a virtual cable that eats headroom and can thump a downstream limiter.
///
/// It belongs in the effect rather than in the `Robot` preset because this app
/// lets users build **custom profiles** from the individual effects — a
/// user-authored profile using this effect would otherwise still get the raw
/// offset.
///
/// The `Robot` preset's own 80 Hz `CleanMic` stage is **not** redundant with
/// this: 80 Hz is *voicing*, thinning the tone, while this 20 Hz stage only
/// removes the offset. Correctness lives here, voicing lives in the preset.
pub struct RobotModulationEffect {
    pub pitch_hz:  f32, // fundamental comb frequency
    buffer:        Vec<f32>,
    write_pos:     usize,
    // DC blocker state: y[n] = x[n] - x[n-1] + R * y[n-1]
    x_prev:        f32,
    y_prev:        f32,
}

impl RobotModulationEffect {
    pub fn new(pitch_hz: f32) -> Self {
        // `.max(1.0)`: a pitch of 0 makes `rate / pitch_hz` infinite, which
        // saturates to usize::MAX on the cast, wraps to 0 on the `+ 1`, and then
        // panics on the modulo in `process`. Reachable from a saved profile.
        Self { pitch_hz: pitch_hz.max(1.0), buffer: Vec::new(), write_pos: 0, x_prev: 0.0, y_prev: 0.0 }
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
        let r = (1.0 - 2.0 * std::f32::consts::PI * DC_BLOCK_HZ / sample_rate as f32).clamp(0.0, 0.9999);

        for s in samples.iter_mut() {
            // Full-wave rectify → comb filter at pitch_hz
            let rect = s.abs();
            let delayed = self.buffer[self.write_pos];
            self.buffer[self.write_pos] = rect;
            self.write_pos = (self.write_pos + 1) % len;
            let combed = rect - delayed * 0.8;

            // Single-pole high-pass, same form as CleanMicEffect. Removes the
            // offset the rectifier above necessarily creates.
            let y = combed - self.x_prev + r * self.y_prev;
            self.x_prev = combed;
            self.y_prev = y;
            *s = y;
        }
    }

    fn name(&self) -> &'static str { "Robot" }

    fn reset(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
        self.x_prev = 0.0;
        self.y_prev = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The gap this closes: rectification is strictly non-negative, so without a
    /// blocker the output rides on a large DC offset that eats headroom and can
    /// thump a downstream limiter. `BS-ChatBot` blocks it in the *profile*, which
    /// leaves any user-authored profile using this effect unprotected.
    #[test]
    fn the_effect_blocks_the_dc_its_own_rectifier_creates() {
        let mut e = RobotModulationEffect::new(120.0);
        let mut s: Vec<f32> = (0..48_000)
            .map(|i| (i as f32 * 2.0 * std::f32::consts::PI * 200.0 / 48_000.0).sin() * 0.5)
            .collect();
        e.process(&mut s, 48_000);

        // Skip the filter's settling time, then check the mean sits at zero.
        let tail = &s[24_000..];
        let mean = tail.iter().sum::<f32>() / tail.len() as f32;
        assert!(mean.abs() < 0.01, "DC offset survived: {mean}");
    }

    /// A blocker that also killed the signal would pass the test above.
    #[test]
    fn the_robot_voice_is_still_audible_after_blocking() {
        let mut e = RobotModulationEffect::new(120.0);
        let mut s: Vec<f32> = (0..48_000)
            .map(|i| (i as f32 * 2.0 * std::f32::consts::PI * 200.0 / 48_000.0).sin() * 0.5)
            .collect();
        e.process(&mut s, 48_000);

        let tail = &s[24_000..];
        let peak = tail.iter().fold(0.0f32, |a, b| a.max(b.abs()));
        assert!(peak > 0.01, "the effect output nothing audible (peak {peak})");
    }
}
