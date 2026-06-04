use super::AudioEffect;

pub struct EffectChain {
    effects: Vec<(Box<dyn AudioEffect>, bool)>, // (effect, enabled)
}

impl EffectChain {
    pub fn new() -> Self {
        Self { effects: Vec::new() }
    }

    pub fn add(&mut self, effect: Box<dyn AudioEffect>, enabled: bool) {
        self.effects.push((effect, enabled));
    }

    pub fn process(&mut self, samples: &mut [f32], sample_rate: u32) {
        for (effect, enabled) in &mut self.effects {
            if *enabled {
                effect.process(samples, sample_rate);
            }
        }
    }

    pub fn reset(&mut self) {
        for (effect, _) in &mut self.effects {
            effect.reset();
        }
    }
}

impl Default for EffectChain {
    fn default() -> Self {
        Self::new()
    }
}
