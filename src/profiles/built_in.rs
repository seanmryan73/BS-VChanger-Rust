use super::{EffectType::*, VoiceProfile};
use super::factory::fx;

pub fn all() -> Vec<VoiceProfile> {
    vec![
        // ── Passthrough ───────────────────────────────────────────────────────
        VoiceProfile::built_in("Passthrough", vec![]),

        // ── Clean voice (primary use-case) ────────────────────────────────────
        VoiceProfile::built_in("Clean Voice", vec![
            fx(CleanMic,         true,  &[]),
            fx(NoiseSuppression, true,  &[]),
            fx(Compressor,       true,  &[("threshold",0.4),("ratio",3.0),("attack",0.005),("release",0.1)]),
            fx(Gain,             true,  &[("gain",1.0)]),
        ]),
        VoiceProfile::built_in("Noise Reduction", vec![
            fx(NoiseSuppression, true,  &[]),
            fx(Gain,             true,  &[("gain",1.0)]),
        ]),
        VoiceProfile::built_in("Studio Voice", vec![
            fx(CleanMic,         true,  &[]),
            fx(NoiseSuppression, true,  &[]),
            fx(DeEsser,          true,  &[("threshold",0.25),("reduction",0.4)]),
            fx(BandpassFilter,   true,  &[("center_freq",2200.0),("q",0.6)]),
            fx(Compressor,       true,  &[("threshold",0.35),("ratio",3.5),("attack",0.003),("release",0.08)]),
            fx(Gain,             true,  &[("gain",1.1)]),
        ]),
        VoiceProfile::built_in("Conference Call", vec![
            fx(CleanMic,         true,  &[]),
            fx(NoiseSuppression, true,  &[]),
            fx(BandpassFilter,   true,  &[("center_freq",1800.0),("q",0.5)]),
            fx(Compressor,       true,  &[("threshold",0.45),("ratio",4.0),("attack",0.005),("release",0.12)]),
            fx(Gain,             true,  &[("gain",1.0)]),
        ]),
        VoiceProfile::built_in("Podcast", vec![
            fx(CleanMic,         true,  &[]),
            fx(NoiseSuppression, true,  &[]),
            fx(DeEsser,          true,  &[("threshold",0.3),("reduction",0.35)]),
            fx(Compressor,       true,  &[("threshold",0.4),("ratio",3.0),("attack",0.005),("release",0.1)]),
            fx(Gain,             true,  &[("gain",1.15)]),
        ]),
        VoiceProfile::built_in("Clarity Boost", vec![
            fx(CleanMic,         true,  &[]),
            fx(NoiseSuppression, true,  &[]),
            fx(BandpassFilter,   true,  &[("center_freq",3000.0),("q",0.5)]),
            fx(Gain,             true,  &[("gain",1.05)]),
        ]),
        VoiceProfile::built_in("Broadcast", vec![
            fx(CleanMic,         true,  &[]),
            fx(NoiseSuppression, true,  &[]),
            fx(BandpassFilter,   true,  &[("center_freq",2000.0),("q",0.7)]),
            fx(Compressor,       true,  &[("threshold",0.3),("ratio",5.0),("attack",0.003),("release",0.08)]),
            fx(DeEsser,          true,  &[("threshold",0.2),("reduction",0.4)]),
            fx(Gain,             true,  &[("gain",1.2)]),
        ]),
        VoiceProfile::built_in("Gentle Compress", vec![
            fx(NoiseSuppression, true,  &[]),
            fx(Compressor,       true,  &[("threshold",0.6),("ratio",2.0),("attack",0.01),("release",0.15)]),
            fx(Gain,             true,  &[("gain",1.0)]),
        ]),

        // ── Pitch / voice character ───────────────────────────────────────────
        VoiceProfile::built_in("Deep Voice", vec![
            fx(CleanMic,         true,  &[]),
            fx(NoiseSuppression, true,  &[]),
            fx(PitchShift,       true,  &[("semitones",-4.0)]),
            fx(Compressor,       true,  &[("threshold",0.5),("ratio",3.0),("attack",0.005),("release",0.1)]),
        ]),
        VoiceProfile::built_in("High Voice", vec![
            fx(PitchShift,       true,  &[("semitones",4.0)]),
            fx(Compressor,       true,  &[("threshold",0.5),("ratio",2.5),("attack",0.005),("release",0.1)]),
        ]),
        VoiceProfile::built_in("Chipmunk", vec![
            fx(PitchShift,       true,  &[("semitones",12.0)]),
            fx(Gain,             true,  &[("gain",0.9)]),
        ]),
        VoiceProfile::built_in("Giant", vec![
            fx(PitchShift,       true,  &[("semitones",-8.0)]),
            fx(Reverb,           true,  &[("room_size",0.7),("wet",0.25)]),
            fx(Gain,             true,  &[("gain",1.1)]),
        ]),

        // ── Telephone / radio character ───────────────────────────────────────
        VoiceProfile::built_in("Telephone", vec![
            fx(BandpassFilter,   true,  &[("center_freq",1500.0),("q",3.0)]),
            fx(LoFi,             true,  &[("bit_depth",10.0),("downsample",1.0)]),
            fx(Compressor,       true,  &[("threshold",0.4),("ratio",5.0),("attack",0.003),("release",0.08)]),
        ]),
        VoiceProfile::built_in("Radio", vec![
            fx(BandpassFilter,   true,  &[("center_freq",2000.0),("q",2.0)]),
            fx(Distortion,       true,  &[("drive",1.5),("hard_clip",0.0)]),
            fx(Compressor,       true,  &[("threshold",0.35),("ratio",6.0),("attack",0.002),("release",0.06)]),
            fx(Gain,             true,  &[("gain",0.8)]),
        ]),
        VoiceProfile::built_in("Walkie-Talkie", vec![
            fx(BandpassFilter,   true,  &[("center_freq",1200.0),("q",4.0)]),
            fx(Distortion,       true,  &[("drive",2.0),("hard_clip",1.0)]),
            fx(LoFi,             true,  &[("bit_depth",8.0),("downsample",2.0)]),
            fx(Gain,             true,  &[("gain",0.7)]),
        ]),
        VoiceProfile::built_in("Megaphone", vec![
            fx(BandpassFilter,   true,  &[("center_freq",2500.0),("q",1.5)]),
            fx(Distortion,       true,  &[("drive",3.0),("hard_clip",0.0)]),
            fx(Compressor,       true,  &[("threshold",0.3),("ratio",8.0),("attack",0.001),("release",0.05)]),
        ]),

        // ── Room / space effects ──────────────────────────────────────────────
        VoiceProfile::built_in("Echo Chamber", vec![
            fx(CleanMic,         true,  &[]),
            fx(Echo,             true,  &[("delay_secs",0.3),("feedback",0.5),("wet",0.6)]),
            fx(Reverb,           true,  &[("room_size",0.6),("wet",0.3)]),
        ]),
        VoiceProfile::built_in("Cathedral", vec![
            fx(Reverb,           true,  &[("room_size",0.95),("wet",0.7)]),
            fx(Chorus,           true,  &[("rate",0.5),("depth",0.004),("wet",0.2)]),
        ]),
        VoiceProfile::built_in("Underwater", vec![
            fx(BandpassFilter,   true,  &[("center_freq",600.0),("q",0.5)]),
            fx(Chorus,           true,  &[("rate",0.8),("depth",0.005),("wet",0.6)]),
            fx(Reverb,           true,  &[("room_size",0.5),("wet",0.4)]),
        ]),

        // ── Creative / entertainment ──────────────────────────────────────────
        VoiceProfile::built_in("Robot", vec![
            fx(Robot,            true,  &[("pitch_hz",120.0)]),
            fx(Compressor,       true,  &[("threshold",0.5),("ratio",4.0),("attack",0.005),("release",0.1)]),
        ]),
        VoiceProfile::built_in("Alien", vec![
            fx(PitchShift,       true,  &[("semitones",7.0)]),
            fx(RingMod,          true,  &[("carrier_freq",180.0)]),
            fx(Reverb,           true,  &[("room_size",0.4),("wet",0.25)]),
        ]),
        VoiceProfile::built_in("Daemon", vec![
            fx(PitchShift,       true,  &[("semitones",-10.0)]),
            fx(RingMod,          true,  &[("carrier_freq",60.0)]),
            fx(Reverb,           true,  &[("room_size",0.8),("wet",0.4)]),
        ]),
        VoiceProfile::built_in("Vintage", vec![
            fx(LoFi,             true,  &[("bit_depth",8.0),("downsample",3.0)]),
            fx(Echo,             true,  &[("delay_secs",0.25),("feedback",0.35),("wet",0.4)]),
            fx(Gain,             true,  &[("gain",0.9)]),
        ]),
        VoiceProfile::built_in("Tremolo Voice", vec![
            fx(CleanMic,         true,  &[]),
            fx(NoiseSuppression, true,  &[]),
            fx(Tremolo,          true,  &[("rate",6.0),("depth",0.6)]),
            fx(Gain,             true,  &[("gain",1.0)]),
        ]),
    ]
}
