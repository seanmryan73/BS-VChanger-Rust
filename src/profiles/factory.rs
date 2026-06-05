use std::collections::HashMap;
use crate::audio::effects::{
    AudioEffect, EffectChain,
    bandpass::BandpassFilterEffect,
    chorus::ChorusEffect,
    clean_mic::CleanMicEffect,
    compressor::CompressorEffect,
    de_esser::DeEsserEffect,
    distortion::{ClipMode, DistortionEffect},
    echo::EchoEffect,
    flanger::FlangerEffect,
    gain::GainEffect,
    lofi::LoFiEffect,
    noise_suppression::NoiseSuppressionEffect,
    pitch::PitchResampleEffect,
    reverb::ReverbEffect,
    ring_mod::RingModEffect,
    robot::RobotModulationEffect,
    tremolo::TremoloEffect,
    vibrato::VibratoEffect,
};
use super::{EffectConfig, EffectType};

/// Reads a param from the config, falling back to `default`.
fn p(cfg: &EffectConfig, key: &str, default: f64) -> f32 {
    cfg.params.get(key).copied().unwrap_or(default) as f32
}

/// Converts a single `EffectConfig` into a heap-allocated `AudioEffect`.
pub fn build_effect(cfg: &EffectConfig) -> Box<dyn AudioEffect> {
    match cfg.effect_type {
        EffectType::Gain =>
            Box::new(GainEffect::new(p(cfg, "gain", 1.0))),

        EffectType::CleanMic =>
            Box::new(CleanMicEffect::new(p(cfg, "cutoff_hz", 20.0))),

        EffectType::NoiseSuppression =>
            Box::new(NoiseSuppressionEffect::new(
                p(cfg, "strength",  1.0),
                p(cfg, "threshold", 0.0),
            )),

        EffectType::PitchShift =>
            Box::new(PitchResampleEffect::new(p(cfg, "semitones", 0.0))),

        EffectType::BandpassFilter =>
            Box::new(BandpassFilterEffect::new(
                p(cfg, "center_freq", 2_000.0),
                p(cfg, "q",           1.0),
            )),

        EffectType::Compressor =>
            Box::new(CompressorEffect::new(
                p(cfg, "threshold", 0.5),
                p(cfg, "ratio",     4.0),
                p(cfg, "attack",    0.005),
                p(cfg, "release",   0.1),
            )),

        EffectType::DeEsser =>
            Box::new(DeEsserEffect::new(
                p(cfg, "threshold", 0.3),
                p(cfg, "reduction", 0.3),
            )),

        EffectType::Echo =>
            Box::new(EchoEffect::new(
                p(cfg, "delay_secs", 0.3),
                p(cfg, "feedback",   0.3),
                p(cfg, "wet",        0.5),
            )),

        EffectType::Reverb =>
            Box::new(ReverbEffect::new(
                p(cfg, "room_size", 0.5),
                p(cfg, "wet",       0.3),
            )),

        EffectType::Chorus =>
            Box::new(ChorusEffect::new(
                p(cfg, "rate",  1.5),
                p(cfg, "depth", 0.003),
                p(cfg, "wet",   0.5),
            )),

        EffectType::Flanger =>
            Box::new(FlangerEffect::new(
                p(cfg, "rate",     0.5),
                p(cfg, "depth",    0.005),
                p(cfg, "feedback", 0.5),
                p(cfg, "wet",      0.5),
            )),

        EffectType::Tremolo =>
            Box::new(TremoloEffect::new(
                p(cfg, "rate",  5.0),
                p(cfg, "depth", 0.7),
            )),

        EffectType::Vibrato =>
            Box::new(VibratoEffect::new(
                p(cfg, "rate",  5.0),
                p(cfg, "depth", 0.003),
            )),

        EffectType::Distortion => {
            let mode = if p(cfg, "hard_clip", 0.0) > 0.5 { ClipMode::Hard } else { ClipMode::Soft };
            Box::new(DistortionEffect::new(p(cfg, "drive", 2.0), mode))
        }

        EffectType::LoFi =>
            Box::new(LoFiEffect::new(
                p(cfg, "bit_depth",    8.0),
                p(cfg, "downsample",   2.0) as u32,
            )),

        EffectType::RingMod =>
            Box::new(RingModEffect::new(p(cfg, "carrier_freq", 200.0))),

        EffectType::Robot =>
            Box::new(RobotModulationEffect::new(p(cfg, "pitch_hz", 100.0))),
    }
}

/// Builds a complete `EffectChain` from a slice of `EffectConfig`.
/// All effects (enabled and disabled) are added; the chain respects the enabled flag.
pub fn build_chain(effects: &[EffectConfig]) -> EffectChain {
    let mut chain = EffectChain::new();
    for cfg in effects {
        chain.add(build_effect(cfg), cfg.enabled);
    }
    chain
}

// ── Helpers for building EffectConfig values in built_in.rs ──────────────────

pub fn fx(effect_type: EffectType, enabled: bool, params: &[(&str, f64)]) -> EffectConfig {
    EffectConfig {
        effect_type,
        enabled,
        params: params.iter().map(|&(k, v)| (k.to_string(), v)).collect::<HashMap<_, _>>(),
    }
}
