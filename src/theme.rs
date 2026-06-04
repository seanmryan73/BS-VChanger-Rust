use eframe::egui::{Color32, Context, Rounding, Stroke, Style, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum ThemeChoice {
    #[default]
    Dark,
    Neon,
}

pub struct AppTheme {
    pub background:   Color32,
    pub panel:        Color32,
    pub accent:       Color32,
    pub accent_alt:   Color32,  // secondary accent (spectrum peaks, highlights)
    pub text:         Color32,
    pub text_muted:   Color32,  // group labels, hints
    pub slider_track: Color32,  // unselected slider rail
    pub border:       Color32,
}

impl AppTheme {
    /// Professional dark theme — deep navy-grey, cool blue accent.
    pub fn dark() -> Self {
        Self {
            background:   Color32::from_rgb(0x12, 0x12, 0x18),
            panel:        Color32::from_rgb(0x1c, 0x1c, 0x26),
            accent:       Color32::from_rgb(0x5b, 0x9c, 0xf6),
            accent_alt:   Color32::from_rgb(0x8b, 0x6c, 0xf5),
            text:         Color32::from_rgb(0xe4, 0xe4, 0xf0),
            text_muted:   Color32::from_rgb(0x60, 0x60, 0x78),
            slider_track: Color32::from_rgb(0x28, 0x28, 0x3c),
            border:       Color32::from_rgb(0x2c, 0x2c, 0x3e),
        }
    }

    /// Neon theme — near-black, vivid cyan accent with magenta highs.
    pub fn neon() -> Self {
        Self {
            background:   Color32::from_rgb(0x09, 0x09, 0x0f),
            panel:        Color32::from_rgb(0x10, 0x10, 0x1a),
            accent:       Color32::from_rgb(0x00, 0xe5, 0xc8),
            accent_alt:   Color32::from_rgb(0xd4, 0x00, 0xf5),
            text:         Color32::WHITE,
            text_muted:   Color32::from_rgb(0x44, 0x88, 0x80),
            slider_track: Color32::from_rgb(0x14, 0x14, 0x28),
            border:       Color32::from_rgb(0x00, 0x55, 0x4c),
        }
    }
}

pub struct ThemeManager {
    pub choice: ThemeChoice,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self { choice: ThemeChoice::Dark }
    }

    pub fn current(&self) -> AppTheme {
        match self.choice {
            ThemeChoice::Dark => AppTheme::dark(),
            ThemeChoice::Neon => AppTheme::neon(),
        }
    }

    pub fn toggle(&mut self) {
        self.choice = match self.choice {
            ThemeChoice::Dark => ThemeChoice::Neon,
            ThemeChoice::Neon => ThemeChoice::Dark,
        };
    }

    pub fn apply(&self, ctx: &Context) {
        let t = self.current();
        let mut style = Style::default();
        let mut v = Visuals::dark();

        // ── Backgrounds ───────────────────────────────────────────────────────
        v.panel_fill           = t.panel;
        v.window_fill          = t.background;
        v.extreme_bg_color     = t.background;
        v.faint_bg_color       = t.slider_track;

        // ── Non-interactive (labels, separators) ─────────────────────────────
        v.widgets.noninteractive.bg_fill          = t.panel;
        v.widgets.noninteractive.fg_stroke        = Stroke::new(1.0, t.border);
        v.widgets.noninteractive.rounding         = Rounding::same(4.0);
        v.widgets.noninteractive.bg_stroke        = Stroke::new(1.0, t.border);

        // ── Inactive (default button / slider track) ──────────────────────────
        v.widgets.inactive.bg_fill     = t.slider_track;
        v.widgets.inactive.fg_stroke   = Stroke::new(1.0, t.border);
        v.widgets.inactive.rounding    = Rounding::same(4.0);
        v.widgets.inactive.bg_stroke   = Stroke::new(1.0, t.border);
        v.widgets.inactive.expansion   = 0.0;

        // ── Hovered ───────────────────────────────────────────────────────────
        v.widgets.hovered.bg_fill      = t.accent.linear_multiply(0.18);
        v.widgets.hovered.fg_stroke    = Stroke::new(1.5, t.accent);
        v.widgets.hovered.rounding     = Rounding::same(4.0);
        v.widgets.hovered.bg_stroke    = Stroke::new(1.0, t.accent.linear_multiply(0.5));
        v.widgets.hovered.expansion    = 1.0;

        // ── Active (pressed / dragging) ───────────────────────────────────────
        v.widgets.active.bg_fill       = t.accent.linear_multiply(0.32);
        v.widgets.active.fg_stroke     = Stroke::new(2.0, t.accent);
        v.widgets.active.rounding      = Rounding::same(4.0);
        v.widgets.active.bg_stroke     = Stroke::new(1.5, t.accent);
        v.widgets.active.expansion     = 1.0;

        // ── Selection (slider fill, text selection) ───────────────────────────
        v.selection.bg_fill            = t.accent.linear_multiply(0.85);
        v.selection.stroke             = Stroke::new(1.0, t.accent);

        // ── Text ──────────────────────────────────────────────────────────────
        v.override_text_color          = Some(t.text);

        // ── Window chrome ─────────────────────────────────────────────────────
        v.window_rounding              = Rounding::same(6.0);
        v.window_shadow               = eframe::egui::epaint::Shadow::NONE;

        style.visuals = v;

        // ── Spacing ───────────────────────────────────────────────────────────
        style.spacing.item_spacing     = eframe::egui::vec2(6.0, 4.0);
        style.spacing.button_padding   = eframe::egui::vec2(10.0, 5.0);
        style.spacing.slider_width     = 140.0;
        style.spacing.interact_size    = eframe::egui::vec2(18.0, 18.0);

        ctx.set_style(style);
    }
}
