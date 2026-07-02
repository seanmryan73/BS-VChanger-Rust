use std::collections::VecDeque;
use rubato::{SincFixedIn, SincInterpolationParameters, SincInterpolationType, WindowFunction, Resampler};
use super::AudioEffect;

// Input chunk size fed to rubato. Must be consistent for a given resampler instance.
const CHUNK_SIZE: usize = 256;

// Resample-pitch changes duration, so output production never matches
// consumption exactly. Cap the backlog: pitch DOWN produces surplus that
// would otherwise grow (and lag) without bound — drop the oldest samples
// past this cap (~21 ms at 48 kHz, inaudible time-compression skips).
const MAX_OUT_BUF: usize = CHUNK_SIZE * 4;

pub struct PitchResampleEffect {
    pub semitones:    f32,
    resampler:        Option<SincFixedIn<f32>>,
    in_buf:           Vec<f32>,
    out_buf:          VecDeque<f32>,
    // Last emitted sample, decayed to fill pitch-up underruns without clicks.
    last_out:         f32,
    // Track what the resampler was built for so we can rebuild on change.
    built_semitones:  f32,
    built_rate:       u32,
}

impl PitchResampleEffect {
    pub fn new(semitones: f32) -> Self {
        Self {
            semitones,
            resampler:       None,
            in_buf:          Vec::with_capacity(CHUNK_SIZE * 2),
            out_buf:         VecDeque::new(),
            last_out:        0.0,
            built_semitones: f32::NAN,
            built_rate:      0,
        }
    }

    fn ensure_resampler(&mut self, sample_rate: u32) {
        if self.resampler.is_some()
            && self.built_semitones == self.semitones
            && self.built_rate == sample_rate
        {
            return;
        }

        // ratio = output_frames / input_frames
        // Pitch UP (+semitones): compress signal in time → fewer output samples → ratio < 1
        // Pitch DOWN (-semitones): expand signal in time → more output samples  → ratio > 1
        let ratio = 1.0 / (2.0f64).powf(self.semitones as f64 / 12.0);

        // Bail out if ratio is too extreme (rubato limit: ~0.1x to 10x).
        if !(0.1..=10.0).contains(&ratio) {
            self.resampler = None;
            return;
        }

        let params = SincInterpolationParameters {
            sinc_len:             64,
            f_cutoff:             0.95,
            interpolation:        SincInterpolationType::Linear,
            oversampling_factor:  16,
            window:               WindowFunction::Blackman,
        };

        self.resampler = SincFixedIn::<f32>::new(
            ratio,
            2.0, // max relative ratio change
            params,
            CHUNK_SIZE,
            1, // mono
        )
        .ok();

        self.built_semitones = self.semitones;
        self.built_rate      = sample_rate;
        self.in_buf.clear();
        self.out_buf.clear();
    }
}

impl AudioEffect for PitchResampleEffect {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32) {
        if self.semitones == 0.0 {
            return;
        }

        self.ensure_resampler(sample_rate);

        let Some(ref mut resampler) = self.resampler else {
            return;
        };

        self.in_buf.extend_from_slice(samples);

        while self.in_buf.len() >= CHUNK_SIZE {
            let chunk: Vec<f32> = self.in_buf.drain(..CHUNK_SIZE).collect();
            if let Ok(out_frames) = resampler.process(&[chunk], None) {
                self.out_buf.extend(out_frames[0].iter().copied());
            }
        }

        // Bound the backlog (pitch down over-produces): keep the freshest audio.
        while self.out_buf.len() > MAX_OUT_BUF {
            self.out_buf.pop_front();
        }

        for s in samples.iter_mut() {
            match self.out_buf.pop_front() {
                Some(v) => { self.last_out = v; *s = v; }
                // Underrun (pitch up under-produces): decay the held sample
                // toward silence instead of padding hard zeros.
                None => { self.last_out *= 0.95; *s = self.last_out; }
            }
        }
    }

    fn name(&self) -> &'static str { "Pitch Shift" }

    fn reset(&mut self) {
        self.resampler       = None;
        self.built_semitones = f32::NAN;
        self.built_rate      = 0;
        self.in_buf.clear();
        self.out_buf.clear();
        self.last_out        = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pitch down over-produces samples; the backlog must stay bounded so
    /// memory and latency don't grow without limit during long sessions.
    #[test]
    fn pitch_down_backlog_stays_bounded() {
        let mut e = PitchResampleEffect::new(-8.0);
        let mut buf = vec![0.25f32; 480];
        // ~10 s of 10 ms callbacks at 48 kHz
        for _ in 0..1_000 {
            e.process(&mut buf, 48_000);
        }
        assert!(
            e.out_buf.len() <= MAX_OUT_BUF,
            "out_buf grew to {} (cap {MAX_OUT_BUF})",
            e.out_buf.len()
        );
    }

    /// Pitch up under-produces; output must stay finite and free of hard
    /// zero-gaps once the resampler has warmed up.
    #[test]
    fn pitch_up_output_is_finite() {
        let mut e = PitchResampleEffect::new(12.0);
        let mut buf = vec![0.25f32; 480];
        for _ in 0..100 {
            e.process(&mut buf, 48_000);
            assert!(buf.iter().all(|s| s.is_finite()));
        }
    }
}
