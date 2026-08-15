use super::AudioEffect;

/// The rate these Freeverb delay constants were tuned at.
const FREEVERB_RATE: f64 = 44_100.0;
const COMB_DELAYS: [usize; 4]   = [1557, 1617, 1491, 1422];
const ALLPASS_DELAYS: [usize; 2] = [225,  556];
const DAMP: f32 = 0.5;

struct CombFilter { buf: Vec<f32>, pos: usize, feedback: f32, damp_store: f32 }
struct AllpassFilter { buf: Vec<f32>, pos: usize }

impl CombFilter {
    fn new(delay: usize, feedback: f32) -> Self {
        // `.max(1)`: the scaling below floors, so a low enough rate reaches 0
        // and `% 0` panics inside the audio callback.
        Self { buf: vec![0.0; delay.max(1)], pos: 0, feedback, damp_store: 0.0 }
    }
    fn process(&mut self, input: f32) -> f32 {
        let out = self.buf[self.pos];
        self.damp_store = out * (1.0 - DAMP) + self.damp_store * DAMP;
        self.buf[self.pos] = input + self.damp_store * self.feedback;
        self.pos = (self.pos + 1) % self.buf.len();
        out
    }
}

impl AllpassFilter {
    fn new(delay: usize) -> Self {
        Self { buf: vec![0.0; delay.max(1)], pos: 0 }
    }
    fn process(&mut self, input: f32) -> f32 {
        let buf_out = self.buf[self.pos];
        let out = -input + buf_out;
        self.buf[self.pos] = input + buf_out * 0.5;
        self.pos = (self.pos + 1) % self.buf.len();
        out
    }
}

/// Cut-down Freeverb: four damped feedback combs into two series allpasses.
///
/// ## The delay lines scale with the sample rate
///
/// They did not before, and the constants above are only correct at 44.1 kHz.
/// At the 48 kHz nearly every Windows endpoint actually runs, every delay was
/// **8.8% short** — audible as a smaller room than the preset asks for, but not
/// obviously *wrong*, which is why it went unnoticed for so long. `Cathedral`
/// and `Echo Chamber` are the presets where it shows most.
///
/// Built **lazily in `process`** rather than taken as a constructor argument, so
/// that changing the device mid-session re-scales a live chain instead of
/// requiring the whole chain to be rebuilt. Switching mic device while running
/// is a real path in this app.
pub struct ReverbEffect {
    pub room_size: f32, // 0.0–1.0
    pub wet:       f32,
    combs:     Vec<CombFilter>,
    allpasses: Vec<AllpassFilter>,
    built_at_rate: u32,
}

impl ReverbEffect {
    pub fn new(room_size: f32, wet: f32) -> Self {
        Self {
            room_size,
            wet,
            combs:     Vec::new(),
            allpasses: Vec::new(),
            built_at_rate: 0,
        }
    }

    /// Allocates the delay lines for `sample_rate`. No-ops once they are right,
    /// so the steady-state realtime path never allocates.
    fn init_buffers(&mut self, sample_rate: u32) {
        if self.built_at_rate == sample_rate && !self.combs.is_empty() {
            return;
        }
        let feedback = 0.84 + self.room_size * 0.15;
        // f64, not f32: `1557 * 44100` is past the range f32 represents exactly,
        // so doing this in f32 makes even the identity case drift by a sample.
        let scale = |d: usize| ((d as f64 * sample_rate as f64) / FREEVERB_RATE) as usize;
        self.combs = COMB_DELAYS.iter().map(|&d| CombFilter::new(scale(d), feedback)).collect();
        self.allpasses = ALLPASS_DELAYS.iter().map(|&d| AllpassFilter::new(scale(d))).collect();
        self.built_at_rate = sample_rate;
    }
}

impl AudioEffect for ReverbEffect {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32) {
        self.init_buffers(sample_rate);
        for s in samples.iter_mut() {
            let input = *s * 0.015;
            let mut sum = 0.0;
            for c in &mut self.combs { sum += c.process(input); }
            let mut out = sum;
            for a in &mut self.allpasses { out = a.process(out); }
            *s = *s * (1.0 - self.wet) + out * self.wet;
        }
    }

    fn name(&self) -> &'static str { "Reverb" }

    fn reset(&mut self) {
        for c in &mut self.combs { c.buf.fill(0.0); c.pos = 0; c.damp_store = 0.0; }
        for a in &mut self.allpasses { a.buf.fill(0.0); a.pos = 0; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn built(room_size: f32, rate: u32) -> ReverbEffect {
        let mut r = ReverbEffect::new(room_size, 0.5);
        r.init_buffers(rate);
        r
    }

    /// At 44.1 kHz the scaling is the identity — the constants were tuned there,
    /// so this is the case that must not move.
    #[test]
    fn forty_four_one_uses_the_untouched_constants() {
        let r = built(0.7, 44_100);
        let lens: Vec<usize> = r.combs.iter().map(|c| c.buf.len()).collect();
        assert_eq!(lens, COMB_DELAYS.to_vec());
    }

    /// The actual bug: at 48 kHz every delay was 8.8% short.
    #[test]
    fn forty_eight_k_is_longer_than_forty_four_one() {
        let a = built(0.7, 44_100);
        let b = built(0.7, 48_000);
        for (x, y) in a.combs.iter().zip(b.combs.iter()) {
            assert!(
                y.buf.len() > x.buf.len(),
                "48 kHz delay {} should exceed 44.1 kHz delay {}",
                y.buf.len(),
                x.buf.len()
            );
        }
        // 48000/44100 = 1.088..., so the first comb goes 1557 -> 1694.
        assert_eq!(b.combs[0].buf.len(), 1_694);
    }

    /// The `% 0` guard. The scale floors, so a low enough rate reaches zero and
    /// the modulo in `process` panics — inside the audio callback.
    #[test]
    fn a_very_low_rate_cannot_produce_a_zero_length_delay_line() {
        let r = built(0.7, 1);
        assert!(r.combs.iter().all(|c| !c.buf.is_empty()));
        assert!(r.allpasses.iter().all(|a| !a.buf.is_empty()));
    }

    /// Switching mic device mid-session changes the rate under a live chain.
    /// The lazy build exists so that re-scales instead of needing a rebuild.
    #[test]
    fn rebuilds_when_the_rate_changes_under_a_live_chain() {
        let mut r = ReverbEffect::new(0.7, 0.5);
        let mut s = vec![0.1f32; 256];
        r.process(&mut s, 44_100);
        let at_44 = r.combs[0].buf.len();
        r.process(&mut s, 48_000);
        assert!(r.combs[0].buf.len() > at_44, "a rate change must re-scale the delay lines");
    }
}
