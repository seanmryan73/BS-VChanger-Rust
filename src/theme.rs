use eframe::egui::{Color32, Context, Rounding, Stroke, Style, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum ThemeChoice {
    #[default]
    NeonVoid,
    CyberRift,
    AcidRain,
    SolarFlare,
    BloodMoon,
    CandyPop,
    ToxicSlime,
    DeepSpace,
    Inferno,
    GlitchMode,
    ArcticNova,
    Vaporwave,
    Galactic,
    CoralStorm,
    Ultraviolet,
    NeonDusk,
    JackedIn,
    MoltenGlow,
}

impl ThemeChoice {
    pub fn label(self) -> &'static str {
        match self {
            ThemeChoice::NeonVoid    => "Neon Void",
            ThemeChoice::CyberRift   => "Cyber Rift",
            ThemeChoice::AcidRain    => "Acid Rain",
            ThemeChoice::SolarFlare  => "Solar Flare",
            ThemeChoice::BloodMoon   => "Blood Moon",
            ThemeChoice::CandyPop    => "Candy Pop",
            ThemeChoice::ToxicSlime  => "Toxic Slime",
            ThemeChoice::DeepSpace   => "Deep Space",
            ThemeChoice::Inferno     => "Inferno",
            ThemeChoice::GlitchMode  => "Glitch Mode",
            ThemeChoice::ArcticNova  => "Arctic Nova",
            ThemeChoice::Vaporwave   => "Vaporwave",
            ThemeChoice::Galactic    => "Galactic",
            ThemeChoice::CoralStorm  => "Coral Storm",
            ThemeChoice::Ultraviolet => "Ultraviolet",
            ThemeChoice::NeonDusk    => "Neon Dusk",
            ThemeChoice::JackedIn    => "Jacked In",
            ThemeChoice::MoltenGlow  => "Molten Glow",
        }
    }

    pub const ALL: &'static [ThemeChoice] = &[
        ThemeChoice::NeonVoid,
        ThemeChoice::CyberRift,
        ThemeChoice::AcidRain,
        ThemeChoice::SolarFlare,
        ThemeChoice::BloodMoon,
        ThemeChoice::CandyPop,
        ThemeChoice::ToxicSlime,
        ThemeChoice::DeepSpace,
        ThemeChoice::Inferno,
        ThemeChoice::GlitchMode,
        ThemeChoice::ArcticNova,
        ThemeChoice::Vaporwave,
        ThemeChoice::Galactic,
        ThemeChoice::CoralStorm,
        ThemeChoice::Ultraviolet,
        ThemeChoice::NeonDusk,
        ThemeChoice::JackedIn,
        ThemeChoice::MoltenGlow,
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
    pub fn neon_void() -> Self {
        Self {
            background:    Color32::from_rgb(0x00, 0x00, 0x00),
            panel:         Color32::from_rgb(0x08, 0x00, 0x10),
            accent:        Color32::from_rgb(0x00, 0xff, 0xff),
            accent_alt:    Color32::from_rgb(0xff, 0x00, 0xaa),
            text:          Color32::from_rgb(0xe0, 0xff, 0xff),
            text_muted:    Color32::from_rgb(0x00, 0x88, 0x88),
            slider_track:  Color32::from_rgb(0x0a, 0x00, 0x1a),
            selection_bg:  Color32::from_rgb(0x00, 0x44, 0x44),
            border:        Color32::from_rgb(0x00, 0x22, 0x22),
            widget_border: Color32::from_rgb(0x00, 0xcc, 0xcc),
        }
    }

    pub fn cyber_rift() -> Self {
        Self {
            background:    Color32::from_rgb(0x08, 0x00, 0x14),
            panel:         Color32::from_rgb(0x12, 0x00, 0x28),
            accent:        Color32::from_rgb(0xff, 0x00, 0xff),
            accent_alt:    Color32::from_rgb(0x00, 0xcc, 0xff),
            text:          Color32::from_rgb(0xf4, 0xd8, 0xff),
            text_muted:    Color32::from_rgb(0x88, 0x00, 0xcc),
            slider_track:  Color32::from_rgb(0x1a, 0x00, 0x34),
            selection_bg:  Color32::from_rgb(0x44, 0x00, 0x88),
            border:        Color32::from_rgb(0x22, 0x00, 0x44),
            widget_border: Color32::from_rgb(0xcc, 0x00, 0xff),
        }
    }

    pub fn acid_rain() -> Self {
        Self {
            background:    Color32::from_rgb(0x00, 0x0c, 0x06),
            panel:         Color32::from_rgb(0x00, 0x1a, 0x0c),
            accent:        Color32::from_rgb(0x00, 0xff, 0x44),
            accent_alt:    Color32::from_rgb(0x00, 0xff, 0xcc),
            text:          Color32::from_rgb(0xcc, 0xff, 0xe8),
            text_muted:    Color32::from_rgb(0x00, 0x88, 0x44),
            slider_track:  Color32::from_rgb(0x00, 0x24, 0x10),
            selection_bg:  Color32::from_rgb(0x00, 0x55, 0x22),
            border:        Color32::from_rgb(0x00, 0x30, 0x16),
            widget_border: Color32::from_rgb(0x00, 0xcc, 0x44),
        }
    }

    pub fn solar_flare() -> Self {
        Self {
            background:    Color32::from_rgb(0x10, 0x04, 0x00),
            panel:         Color32::from_rgb(0x20, 0x08, 0x00),
            accent:        Color32::from_rgb(0xff, 0x66, 0x00),
            accent_alt:    Color32::from_rgb(0xff, 0xee, 0x00),
            text:          Color32::from_rgb(0xff, 0xf4, 0xcc),
            text_muted:    Color32::from_rgb(0xaa, 0x44, 0x00),
            slider_track:  Color32::from_rgb(0x2c, 0x0e, 0x00),
            selection_bg:  Color32::from_rgb(0x88, 0x22, 0x00),
            border:        Color32::from_rgb(0x38, 0x14, 0x00),
            widget_border: Color32::from_rgb(0xff, 0x88, 0x00),
        }
    }

    pub fn blood_moon() -> Self {
        Self {
            background:    Color32::from_rgb(0x10, 0x00, 0x00),
            panel:         Color32::from_rgb(0x1e, 0x00, 0x00),
            accent:        Color32::from_rgb(0xff, 0x00, 0x33),
            accent_alt:    Color32::from_rgb(0xff, 0x44, 0x99),
            text:          Color32::from_rgb(0xff, 0xe0, 0xe0),
            text_muted:    Color32::from_rgb(0xaa, 0x00, 0x22),
            slider_track:  Color32::from_rgb(0x28, 0x00, 0x00),
            selection_bg:  Color32::from_rgb(0x77, 0x00, 0x11),
            border:        Color32::from_rgb(0x30, 0x00, 0x00),
            widget_border: Color32::from_rgb(0xdd, 0x00, 0x22),
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
            widget_border: Color32::from_rgb(0xff, 0x00, 0x88),
        }
    }

    pub fn toxic_slime() -> Self {
        Self {
            background:    Color32::from_rgb(0x03, 0x0a, 0x00),
            panel:         Color32::from_rgb(0x06, 0x16, 0x00),
            accent:        Color32::from_rgb(0xaa, 0xff, 0x00),
            accent_alt:    Color32::from_rgb(0x00, 0xff, 0x55),
            text:          Color32::from_rgb(0xee, 0xff, 0xcc),
            text_muted:    Color32::from_rgb(0x66, 0x99, 0x00),
            slider_track:  Color32::from_rgb(0x08, 0x1e, 0x00),
            selection_bg:  Color32::from_rgb(0x44, 0x66, 0x00),
            border:        Color32::from_rgb(0x0e, 0x28, 0x00),
            widget_border: Color32::from_rgb(0x88, 0xff, 0x00),
        }
    }

    pub fn deep_space() -> Self {
        Self {
            background:    Color32::from_rgb(0x02, 0x00, 0x12),
            panel:         Color32::from_rgb(0x06, 0x00, 0x26),
            accent:        Color32::from_rgb(0x77, 0x00, 0xff),
            accent_alt:    Color32::from_rgb(0x00, 0xcc, 0xff),
            text:          Color32::from_rgb(0xe0, 0xd8, 0xff),
            text_muted:    Color32::from_rgb(0x55, 0x00, 0xbb),
            slider_track:  Color32::from_rgb(0x0a, 0x00, 0x38),
            selection_bg:  Color32::from_rgb(0x33, 0x00, 0x88),
            border:        Color32::from_rgb(0x10, 0x00, 0x44),
            widget_border: Color32::from_rgb(0x77, 0x00, 0xff),
        }
    }

    pub fn inferno() -> Self {
        Self {
            background:    Color32::from_rgb(0x10, 0x05, 0x00),
            panel:         Color32::from_rgb(0x1e, 0x0a, 0x00),
            accent:        Color32::from_rgb(0xff, 0x22, 0x00),
            accent_alt:    Color32::from_rgb(0xff, 0x88, 0x00),
            text:          Color32::from_rgb(0xff, 0xf0, 0xdd),
            text_muted:    Color32::from_rgb(0xaa, 0x33, 0x00),
            slider_track:  Color32::from_rgb(0x2a, 0x0e, 0x00),
            selection_bg:  Color32::from_rgb(0x88, 0x14, 0x00),
            border:        Color32::from_rgb(0x38, 0x12, 0x00),
            widget_border: Color32::from_rgb(0xff, 0x44, 0x00),
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
            widget_border: Color32::from_rgb(0x00, 0xcc, 0x55),
        }
    }

    pub fn arctic_nova() -> Self {
        Self {
            background:    Color32::from_rgb(0x00, 0x16, 0x28),
            panel:         Color32::from_rgb(0x00, 0x22, 0x40),
            accent:        Color32::from_rgb(0x00, 0xdd, 0xff),
            accent_alt:    Color32::from_rgb(0xff, 0xff, 0xff),
            text:          Color32::from_rgb(0xe8, 0xfa, 0xff),
            text_muted:    Color32::from_rgb(0x22, 0x88, 0xaa),
            slider_track:  Color32::from_rgb(0x00, 0x30, 0x50),
            selection_bg:  Color32::from_rgb(0x00, 0x55, 0x77),
            border:        Color32::from_rgb(0x00, 0x3c, 0x5a),
            widget_border: Color32::from_rgb(0x00, 0xaa, 0xdd),
        }
    }

    pub fn vaporwave() -> Self {
        Self {
            background:    Color32::from_rgb(0x0c, 0x00, 0x18),
            panel:         Color32::from_rgb(0x18, 0x00, 0x28),
            accent:        Color32::from_rgb(0xff, 0x44, 0xcc),
            accent_alt:    Color32::from_rgb(0x44, 0xee, 0xff),
            text:          Color32::from_rgb(0xff, 0xd8, 0xff),
            text_muted:    Color32::from_rgb(0x99, 0x22, 0xaa),
            slider_track:  Color32::from_rgb(0x22, 0x00, 0x38),
            selection_bg:  Color32::from_rgb(0x77, 0x00, 0x66),
            border:        Color32::from_rgb(0x2c, 0x00, 0x48),
            widget_border: Color32::from_rgb(0xee, 0x00, 0xcc),
        }
    }

    pub fn galactic() -> Self {
        Self {
            background:    Color32::from_rgb(0x06, 0x00, 0x18),
            panel:         Color32::from_rgb(0x0c, 0x00, 0x2c),
            accent:        Color32::from_rgb(0xff, 0xcc, 0x00),
            accent_alt:    Color32::from_rgb(0xbb, 0x00, 0xff),
            text:          Color32::from_rgb(0xf8, 0xee, 0xff),
            text_muted:    Color32::from_rgb(0x88, 0x55, 0xaa),
            slider_track:  Color32::from_rgb(0x10, 0x00, 0x3a),
            selection_bg:  Color32::from_rgb(0x55, 0x33, 0x00),
            border:        Color32::from_rgb(0x18, 0x00, 0x44),
            widget_border: Color32::from_rgb(0xdd, 0xaa, 0x00),
        }
    }

    pub fn coral_storm() -> Self {
        Self {
            background:    Color32::from_rgb(0x00, 0x12, 0x12),
            panel:         Color32::from_rgb(0x00, 0x1e, 0x1e),
            accent:        Color32::from_rgb(0x1a, 0x70, 0x90),
            accent_alt:    Color32::from_rgb(0x00, 0xff, 0xdd),
            text:          Color32::from_rgb(0xff, 0xf4, 0xee),
            text_muted:    Color32::from_rgb(0x22, 0x99, 0x88),
            slider_track:  Color32::from_rgb(0x00, 0x2c, 0x2c),
            selection_bg:  Color32::from_rgb(0x0d, 0x38, 0x4a),
            border:        Color32::from_rgb(0x00, 0x38, 0x38),
            widget_border: Color32::from_rgb(0xff, 0x44, 0x22),
        }
    }

    pub fn ultraviolet() -> Self {
        Self {
            background:    Color32::from_rgb(0x04, 0x00, 0x0c),
            panel:         Color32::from_rgb(0x08, 0x00, 0x1a),
            accent:        Color32::from_rgb(0xcc, 0x00, 0xff),
            accent_alt:    Color32::from_rgb(0xaa, 0xff, 0x00),
            text:          Color32::from_rgb(0xf0, 0xd8, 0xff),
            text_muted:    Color32::from_rgb(0x88, 0x00, 0xcc),
            slider_track:  Color32::from_rgb(0x0e, 0x00, 0x24),
            selection_bg:  Color32::from_rgb(0x44, 0x00, 0xaa),
            border:        Color32::from_rgb(0x14, 0x00, 0x30),
            widget_border: Color32::from_rgb(0xaa, 0x00, 0xee),
        }
    }

    pub fn neon_dusk() -> Self {
        Self {
            background:    Color32::from_rgb(0x10, 0x00, 0x0c),
            panel:         Color32::from_rgb(0x1c, 0x00, 0x18),
            accent:        Color32::from_rgb(0xff, 0x55, 0x00),
            accent_alt:    Color32::from_rgb(0xff, 0x00, 0x88),
            text:          Color32::from_rgb(0xff, 0xec, 0xe0),
            text_muted:    Color32::from_rgb(0xaa, 0x33, 0x44),
            slider_track:  Color32::from_rgb(0x26, 0x00, 0x18),
            selection_bg:  Color32::from_rgb(0x88, 0x22, 0x00),
            border:        Color32::from_rgb(0x30, 0x00, 0x22),
            widget_border: Color32::from_rgb(0xff, 0x44, 0x00),
        }
    }

    pub fn jacked_in() -> Self {
        Self {
            background:    Color32::from_rgb(0x00, 0x00, 0x00),
            panel:         Color32::from_rgb(0x00, 0x08, 0x00),
            accent:        Color32::from_rgb(0x00, 0xff, 0x00),
            accent_alt:    Color32::from_rgb(0x55, 0xff, 0x22),
            text:          Color32::from_rgb(0xdd, 0xff, 0xdd),
            text_muted:    Color32::from_rgb(0x00, 0xaa, 0x22),
            slider_track:  Color32::from_rgb(0x00, 0x0e, 0x00),
            selection_bg:  Color32::from_rgb(0x00, 0x44, 0x11),
            border:        Color32::from_rgb(0x00, 0x18, 0x00),
            widget_border: Color32::from_rgb(0x00, 0xcc, 0x00),
        }
    }

    pub fn molten_glow() -> Self {
        Self {
            background:    Color32::from_rgb(0x0e, 0x08, 0x00),
            panel:         Color32::from_rgb(0x1c, 0x12, 0x00),
            accent:        Color32::from_rgb(0xff, 0xee, 0x00),
            accent_alt:    Color32::from_rgb(0xff, 0x77, 0x00),
            text:          Color32::from_rgb(0xff, 0xfa, 0xcc),
            text_muted:    Color32::from_rgb(0x99, 0x77, 0x00),
            slider_track:  Color32::from_rgb(0x26, 0x18, 0x00),
            selection_bg:  Color32::from_rgb(0x88, 0x77, 0x00),
            border:        Color32::from_rgb(0x34, 0x22, 0x00),
            widget_border: Color32::from_rgb(0xdd, 0xcc, 0x00),
        }
    }
}

