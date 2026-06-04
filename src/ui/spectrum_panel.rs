use std::f32::consts::PI;
use std::sync::Arc;
use eframe::egui::{self, Color32, Pos2, Rect, Sense, Ui, Vec2};
use rustfft::{Fft, FftPlanner, num_complex::Complex};

use crate::audio::spectrum::{SpectrumBuffer, FFT_FRAME_SIZE};

const N_BINS:       usize = 64;
const DECAY:        f32   = 0.80;   // bar fall-off factor per frame
const DB_FLOOR:     f32   = -70.0;  // dB below which a bin is treated as silence
const DB_CEIL:      f32   = -10.0;  // dB at which a bin hits full height
const MIN_FREQ:     f32   = 60.0;   // Hz — lowest displayed bin
const MAX_FREQ:     f32   = 18_000.0;

pub struct SpectrumPanel {
    fft:     Arc<dyn Fft<f32>>,
    bars:    Vec<f32>,     // current smoothed bar heights, 0.0–1.0
    scratch: Vec<Complex<f32>>,
}

impl SpectrumPanel {
    pub fn new() -> Self {
        let mut planner = FftPlanner::new();
        let fft         = planner.plan_fft_forward(FFT_FRAME_SIZE);
        let scratch_len = fft.get_outofplace_scratch_len().max(FFT_FRAME_SIZE);
        Self {
            fft,
            bars:    vec![0.0; N_BINS],
            scratch: vec![Complex::new(0.0, 0.0); scratch_len],
        }
    }

    /// Called every frame. Reads from the spectrum buffer, runs FFT, updates bars.
    pub fn update(&mut self, spectrum: &SpectrumBuffer, sample_rate: u32) {
        // Decay bars every frame regardless of new data
        for b in &mut self.bars {
            *b *= DECAY;
        }

        let Some(samples) = spectrum.take_frame() else { return };

        // Apply Hann window
        let mut buf: Vec<Complex<f32>> = samples
            .iter()
            .enumerate()
            .map(|(i, &s)| {
                let w = 0.5 * (1.0 - (2.0 * PI * i as f32 / (FFT_FRAME_SIZE - 1) as f32).cos());
                Complex::new(s * w, 0.0)
            })
            .collect();

        self.fft.process_with_scratch(&mut buf, &mut self.scratch);

        // Magnitude spectrum (positive frequencies only)
        let mags: Vec<f32> = buf[..FFT_FRAME_SIZE / 2]
            .iter()
            .map(|c| c.norm() / FFT_FRAME_SIZE as f32)
            .collect();

        let new_bars = log_bins(&mags, sample_rate);

        // Attack-instant, decay-smooth
        for (bar, new) in self.bars.iter_mut().zip(new_bars.iter()) {
            if *new > *bar {
                *bar = *new;
            }
        }
    }

    /// Draws the spectrum into the remaining space of `ui`.
    pub fn show(&self, ui: &mut Ui, accent: Color32, active: bool) {
        let desired = Vec2::new(ui.available_width(), ui.available_height());
        let (resp, painter) = ui.allocate_painter(desired, Sense::hover());
        let rect = resp.rect;

        // Dark background
        painter.rect_filled(rect, 0.0, Color32::from_rgb(0x0e, 0x0e, 0x14));

        if !active {
            // Flat line when engine is stopped
            let y = rect.bottom() - 1.0;
            painter.line_segment(
                [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
                egui::Stroke::new(1.0, Color32::from_rgb(0x33, 0x33, 0x44)),
            );
            return;
        }

        let bar_width = rect.width() / N_BINS as f32;

        for (i, &height_norm) in self.bars.iter().enumerate() {
            if height_norm < 0.001 { continue; }

            let bar_h = height_norm * rect.height();
            let x     = rect.left() + i as f32 * bar_width;

            let bar_rect = Rect::from_min_size(
                Pos2::new(x + 0.5, rect.bottom() - bar_h),
                Vec2::new((bar_width - 1.0).max(1.0), bar_h),
            );

            // Gradient: dim at base, full accent near top
            let t = height_norm.powf(0.6); // gamma-correct for perceptual linearity
            let r = lerp_u8(accent.r() / 2, accent.r(), t);
            let g = lerp_u8(accent.g() / 2, accent.g(), t);
            let b = lerp_u8(accent.b() / 2, accent.b(), t);
            let a = lerp_u8(80,  220, t);

            painter.rect_filled(bar_rect, 1.5, Color32::from_rgba_unmultiplied(r, g, b, a));
        }
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

/// Maps raw FFT magnitudes to N_BINS log-spaced frequency bins.
fn log_bins(mags: &[f32], sample_rate: u32) -> Vec<f32> {
    let nyquist       = sample_rate as f32 / 2.0;
    let freq_per_bin  = nyquist / mags.len() as f32;
    let max_freq      = MAX_FREQ.min(nyquist * 0.95);
    let log_min       = MIN_FREQ.log10();
    let log_max       = max_freq.log10();

    (0..N_BINS)
        .map(|i| {
            let t0 = i as f32 / N_BINS as f32;
            let t1 = (i + 1) as f32 / N_BINS as f32;
            let f_lo = 10.0f32.powf(log_min + t0 * (log_max - log_min));
            let f_hi = 10.0f32.powf(log_min + t1 * (log_max - log_min));

            let bin_lo = ((f_lo / freq_per_bin) as usize).min(mags.len() - 1);
            let bin_hi = ((f_hi / freq_per_bin) as usize).min(mags.len() - 1).max(bin_lo);

            let peak = mags[bin_lo..=bin_hi].iter().cloned().fold(0.0f32, f32::max);

            // Convert to dB, normalise to 0–1
            let db = if peak > 1e-9 { 20.0 * peak.log10() } else { DB_FLOOR };
            ((db - DB_FLOOR) / (DB_CEIL - DB_FLOOR)).clamp(0.0, 1.0)
        })
        .collect()
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).clamp(0.0, 255.0) as u8
}
