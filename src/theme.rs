use eframe::egui::{Color32, Context, Rounding, Stroke, Style, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum ThemeChoice {
    CoralStorm,
    CandyPop,
    GlitchMode,
    ColdSteel,
    #[default]
    Lucky,
}

impl ThemeChoice {
    pub fn label(self) -> &'static str {
        match self {
            ThemeChoice::CoralStorm => "Coral Storm",
            ThemeChoice::CandyPop   => "Candy Pop",
            ThemeChoice::GlitchMode => "Glitch Mode",
            ThemeChoice::ColdSteel  => "Cold Steel",
            ThemeChoice::Lucky      => "Lucky",
        }
    }

    pub const ALL: &'static [ThemeChoice] = &[
        ThemeChoice::CoralStorm,
        ThemeChoice::CandyPop,
        ThemeChoice::GlitchMode,
        ThemeChoice::ColdSteel,
        ThemeChoice::Lucky,
    ];
}

pub struct AppTheme {
    pub background:    Color32,
    pub panel:         Color32,
    pub accent:        Color32,
    pub accent_alt:    Color32,
    pub text:          Color32,
    pub text_muted:    Color32,
    pub slider_track:  Color32,
    pub selection_bg:  Color32,
    pub border:        Color32,
    pub widget_border: Color32,
}

impl AppTheme {
    pub fn coral_storm() -> Self {
        Self {
            background:    Color32::from_rgb(0x00, 0x12, 0x12),
            panel:         Color32::from_rgb(0x00, 0x1e, 0x1e),
            accent:        Color32::from_rgb(0xff, 0x55, 0x33),
            accent_alt:    Color32::from_rgb(0x00, 0xff, 0xdd),
            text:          Color32::from_rgb(0xff, 0xf4, 0xee),
            text_muted:    Color32::from_rgb(0x22, 0x99, 0x88),
            slider_track:  Color32::from_rgb(0x00, 0x2c, 0x2c),
            selection_bg:  Color32::from_rgb(0x88, 0x22, 0x00),
            border:        Color32::from_rgb(0x00, 0x38, 0x38),
            widget_border: Color32::from_rgb(0x22, 0x99, 0x88),
        }
    }

    pub fn candy_pop() -> Self {
        Self {
            background:    Color32::from_rgb(0x10, 0x00, 0x08),
            panel:         Color32::from_rgb(0x1e, 0x00, 0x12),
            accent:        Color32::from_rgb(0xff, 0x00, 0x88),
            accent_alt:    Color32::from_rgb(0x00, 0xff, 0xaa),
            text:          Color32::from_rgb(0xff, 0xdd, 0xee),
            text_muted:    Color32::from_rgb(0xaa, 0x00, 0x55),
            slider_track:  Color32::from_rgb(0x28, 0x00, 0x18),
            selection_bg:  Color32::from_rgb(0x77, 0x00, 0x33),
            border:        Color32::from_rgb(0x30, 0x00, 0x1e),
            widget_border: Color32::from_rgb(0xaa, 0x00, 0x55),
        }
    }

    pub fn glitch_mode() -> Self {
        Self {
            background:    Color32::from_rgb(0x00, 0x03, 0x00),
            panel:         Color32::from_rgb(0x00, 0x08, 0x02),
            accent:        Color32::from_rgb(0x00, 0xff, 0x66),
            accent_alt:    Color32::from_rgb(0xff, 0x00, 0xcc),
            text:          Color32::from_rgb(0xcc, 0xff, 0xdd),
            text_muted:    Color32::from_rgb(0x00, 0x77, 0x33),
            slider_track:  Color32::from_rgb(0x00, 0x10, 0x00),
            selection_bg:  Color32::from_rgb(0x00, 0x44, 0x22),
            border:        Color32::from_rgb(0x00, 0x1c, 0x06),
            widget_border: Color32::from_rgb(0x00, 0x77, 0x33),
        }
    }

    pub fn cold_steel() -> Self {
        Self {
            background:    Color32::from_rgb(0x08, 0x08, 0x08),
            panel:         Color32::from_rgb(0x10, 0x10, 0x10),
            accent:        Color32::from_rgb(0x00, 0x88, 0xff),
            accent_alt:    Color32::from_rgb(0x44, 0xbb, 0xff),
            text:          Color32::from_rgb(0xec, 0xec, 0xec),
            text_muted:    Color32::from_rgb(0x66, 0x66, 0x66),
            slider_track:  Color32::from_rgb(0x1a, 0x1a, 0x1a),
            selection_bg:  Color32::from_rgb(0x00, 0x44, 0xcc),
            border:        Color32::from_rgb(0x28, 0x28, 0x28),
            widget_border: Color32::from_rgb(0x48, 0x48, 0x48),
        }
    }