pub struct ThemeManager {
    pub choice: ThemeChoice,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self { choice: ThemeChoice::CoralStorm }
    }

    pub fn current(&self) -> AppTheme {
        match self.choice {
            ThemeChoice::NeonVoid    => AppTheme::neon_void(),
            ThemeChoice::CyberRift   => AppTheme::cyber_rift(),
            ThemeChoice::AcidRain    => AppTheme::acid_rain(),
            ThemeChoice::SolarFlare  => AppTheme::solar_flare(),
            ThemeChoice::BloodMoon   => AppTheme::blood_moon(),
            ThemeChoice::CandyPop    => AppTheme::candy_pop(),
            ThemeChoice::ToxicSlime  => AppTheme::toxic_slime(),
            ThemeChoice::DeepSpace   => AppTheme::deep_space(),
            ThemeChoice::Inferno     => AppTheme::inferno(),
            ThemeChoice::GlitchMode  => AppTheme::glitch_mode(),
            ThemeChoice::ArcticNova  => AppTheme::arctic_nova(),
            ThemeChoice::Vaporwave   => AppTheme::vaporwave(),
            ThemeChoice::Galactic    => AppTheme::galactic(),
            ThemeChoice::CoralStorm  => AppTheme::coral_storm(),
            ThemeChoice::Ultraviolet => AppTheme::ultraviolet(),
            ThemeChoice::NeonDusk    => AppTheme::neon_dusk(),
            ThemeChoice::JackedIn    => AppTheme::jacked_in(),
            ThemeChoice::MoltenGlow  => AppTheme::molten_glow(),
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
