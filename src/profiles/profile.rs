use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::EffectType;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EffectConfig {
    pub effect_type: EffectType,
    pub enabled:     bool,
    pub params:      HashMap<String, f64>,
}

impl EffectConfig {
    pub fn new(effect_type: EffectType, params: HashMap<String, f64>) -> Self {
        Self { effect_type, enabled: true, params }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VoiceProfile {
    pub id:       Uuid,
    pub name:     String,
    pub effects:  Vec<EffectConfig>,
    pub built_in: bool,
}

impl VoiceProfile {
    pub fn new(name: impl Into<String>, effects: Vec<EffectConfig>) -> Self {
        Self { id: Uuid::new_v4(), name: name.into(), effects, built_in: false }
    }

    pub fn built_in(name: impl Into<String>, effects: Vec<EffectConfig>) -> Self {
        Self { id: Uuid::new_v4(), name: name.into(), effects, built_in: true }
    }
}
