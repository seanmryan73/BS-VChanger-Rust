use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectType {
    Gain,
    CleanMic,
    BandpassFilter,
    Compressor,
    DeEsser,
    NoiseSuppression,
    PitchShift,
    Echo,
    Reverb,
    Chorus,
    Flanger,
    Tremolo,
    Vibrato,
    Distortion,
    LoFi,
    RingMod,
    Robot,
}
