use eframe::egui::{Color32, Context, Rounding, Style, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum ThemeChoice {
    #[default]
    Dark,
    Neon,
}

pub struct AppTheme {
    pub background:  Color32,
    pub panel:       Color32,
    pub accent:      Color32,
    pub accent_alt:  Color32,
    pub text:        Color32,
    pub slider_fill: Color32,
    pub border:      Color32,
}

impl AppTheme {
    pub fn dark() -> Self {
        Self {
            background:  Color32::from_rgb(0x1a, 0x1a, 0x1e),
            panel:       Color32::from_rgb(0x25, 0x25, 0x2b),
            accent:      Color32::from_rgb(0x5b, 0x9c, 0xf6),
            accent_alt:  Color32::from_rgb(0x8b, 0x6c, 0xf5),
            text:        Color32::from_rgb(0xe0, 0xe0, 0xe0),
            slider_fill: Color32::from_rgb(0x3a, 0x6b, 0xc8),
            border:      Color32::from_rgb(0x3a, 0x3a, 0x44),
        }
    }

    pub fn neon() -> Self {
        Self {
            background:  Color32::from_rgb(0x0d, 0x0d, 0x14),
            panel:       Color32::from_rgb(0x14, 0x14, 0x1f),
            accent:      Color32::from_rgb(0x00, 0xf5, 0xd4),
            accent_alt:  Color32::from_rgb(0xf7, 0x00, 0xff),
            text:        Color32::WHITE,
            slider_fill: Color32::from_rgb(0x7b, 0x00, 0xff),
            border:      Color32::from_rgb(0x00, 0xf5, 0xd4),
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

        v.panel_fill             = t.panel;
        v.window_fill            = t.background;
        v.extreme_bg_color       = t.background;
        v.faint_bg_color         = t.panel;
        v.widgets.noninteractive.bg_fill = t.panel;
        v.widgets.noninteractive.fg_stroke.color = t.border;
        v.widgets.inactive.bg_fill = t.panel;
        v.widgets.inactive.fg_stroke.color = t.border;
        v.widgets.hovered.bg_fill = t.accent.linear_multiply(0.25);
        v.widgets.hovered.fg_stroke.color = t.accent;
        v.widgets.active.bg_fill  = t.accent.linear_multiply(0.45);
        v.widgets.active.fg_stroke.color = t.accent;
        v.selection.bg_fill       = t.accent.linear_multiply(0.35);
        v.selection.stroke.color  = t.accent;
        v.override_text_color     = Some(t.text);
        v.window_rounding         = Rounding::same(6.0);

        style.visuals = v;
        ctx.set_style(style);
    }
}
