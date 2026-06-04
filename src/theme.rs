use eframe::egui::{Color32, Context, Rounding, Stroke, Style, Visuals};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize, Default)]
pub enum ThemeChoice {
    Mercury,
    Venus,
    #[default]
    Earth,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
    Pluto,
}

impl ThemeChoice {
    pub fn label(self) -> &'static str {
        match self {
            ThemeChoice::Mercury => "Mercury",
            ThemeChoice::Venus   => "Venus",
            ThemeChoice::Earth   => "Earth",
            ThemeChoice::Mars    => "Mars",
            ThemeChoice::Jupiter => "Jupiter",
            ThemeChoice::Saturn  => "Saturn",
            ThemeChoice::Uranus  => "Uranus",
            ThemeChoice::Neptune => "Neptune",
            ThemeChoice::Pluto   => "Pluto",
        }
    }

    pub const ALL: &'static [ThemeChoice] = &[
        ThemeChoice::Mercury,
        ThemeChoice::Venus,
        ThemeChoice::Earth,
        ThemeChoice::Mars,
        ThemeChoice::Jupiter,
        ThemeChoice::Saturn,
        ThemeChoice::Uranus,
        ThemeChoice::Neptune,
        ThemeChoice::Pluto,
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
    /// Mercury — scorched on one side, frozen on the other. Jet black with solar fire.
    pub fn mercury() -> Self {
        Self {
            background:    Color32::from_rgb(0x0a, 0x0a, 0x0a),
            panel:         Color32::from_rgb(0x16, 0x16, 0x16),
            accent:        Color32::from_rgb(0xff, 0x6b, 0x2b),
            accent_alt:    Color32::from_rgb(0xff, 0xcc, 0x00),
            text:          Color32::from_rgb(0xf2, 0xf2, 0xf2),
            text_muted:    Color32::from_rgb(0x77, 0x77, 0x77),
            slider_track:  Color32::from_rgb(0x22, 0x22, 0x22),
            selection_bg:  Color32::from_rgb(0x99, 0x33, 0x00),
            border:        Color32::from_rgb(0x2a, 0x2a, 0x2a),
            widget_border: Color32::from_rgb(0x88, 0x66, 0x55),
        }
    }

    /// Venus — toxic, beautiful, suffocating. Sulfuric acid yellow over deep amber darkness.
    pub fn venus() -> Self {
        Self {
            background:    Color32::from_rgb(0x18, 0x0e, 0x00),
            panel:         Color32::from_rgb(0x28, 0x1a, 0x00),
            accent:        Color32::from_rgb(0xff, 0xd7, 0x00),
            accent_alt:    Color32::from_rgb(0xff, 0x88, 0x00),
            text:          Color32::from_rgb(0xff, 0xf0, 0xcc),
            text_muted:    Color32::from_rgb(0xaa, 0x88, 0x44),
            slider_track:  Color32::from_rgb(0x38, 0x26, 0x00),
            selection_bg:  Color32::from_rgb(0x88, 0x58, 0x00),
            border:        Color32::from_rgb(0x44, 0x32, 0x00),
            widget_border: Color32::from_rgb(0xcc, 0x99, 0x00),
        }
    }

    /// Earth — deep ocean floors, bioluminescence, life. Our pale blue dot.
    pub fn earth() -> Self {
        Self {
            background:    Color32::from_rgb(0x04, 0x0d, 0x18),
            panel:         Color32::from_rgb(0x09, 0x18, 0x2c),
            accent:        Color32::from_rgb(0x00, 0xcc, 0x66),
            accent_alt:    Color32::from_rgb(0x22, 0x99, 0xff),
            text:          Color32::from_rgb(0xe0, 0xf4, 0xf8),
            text_muted:    Color32::from_rgb(0x3a, 0x80, 0x60),
            slider_track:  Color32::from_rgb(0x0e, 0x22, 0x30),
            selection_bg:  Color32::from_rgb(0x0a, 0x60, 0x38),
            border:        Color32::from_rgb(0x10, 0x2a, 0x3c),
            widget_border: Color32::from_rgb(0x1a, 0x88, 0x55),
        }
    }

    /// Mars — the red planet. Dust storms, ancient craters, rust forever.
    pub fn mars() -> Self {
        Self {
            background:    Color32::from_rgb(0x18, 0x08, 0x00),
            panel:         Color32::from_rgb(0x28, 0x12, 0x00),
            accent:        Color32::from_rgb(0xff, 0x45, 0x00),
            accent_alt:    Color32::from_rgb(0xcc, 0x88, 0x44),
            text:          Color32::from_rgb(0xf8, 0xe0, 0xcc),
            text_muted:    Color32::from_rgb(0x99, 0x66, 0x44),
            slider_track:  Color32::from_rgb(0x38, 0x18, 0x00),
            selection_bg:  Color32::from_rgb(0x8a, 0x22, 0x00),
            border:        Color32::from_rgb(0x44, 0x20, 0x00),
            widget_border: Color32::from_rgb(0xcc, 0x55, 0x22),
        }
    }

    /// Jupiter — swirling bands, the Great Red Spot, a storm older than civilization.
    pub fn jupiter() -> Self {
        Self {
            background:    Color32::from_rgb(0x16, 0x0e, 0x04),
            panel:         Color32::from_rgb(0x24, 0x18, 0x08),
            accent:        Color32::from_rgb(0xff, 0x8c, 0x00),
            accent_alt:    Color32::from_rgb(0xff, 0xe4, 0xb5),
            text:          Color32::from_rgb(0xff, 0xee, 0xdd),
            text_muted:    Color32::from_rgb(0x99, 0x77, 0x55),
            slider_track:  Color32::from_rgb(0x30, 0x20, 0x0a),
            selection_bg:  Color32::from_rgb(0x7a, 0x40, 0x00),
            border:        Color32::from_rgb(0x3c, 0x28, 0x10),
            widget_border: Color32::from_rgb(0xcc, 0x77, 0x22),
        }
    }

    /// Saturn — lord of the rings, golden and ancient, impossibly serene.
    pub fn saturn() -> Self {
        Self {
            background:    Color32::from_rgb(0x0e, 0x0c, 0x00),
            panel:         Color32::from_rgb(0x1c, 0x18, 0x00),
            accent:        Color32::from_rgb(0xe8, 0xc0, 0x40),
            accent_alt:    Color32::from_rgb(0xff, 0x99, 0x44),
            text:          Color32::from_rgb(0xff, 0xf8, 0xe0),
            text_muted:    Color32::from_rgb(0x8a, 0x7a, 0x40),
            slider_track:  Color32::from_rgb(0x28, 0x22, 0x00),
            selection_bg:  Color32::from_rgb(0x6a, 0x58, 0x00),
            border:        Color32::from_rgb(0x3a, 0x30, 0x00),
            widget_border: Color32::from_rgb(0xb0, 0x90, 0x20),
        }
    }

    /// Uranus — the sideways ice giant, impossibly cold, impossibly calm.
    pub fn uranus() -> Self {
        Self {
            background:    Color32::from_rgb(0x04, 0x10, 0x14),
            panel:         Color32::from_rgb(0x08, 0x1e, 0x24),
            accent:        Color32::from_rgb(0x40, 0xe0, 0xd0),
            accent_alt:    Color32::from_rgb(0x80, 0xff, 0xee),
            text:          Color32::from_rgb(0xd0, 0xf4, 0xf8),
            text_muted:    Color32::from_rgb(0x30, 0x88, 0x88),
            slider_track:  Color32::from_rgb(0x0e, 0x28, 0x30),
            selection_bg:  Color32::from_rgb(0x08, 0x58, 0x60),
            border:        Color32::from_rgb(0x10, 0x32, 0x40),
            widget_border: Color32::from_rgb(0x20, 0xb8, 0xb0),
        }
    }

    /// Neptune — the storm giant. Winds faster than anything on Earth, forever.
    pub fn neptune() -> Self {
        Self {
            background:    Color32::from_rgb(0x02, 0x06, 0x14),
            panel:         Color32::from_rgb(0x04, 0x10, 0x28),
            accent:        Color32::from_rgb(0x44, 0x88, 0xff),
            accent_alt:    Color32::from_rgb(0x00, 0xcc, 0xff),
            text:          Color32::from_rgb(0xc0, 0xd8, 0xff),
            text_muted:    Color32::from_rgb(0x33, 0x55, 0xaa),
            slider_track:  Color32::from_rgb(0x08, 0x18, 0x38),
            selection_bg:  Color32::from_rgb(0x18, 0x38, 0x99),
            border:        Color32::from_rgb(0x0e, 0x22, 0x4a),
            widget_border: Color32::from_rgb(0x33, 0x66, 0xdd),
        }
    }

    /// Pluto — tiny, frozen, 5.9 billion km away. The heart-shaped Tombaugh Regio in dusty pink.
    pub fn pluto() -> Self {
        Self {
            background:    Color32::from_rgb(0x08, 0x08, 0x12),
            panel:         Color32::from_rgb(0x12, 0x10, 0x1e),
            accent:        Color32::from_rgb(0xdd, 0x88, 0xbb),
            accent_alt:    Color32::from_rgb(0x88, 0x99, 0xcc),
            text:          Color32::from_rgb(0xe0, 0xd0, 0xe8),
            text_muted:    Color32::from_rgb(0x66, 0x55, 0x77),
            slider_track:  Color32::from_rgb(0x1a, 0x18, 0x2a),
            selection_bg:  Color32::from_rgb(0x66, 0x30, 0x66),
            border:        Color32::from_rgb(0x22, 0x20, 0x32),
            widget_border: Color32::from_rgb(0x99, 0x66, 0xaa),
        }
    }
}

pub struct ThemeManager {
    pub choice: ThemeChoice,
}

impl ThemeManager {
    pub fn new() -> Self {
        Self { choice: ThemeChoice::Earth }
    }

    pub fn current(&self) -> AppTheme {
        match self.choice {
            ThemeChoice::Mercury => AppTheme::mercury(),
            ThemeChoice::Venus   => AppTheme::venus(),
            ThemeChoice::Earth   => AppTheme::earth(),
            ThemeChoice::Mars    => AppTheme::mars(),
            ThemeChoice::Jupiter => AppTheme::jupiter(),
            ThemeChoice::Saturn  => AppTheme::saturn(),
            ThemeChoice::Uranus  => AppTheme::uranus(),
            ThemeChoice::Neptune => AppTheme::neptune(),
            ThemeChoice::Pluto   => AppTheme::pluto(),
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
