pub mod audio_effect;
pub mod bandpass;
pub mod chain;
pub mod chorus;
pub mod clean_mic;
pub mod compressor;
pub mod de_esser;
pub mod distortion;
pub mod echo;
pub mod flanger;
pub mod gain;
pub mod lofi;
pub mod noise_suppression;
pub mod pitch;
pub mod reverb;
pub mod ring_mod;
pub mod robot;
pub mod tremolo;
pub mod vibrato;

pub use audio_effect::AudioEffect;
pub use chain::EffectChain;

/// Cross-cutting properties every effect must hold, at every rate this app runs at.
///
/// Back-ported from `BS-VChanger-and-ChatBot` (`crates/bs-audio`), where it was
/// written during the merge. **This suite is the reason that repo exists in the
/// shape it does** — it is what would have caught the 44.1 kHz-only Freeverb
/// constants years ago, and until now nothing here tested reverb, robot,
/// noise_suppression, chorus, flanger, vibrato, distortion, lofi, ring_mod,
/// tremolo, clean_mic or de_esser at all.
///
/// It goes in **before** the DSP fixes it exists to protect, so a failure is
/// attributable to the code that is already here rather than to the port.
#[cfg(test)]
mod suite {
    use super::audio_effect::AudioEffect;
    use super::bandpass::BandpassFilterEffect;
    use super::chorus::ChorusEffect;
    use super::clean_mic::CleanMicEffect;
    use super::compressor::CompressorEffect;
    use super::de_esser::DeEsserEffect;
    use super::distortion::{ClipMode, DistortionEffect};
    use super::echo::EchoEffect;
    use super::flanger::FlangerEffect;
    use super::gain::GainEffect;
    use super::lofi::LoFiEffect;
    use super::noise_suppression::NoiseSuppressionEffect;
    use super::pitch::PitchResampleEffect;
    use super::reverb::ReverbEffect;
    use super::ring_mod::RingModEffect;
    use super::robot::RobotModulationEffect;
    use super::tremolo::TremoloEffect;
    use super::vibrato::VibratoEffect;

    fn every_effect() -> Vec<Box<dyn AudioEffect>> {
        vec![
            Box::new(GainEffect::new(1.5)),
            Box::new(CleanMicEffect::new(80.0)),
            Box::new(BandpassFilterEffect::new(1_000.0, 1.0)),
            Box::new(CompressorEffect::new(0.5, 4.0, 0.005, 0.1)),
            Box::new(DeEsserEffect::new(0.1, 0.5)),
            Box::new(NoiseSuppressionEffect::new(0.8, 0.1)),
            Box::new(PitchResampleEffect::new(5.0)),
            Box::new(EchoEffect::new(0.2, 0.4, 0.4)),
            Box::new(ReverbEffect::new(0.7, 0.3)),
            Box::new(ChorusEffect::new(1.5, 0.005, 0.5)),
            Box::new(FlangerEffect::new(0.5, 0.005, 0.5, 0.5)),
            Box::new(TremoloEffect::new(5.0, 0.7)),
            Box::new(VibratoEffect::new(5.0, 0.002)),
            Box::new(DistortionEffect::new(3.0, ClipMode::Soft)),
            Box::new(LoFiEffect::new(8.0, 2)),
            Box::new(RingModEffect::new(180.0)),
            Box::new(RobotModulationEffect::new(120.0)),
        ]
    }

    /// 16 kHz is below anything this app opens, but it is the rate a low-end
    /// endpoint can report and the one that reaches a zero-length delay line
    /// first. 48 kHz is what nearly every Windows endpoint actually runs at.
    const RATES: &[u32] = &[16_000, 22_050, 44_100, 48_000, 96_000];

    #[test]
    fn there_are_seventeen_effects() {
        assert_eq!(every_effect().len(), 17);
    }

    #[test]
    fn no_effect_panics_or_produces_nan_at_any_rate() {
        for rate in RATES {
            for mut fx in every_effect() {
                let mut s: Vec<f32> = (0..4_096).map(|i| (i as f32 * 0.05).sin() * 0.5).collect();
                fx.process(&mut s, *rate);
                assert!(
                    s.iter().all(|v| v.is_finite()),
                    "{} produced a non-finite sample at {rate} Hz",
                    fx.name()
                );
                assert_eq!(s.len(), 4_096, "{} changed the buffer length", fx.name());
            }
        }
    }

    #[test]
    fn no_effect_invents_signal_from_silence() {
        // A stage that turns silence into noise is audible as a permanent hiss on
        // an open mic, and every profile that includes it inherits the fault.
        for rate in RATES {
            for mut fx in every_effect() {
                let mut s = vec![0.0f32; 4_096];
                fx.process(&mut s, *rate);
                let peak = s.iter().fold(0.0f32, |a, b| a.max(b.abs()));
                assert!(peak < 1e-3, "{} invented signal at {rate} Hz (peak {peak})", fx.name());
            }
        }
    }

    /// Degenerate parameters, all reachable from a user-authored profile.
    ///
    /// Each of these panicked or produced NaN before the guards went in. They are
    /// asserted here rather than in each effect's own module because the property
    /// is the same one in every case: **a bad number in a saved profile must not
    /// be able to take down the audio callback.**
    #[test]
    fn degenerate_parameters_are_guarded() {
        let mut fx: Vec<Box<dyn AudioEffect>> = vec![
            // q = 0 divided by zero and put NaN through the whole stream.
            Box::new(BandpassFilterEffect::new(1_000.0, 0.0)),
            // ratio = 0 divided by zero in the gain computation.
            Box::new(CompressorEffect::new(0.5, 0.0, 0.005, 0.1)),
            // 2^128 overflows f32 to inf; every sample then NaN.
            Box::new(LoFiEffect::new(128.0, 1)),
            // downsample_rate = 0 quantised every sample away.
            Box::new(LoFiEffect::new(8.0, 0)),
            // A zero-length delay buffer is a `% 0` panic.
            Box::new(EchoEffect::new(0.0, 0.4, 0.4)),
            // A delay longer than the buffer underflowed write_pos + len - delay.
            Box::new(ChorusEffect::new(1.5, 10.0, 0.5)),
        ];

        for fx in fx.iter_mut() {
            for rate in RATES {
                let mut s: Vec<f32> = (0..1_024).map(|i| (i as f32 * 0.05).sin() * 0.5).collect();
                fx.process(&mut s, *rate);
                assert!(
                    s.iter().all(|v| v.is_finite()),
                    "{} produced a non-finite sample at {rate} Hz with a degenerate parameter",
                    fx.name()
                );
            }
        }
    }

    #[test]
    fn reset_returns_every_effect_to_silence() {
        // Audio leaking across a reset is audible as the tail of the *previous*
        // session arriving after the engine restarts.
        for mut fx in every_effect() {
            let mut hot = vec![0.8f32; 2_048];
            fx.process(&mut hot, 48_000);
            fx.reset();
            let mut s = vec![0.0f32; 2_048];
            fx.process(&mut s, 48_000);
            let peak = s.iter().fold(0.0f32, |a, b| a.max(b.abs()));
            assert!(peak < 1e-2, "{} leaked audio across reset (peak {peak})", fx.name());
        }
    }
}