    pub fn lucky() -> Self {
        Self {
            background:    Color32::from_rgb(0x14, 0x00, 0x2d),
            panel:         Color32::from_rgb(0x2d, 0x08, 0x50),
            accent:        Color32::from_rgb(0xff, 0x14, 0xd2),
            accent_alt:    Color32::from_rgb(0xc3, 0xff, 0x28),
            text:          Color32::from_rgb(0xc3, 0xff, 0x28),
            text_muted:    Color32::from_rgb(0x00, 0xa0, 0x8c),
            slider_track:  Color32::from_rgb(0x23, 0x04, 0x41),
            selection_bg:  Color32::from_rgb(0xb4, 0x4b, 0x00),
            border:        Color32::from_rgb(0x3c, 0x14, 0x5a),
            widget_border: Color32::from_rgb(0x00, 0xa0, 0x8c),
        }
    }
}

pub struct ThemeManager {
    pub choice: ThemeChoice,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self { choice: ThemeChoice::Lucky }
    }

    pub fn current(&self) -> AppTheme {
        match self.choice {
            ThemeChoice::CoralStorm => AppTheme::coral_storm(),
            ThemeChoice::CandyPop   => AppTheme::candy_pop(),
            ThemeChoice::GlitchMode => AppTheme::glitch_mode(),
            ThemeChoice::ColdSteel  => AppTheme::cold_steel(),
            ThemeChoice::Lucky      => AppTheme::lucky(),
        }
    }

    pub fn apply(&self, ctx: &Context) {
        let t = self.current();
        let mut style = Style::default();
        let mut v = Visuals::dark();

        v.panel_fill       = t.panel;
        v.window_fill      = t.background;
        v.extreme_bg_color = t.background;
        v.faint_bg_color   = t.slider_track;

        v.widgets.noninteractive.bg_fill   = t.panel;
        v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, t.border);
        v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, t.border);
        v.widgets.noninteractive.rounding  = Rounding::same(4.0);

        // Inactive: slider track + unchecked checkbox border (widget_border for visibility)
        v.widgets.inactive.bg_fill   = t.slider_track;
        v.widgets.inactive.fg_stroke = Stroke::new(1.5, t.widget_border);
        v.widgets.inactive.bg_stroke = Stroke::new(1.5, t.widget_border);
        v.widgets.inactive.rounding  = Rounding::same(4.0);
        v.widgets.inactive.expansion = 0.0;

        v.widgets.hovered.bg_fill   = t.accent.linear_multiply(0.18);
        v.widgets.hovered.fg_stroke = Stroke::new(1.5, t.accent);
        v.widgets.hovered.bg_stroke = Stroke::new(1.0, t.accent.linear_multiply(0.5));
        v.widgets.hovered.rounding  = Rounding::same(4.0);
        v.widgets.hovered.expansion = 1.0;

        v.widgets.active.bg_fill   = t.accent.linear_multiply(0.32);
        v.widgets.active.fg_stroke = Stroke::new(2.0, t.accent);
        v.widgets.active.bg_stroke = Stroke::new(1.5, t.accent);
        v.widgets.active.rounding  = Rounding::same(4.0);
        v.widgets.active.expansion = 1.0;

        // Checked checkbox fill + slider fill — readable under white text
        v.selection.bg_fill = t.selection_bg;
        v.selection.stroke  = Stroke::new(2.0, Color32::WHITE);

        v.override_text_color = Some(t.text);
        v.window_rounding     = Rounding::same(6.0);
        v.window_shadow       = eframe::egui::epaint::Shadow::NONE;

        style.visuals = v;
        style.spacing.item_spacing   = eframe::egui::vec2(6.0, 4.0);
        style.spacing.button_padding = eframe::egui::vec2(10.0, 5.0);
        style.spacing.slider_width   = 140.0;
        style.spacing.interact_size  = eframe::egui::vec2(18.0, 18.0);

        ctx.set_style(style);
    }
}
