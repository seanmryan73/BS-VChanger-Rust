use std::f32::consts::PI;
use std::sync::Arc;
use eframe::egui::{self, Color32, Pos2, Rect, Sense, Ui, Vec2};
use rustfft::{Fft, FftPlanner, num_complex::Complex};

use crate::audio::spectrum::{SpectrumBuffer, FFT_FRAME_SIZE};

const N_BINS:       usize = 96;
const DECAY:        f32   = 0.93;   // slower hang — bars linger after the voice cuts
const PEAK_HOLD:    u32   = 45;     // frames (~0.75s at 60fps) before peak dot starts falling
const PEAK_DECAY:   f32   = 0.97;   // peak dot drift speed after hold expires
const DB_FLOOR:     f32   = -70.0;
const DB_CEIL:      f32   = -10.0;
const MIN_FREQ:     f32   = 60.0;
const MAX_FREQ:     f32   = 18_000.0;
const GLOW_LAYERS:  usize = 3;

pub struct SpectrumPanel {
    fft:         Arc<dyn Fft<f32>>,
    bars:        Vec<f32>,
    peaks:       Vec<f32>,
    peak_timers: Vec<u32>,
    scratch:     Vec<Complex<f32>>,
}

impl SpectrumPanel {
    pub fn new() -> Self {
        let mut planner = FftPlanner::new();
        let fft         = planner.plan_fft_forward(FFT_FRAME_SIZE);
        let scratch_len = fft.get_outofplace_scratch_len().max(FFT_FRAME_SIZE);
        Self {
            fft,
            bars:        vec![0.0; N_BINS],
            peaks:       vec![0.0; N_BINS],
            peak_timers: vec![0;   N_BINS],
            scratch:     vec![Complex::new(0.0, 0.0); scratch_len],
        }
    }

    pub fn update(&mut self, spectrum: &SpectrumBuffer, sample_rate: u32) {
        for b in &mut self.bars {
            *b *= DECAY;
        }
        for i in 0..N_BINS {
            if self.peak_timers[i] > 0 {
                self.peak_timers[i] -= 1;
            } else {
                self.peaks[i] *= PEAK_DECAY;
            }
        }

        let Some(samples) = spectrum.take_frame() else { return };

        let mut buf: Vec<Complex<f32>> = samples
            .iter()
            .enumerate()
            .map(|(i, &s)| {
                let w = 0.5 * (1.0 - (2.0 * PI * i as f32 / (FFT_FRAME_SIZE - 1) as f32).cos());
                Complex::new(s * w, 0.0)
            })
            .collect();

        self.fft.process_with_scratch(&mut buf, &mut self.scratch);

        let mags: Vec<f32> = buf[..FFT_FRAME_SIZE / 2]
            .iter()
            .map(|c| c.norm() / FFT_FRAME_SIZE as f32)
            .collect();

        let new_bars = log_bins(&mags, sample_rate);

        for (i, (bar, &new)) in self.bars.iter_mut().zip(new_bars.iter()).enumerate() {
            if new > *bar {
                *bar = new;
            }
            if *bar > self.peaks[i] {
                self.peaks[i]       = *bar;
                self.peak_timers[i] = PEAK_HOLD;
            }
        }
    }

