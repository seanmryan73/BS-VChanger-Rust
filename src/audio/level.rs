use std::sync::Arc;
use parking_lot::Mutex;

/// Smoothed RMS level of the raw microphone input (before effects).
/// Written by the audio thread with `try_lock`; read by the UI thread.
#[derive(Clone)]
pub struct LevelBuffer {
    rms: Arc<Mutex<f32>>,
}

impl LevelBuffer {
    pub fn new() -> Self {
        Self { rms: Arc::new(Mutex::new(0.0)) }
    }

    /// Audio thread: update smoothed RMS. Values > 1.0 indicate clipping.
    pub fn push(&self, samples: &[f32]) {
        if samples.is_empty() { return; }
        let Some(mut level) = self.rms.try_lock() else { return };
        let rms = (samples.iter().map(|s| s * s).sum::<f32>() / samples.len() as f32).sqrt();
        // Exponential moving average — fast attack, medium release
        *level = (*level * 0.80 + rms * 0.20).min(2.0);
    }

    /// UI thread: returns smoothed RMS (0.0 = silence, 1.0 = full scale, >1.0 = clipping).
    pub fn get(&self) -> f32 {
        *self.rms.lock()
    }

    pub fn reset(&self) {
        if let Some(mut l) = self.rms.try_lock() { *l = 0.0; }
    }
}
