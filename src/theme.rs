use eframe::egui::{Color32, Context, Rounding, Stroke, Style, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum ThemeChoice {
    CoralStorm,
    Shibui,
    Kasane,
    #[default]
    ColdSteel,
    Jizo,
    HotSteel,
}

impl ThemeChoice {
    pub fn label(self) -> &'static str {
        match self {
            ThemeChoice::CoralStorm => "Coral Storm",
            ThemeChoice::Shibui     => "Shibui",
            ThemeChoice::Kasane     => "Kasane",
            ThemeChoice::ColdSteel  => "Cold Steel",
            ThemeChoice::Jizo       => "Jizo",
            ThemeChoice::HotSteel   => "Hot Steel",
        }
    }

    pub const ALL: &'static [ThemeChoice] = &[
        ThemeChoice::CoralStorm,
        ThemeChoice::Shibui,
        ThemeChoice::Kasane,
        ThemeChoice::ColdSteel,
        ThemeChoice::Jizo,
        ThemeChoice::HotSteel,
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

    pub fn shibui() -> Self {
        Self {
            background:    Color32::from_rgb(0x14, 0x0a, 0x0c),
            panel:         Color32::from_rgb(0x24, 0x13, 0x18),
            accent:        Color32::from_rgb(0xdf, 0xa2, 0x3d),
            accent_alt:    Color32::from_rgb(0x7c, 0x93, 0xa8),
            text:          Color32::from_rgb(0xed, 0xe3, 0xd6),
            text_muted:    Color32::from_rgb(0x7a, 0x27, 0x32),
            slider_track:  Color32::from_rgb(0x1c, 0x0f, 0x12),
            selection_bg:  Color32::from_rgb(0x76, 0x7c, 0x33),
            border:        Color32::from_rgb(0x18, 0x0d, 0x10),
            widget_border: Color32::from_rgb(0x6b, 0x57, 0x70),
        }
    }

    pub fn kasane() -> Self {
        Self {
            background:    Color32::from_rgb(0x0d, 0x07, 0x14),
            panel:         Color32::from_rgb(0x1c, 0x0f, 0x2b),
            accent:        Color32::from_rgb(0xe1, 0x40, 0x1d),
            accent_alt:    Color32::from_rgb(0xa0, 0xcc, 0x78),
            text:          Color32::from_rgb(0xf7, 0xcf, 0xd4),
            text_muted:    Color32::from_rgb(0xa8, 0x7c, 0xc2),
            slider_track:  Color32::from_rgb(0x17, 0x0c, 0x22),
            selection_bg:  Color32::from_rgb(0xf0, 0x84, 0x80),
            border:        Color32::from_rgb(0x14, 0x0a, 0x1e),
            widget_border: Color32::from_rgb(0xe7, 0xb3, 0x3d),
        }
    }

    pub fn cold_steel() -> Self {
        Self {
            background:    Color32::from_rgb(0x0a, 0x0b, 0x0d),
            panel:         Color32::from_rgb(0x16, 0x17, 0x1b),
            accent:        Color32::from_rgb(0x2e, 0x8f, 0xff),
            accent_alt:    Color32::from_rgb(0x7f, 0x97, 0xb3),
            text:          Color32::from_rgb(0xe8, 0xec, 0xf1),
            text_muted:    Color32::from_rgb(0x7f, 0x8f, 0xa6),
            slider_track:  Color32::from_rgb(0x1c, 0x1e, 0x23),
            selection_bg:  Color32::from_rgb(0x2f, 0x6f, 0xd6),
            border:        Color32::from_rgb(0x23, 0x25, 0x29),
            widget_border: Color32::from_rgb(0x3f, 0x6f, 0x99),
        }
    }

    pub fn jizo() -> Self {
        Self {
            background:    Color32::from_rgb(0x12, 0x0f, 0x0c),
            panel:         Color32::from_rgb(0x1f, 0x19, 0x13),
            accent:        Color32::from_rgb(0xb5, 0x28, 0x3a),
            accent_alt:    Color32::from_rgb(0xb8, 0xc2, 0x3f),
            text:          Color32::from_rgb(0xe3, 0xcf, 0xd2),
            text_muted:    Color32::from_rgb(0x8f, 0x81, 0x75),
            slider_track:  Color32::from_rgb(0x1a, 0x14, 0x0f),
            selection_bg:  Color32::from_rgb(0x8f, 0x20, 0x30),
            border:        Color32::from_rgb(0x18, 0x13, 0x10),
            widget_border: Color32::from_rgb(0x6b, 0x5f, 0x52),
        }
    }

    pub fn hot_steel() -> Self {
        Self {
            background:    Color32::from_rgb(0x0d, 0x0a, 0x0b),
            panel:         Color32::from_rgb(0x1b, 0x15, 0x17),
            accent:        Color32::from_rgb(0xff, 0x3f, 0x8f),
            accent_alt:    Color32::from_rgb(0xb3, 0x83, 0x8f),
            text:          Color32::from_rgb(0xf1, 0xe8, 0xec),
            text_muted:    Color32::from_rgb(0xa6, 0x80, 0x8a),
            slider_track:  Color32::from_rgb(0x23, 0x1c, 0x1e),
            selection_bg:  Color32::from_rgb(0xd6, 0x3f, 0x7f),
            border:        Color32::from_rgb(0x2b, 0x23, 0x25),
            widget_border: Color32::from_rgb(0x8f, 0x3f, 0x5f),
        }
    }
}

pub struct ThemeManager {
    pub choice: ThemeChoice,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self { choice: ThemeChoice::ColdSteel }
    }

    pub fn current(&self) -> AppTheme {
        match self.choice {
            ThemeChoice::CoralStorm => AppTheme::coral_storm(),
            ThemeChoice::Shibui     => AppTheme::shibui(),
            ThemeChoice::Kasane     => AppTheme::kasane(),
            ThemeChoice::ColdSteel  => AppTheme::cold_steel(),
            ThemeChoice::Jizo       => AppTheme::jizo(),
            ThemeChoice::HotSteel   => AppTheme::hot_steel(),
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
        v.widgets.hovered.bg_stroke = Stroke::new(1.5, t.accent);   // match inactive width — prevents bounce
        v.widgets.hovered.rounding  = Rounding::same(4.0);
        v.widgets.hovered.expansion = 0.0;                          // NEVER > 0 — causes layout shift/bounce

        v.widgets.active.bg_fill   = t.accent.linear_multiply(0.32);
        v.widgets.active.fg_stroke = Stroke::new(1.5, t.accent);
        v.widgets.active.bg_stroke = Stroke::new(1.5, t.accent);
        v.widgets.active.rounding  = Rounding::same(4.0);
        v.widgets.active.expansion = 0.0;                           // NEVER > 0 — causes layout shift/bounce

        v.widgets.open.bg_fill   = t.accent.linear_multiply(0.24);  // ComboBox/dropdown while open
        v.widgets.open.fg_stroke = Stroke::new(1.5, t.accent);
        v.widgets.open.bg_stroke = Stroke::new(1.5, t.accent);      // match inactive width — prevents open-state bounce
        v.widgets.open.rounding  = Rounding::same(4.0);
        v.widgets.open.expansion = 0.0;

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
        // Solid, muted scrollbar (uses widget bg_fill, not raw accent) — avoids the
        // default floating style's invisible-then-neon-bright-on-hover jump.
        style.spacing.scroll = eframe::egui::style::ScrollStyle::solid();

        ctx.set_style(style);
    }
}