    pub fn show(&self, ui: &mut Ui, accent: Color32, active: bool) {
        let desired = Vec2::new(ui.available_width(), ui.available_height());
        let (resp, painter) = ui.allocate_painter(desired, Sense::hover());
        let rect = resp.rect;

        painter.rect_filled(rect, 0.0, Color32::from_rgb(0x05, 0x05, 0x0c));

        // Split: top 68% = bar zone, bottom 32% = reflection zone
        let split_y      = rect.top() + rect.height() * 0.68;
        let bar_h_range  = split_y - rect.top();
        let refl_h_range = rect.bottom() - split_y;

        // Faint horizontal grid lines in bar zone
        for frac in [0.25f32, 0.50, 0.75] {
            let y = split_y - frac * bar_h_range;
            painter.line_segment(
                [Pos2::new(rect.left(), y), Pos2::new(rect.right(), y)],
                egui::Stroke::new(0.5, Color32::from_rgba_unmultiplied(130, 130, 190, 12)),
            );
        }

        // Frequency labels always visible (even when stopped)
        draw_freq_labels(&painter, rect, split_y);

        if !active {
            painter.line_segment(
                [Pos2::new(rect.left(), split_y), Pos2::new(rect.right(), split_y)],
                egui::Stroke::new(1.0, Color32::from_rgb(0x1a, 0x1a, 0x28)),
            );
            return;
        }

        let bar_width = rect.width() / N_BINS as f32;
        let inner_w   = (bar_width - 1.5).max(1.0);

        // Draw bars + reflections
        for (i, &height_norm) in self.bars.iter().enumerate() {
            if height_norm < 0.001 { continue; }

            let bar_h   = height_norm * bar_h_range;
            let x       = rect.left() + i as f32 * bar_width;
            let bar_top = split_y - bar_h;
            let color   = vu_color(height_norm, accent);

            // Glow layers
            for g in (0..GLOW_LAYERS).rev() {
                let spread = (g + 1) as f32 * 2.0;
                let alpha  = ((color.a() as f32) * 0.18 / (g + 1) as f32) as u8;
                let gc     = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);
                painter.rect_filled(
                    Rect::from_min_size(
                        Pos2::new(x + 0.5 - spread, bar_top - spread * 0.5),
                        Vec2::new(inner_w + spread * 2.0, bar_h + spread * 0.5),
                    ),
                    3.0, gc,
                );
            }

            // Main bar body
            painter.rect_filled(
                Rect::from_min_size(Pos2::new(x + 0.5, bar_top), Vec2::new(inner_w, bar_h)),
                2.0, color,
            );

            // Bright cap
            let cap_h = (bar_h * 0.04).clamp(2.0, 4.0);
            painter.rect_filled(
                Rect::from_min_size(Pos2::new(x + 0.5, bar_top), Vec2::new(inner_w, cap_h)),
                1.0, lighten(color, 0.75),
            );

            // Reflection — two-tier gradient: top half ~28%, bottom half ~10%
            let rh = (height_norm * refl_h_range * 0.82).min(refl_h_range);
            if rh > 0.5 {
                let half = rh * 0.5;
                painter.rect_filled(
                    Rect::from_min_size(Pos2::new(x + 0.5, split_y), Vec2::new(inner_w, half)),
                    1.0,
                    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), (color.a() as f32 * 0.28) as u8),
                );
                painter.rect_filled(
                    Rect::from_min_size(Pos2::new(x + 0.5, split_y + half), Vec2::new(inner_w, rh - half)),
                    1.0,
                    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), (color.a() as f32 * 0.09) as u8),
                );
            }
        }

        // Peak-hold dots
        for (i, &peak) in self.peaks.iter().enumerate() {
            if peak < 0.01 { continue; }
            let x      = rect.left() + i as f32 * bar_width;
            let peak_y = split_y - peak * bar_h_range;
            painter.rect_filled(
                Rect::from_min_size(Pos2::new(x + 0.5, peak_y - 2.0), Vec2::new(inner_w, 2.0)),
                0.0, lighten(vu_color(peak, accent), 0.9),
            );
        }

        // Floor separator line (accent-tinted)
        painter.line_segment(
            [Pos2::new(rect.left(), split_y), Pos2::new(rect.right(), split_y)],
            egui::Stroke::new(1.0, Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 50)),
        );
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn log_bins(mags: &[f32], sample_rate: u32) -> Vec<f32> {
    let nyquist      = sample_rate as f32 / 2.0;
    let freq_per_bin = nyquist / mags.len() as f32;
    let max_freq     = MAX_FREQ.min(nyquist * 0.95);
    let log_min      = MIN_FREQ.log10();
    let log_max      = max_freq.log10();

    (0..N_BINS)
        .map(|i| {
            let t0     = i as f32 / N_BINS as f32;
            let t1     = (i + 1) as f32 / N_BINS as f32;
            let f_lo   = 10.0f32.powf(log_min + t0 * (log_max - log_min));
            let f_hi   = 10.0f32.powf(log_min + t1 * (log_max - log_min));
            let bin_lo = ((f_lo / freq_per_bin) as usize).min(mags.len() - 1);
            let bin_hi = ((f_hi / freq_per_bin) as usize).min(mags.len() - 1).max(bin_lo);
            let peak   = mags[bin_lo..=bin_hi].iter().cloned().fold(0.0f32, f32::max);
            let db     = if peak > 1e-9 { 20.0 * peak.log10() } else { DB_FLOOR };
            ((db - DB_FLOOR) / (DB_CEIL - DB_FLOOR)).clamp(0.0, 1.0)
        })
        .collect()
}

fn vu_color(norm: f32, accent: Color32) -> Color32 {
    let yellow = Color32::from_rgb(0xe8, 0xb8, 0x00);
    let red    = Color32::from_rgb(0xe8, 0x30, 0x30);

    let base = if norm < 0.65 {
        let dim = Color32::from_rgba_unmultiplied(
            accent.r() / 3,
            accent.g() / 3,
            accent.b() / 3,
            100,
        );
        lerp_color(dim, accent, norm / 0.65)
    } else if norm < 0.82 {
        lerp_color(accent, yellow, (norm - 0.65) / 0.17)
    } else {
        lerp_color(yellow, red, (norm - 0.82) / 0.18)
    };

    let alpha = lerp_u8(70, 255, norm);
    Color32::from_rgba_unmultiplied(base.r(), base.g(), base.b(), alpha)
}

fn lighten(c: Color32, t: f32) -> Color32 {
    Color32::from_rgba_unmultiplied(
        lerp_u8(c.r(), 255, t),
        lerp_u8(c.g(), 255, t),
        lerp_u8(c.b(), 255, t),
        255,
    )
}

fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    Color32::from_rgb(
        lerp_u8(a.r(), b.r(), t),
        lerp_u8(a.g(), b.g(), t),
        lerp_u8(a.b(), b.b(), t),
    )
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).clamp(0.0, 255.0) as u8
}

fn draw_freq_labels(painter: &egui::Painter, rect: Rect, floor_y: f32) {
    const LABELS: &[(f32, &str)] = &[
        (60.0,     "60"),
        (200.0,    "200"),
        (500.0,    "500"),
        (1_000.0,  "1k"),
        (3_000.0,  "3k"),
        (8_000.0,  "8k"),
        (18_000.0, "18k"),
    ];
    let log_min = MIN_FREQ.log10();
    let log_max = MAX_FREQ.log10();
    let label_col = Color32::from_rgba_unmultiplied(155, 155, 195, 85);
    let tick_col  = Color32::from_rgba_unmultiplied(130, 130, 175, 45);

    for &(freq, label) in LABELS {
        let t = (freq.log10() - log_min) / (log_max - log_min);
        let x = rect.left() + t * rect.width();
        painter.line_segment(
            [Pos2::new(x, floor_y - 5.0), Pos2::new(x, floor_y)],
            egui::Stroke::new(0.5, tick_col),
        );
        painter.text(
            Pos2::new(x, floor_y - 6.0),
            egui::Align2::CENTER_BOTTOM,
            label,
            egui::FontId::monospace(8.5),
            label_col,
        );
    }
}
