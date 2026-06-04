use std::collections::VecDeque;
use std::sync::Arc;
use parking_lot::Mutex;

pub const FFT_FRAME_SIZE: usize = 2048;

/// Lock-free-ish bridge between the audio thread and the UI thread.
///
/// The audio thread calls `push` with `try_lock` — it never blocks.
/// The UI thread calls `take_frame` with a regular lock to get the latest samples.
#[derive(Clone)]
pub struct SpectrumBuffer {
    inner: Arc<Mutex<VecDeque<f32>>>,
}

impl SpectrumBuffer {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(VecDeque::with_capacity(FFT_FRAME_SIZE * 2))),
        }
    }

    /// Audio thread: appends samples, keeping only the most recent FFT_FRAME_SIZE.
    /// Uses `try_lock` so the audio callback is never blocked.
    pub fn push(&self, samples: &[f32]) {
        let Some(mut buf) = self.inner.try_lock() else { return };
        for &s in samples {
            buf.push_back(s);
        }
        while buf.len() > FFT_FRAME_SIZE {
            buf.pop_front();
        }
    }

    /// UI thread: returns the latest FFT_FRAME_SIZE samples if enough have arrived.
    pub fn take_frame(&self) -> Option<Vec<f32>> {
        let buf = self.inner.lock();
        if buf.len() >= FFT_FRAME_SIZE {
            Some(buf.iter().copied().collect())
        } else {
            None
        }
    }

    pub fn clear(&self) {
        self.inner.lock().clear();
    }
}
