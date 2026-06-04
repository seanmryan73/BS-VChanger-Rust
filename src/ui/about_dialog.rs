use eframe::egui::{self, Color32, Context, RichText, ScrollArea, Ui};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Draws the About / Help modal. Returns `true` while the window should remain open.
pub fn show(ctx: &Context, open: &mut bool, on_reset: &mut bool) {
    if !*open {
        return;
    }

    egui::Window::new("BS-VChanger — Help & About")
        .collapsible(false)
        .resizable(false)
        .fixed_size([460.0, 540.0])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                draw_content(ui, open, on_reset);
            });
        });
}

fn draw_content(ui: &mut Ui, open: &mut bool, on_reset: &mut bool) {
    let accent = Color32::from_rgb(0x5b, 0x9c, 0xf6);
    let muted  = Color32::from_rgb(0x88, 0x88, 0x99);
    let warn   = Color32::from_rgb(0xe0, 0x90, 0x50);

    // ── Title ─────────────────────────────────────────────────────────────────
    ui.vertical_centered(|ui| {
        ui.add_space(8.0);
        ui.label(RichText::new("BS-VChanger").size(22.0).strong().color(accent));
        ui.label(RichText::new(format!("v{VERSION}  •  Real-time voice changer for Windows"))
            .color(muted).small());
    });

    ui.add_space(12.0);
    ui.separator();
    ui.add_space(8.0);

    // ── Quick Start ───────────────────────────────────────────────────────────
    ui.label(RichText::new("Quick Start").strong());
    ui.add_space(4.0);

    for (n, step) in [
        ("1.", "Select your microphone in the Input dropdown."),
        ("2.", "Select your speakers or headphones as Monitor output."),
        ("3.", "Choose a profile from the left panel (start with Clean Voice)."),
        ("4.", "Click Start — you will hear your processed voice through the monitor."),
    ] {
        ui.horizontal(|ui| {
            ui.label(RichText::new(n).color(accent).strong());
            ui.label(step);
        });
    }

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(8.0);

    // ── Profiles guide ────────────────────────────────────────────────────────
    ui.label(RichText::new("Profile Guide").strong());
    ui.add_space(4.0);

    let profiles = [
        ("Clean Voice",    "Noise suppression + light compression. Best starting point."),
        ("Noise Reduction","RNNoise denoiser only — removes background room noise."),
        ("Studio Voice",   "Full broadcast chain: noise reduction, de-esser, EQ, compress."),
        ("Conference Call","Optimised for meeting clarity — tight EQ and compression."),
        ("Podcast",        "Warm, present voice with gentle de-essing."),
        ("Telephone",      "Narrow EQ + lo-fi to sound like a phone call."),
        ("Radio",          "AM radio character with mild distortion."),
        ("Robot",          "Comb-filter pitch effect for a robotic tone."),
    ];
    for (name, desc) in profiles {
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(name).strong().color(accent));
            ui.label(RichText::new("—").color(muted));
            ui.label(desc);
        });
    }

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(8.0);

    // ── Virtual output ────────────────────────────────────────────────────────
    ui.label(RichText::new("Virtual Output (VB-CABLE)").strong());
    ui.add_space(4.0);
    ui.label(
        "The Virtual output sends your processed voice to a virtual microphone \
         that Discord, Zoom, OBS and other apps can see as an input device.",
    );
    ui.add_space(4.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Requires:").strong());
        ui.label("Install VB-CABLE from vb-audio.com (free). After install, \
                  select \"CABLE Input\" as the Virtual output here, then set \
                  \"CABLE Output\" as the microphone inside your meeting app.");
    });

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(8.0);

    // ── Effect chain tips ─────────────────────────────────────────────────────
    ui.label(RichText::new("Effect Chain Tips").strong());
    ui.add_space(4.0);
    for tip in [
        "Click an effect name to expand its parameters.",
        "Uncheck an effect to bypass it without removing it.",
        "Noise Suppression works only at 48 kHz (the Windows WASAPI default).",
        "Pitch Shift changes the speed of your voice — larger values sound more extreme.",
        "Stack Compressor after Noise Suppression for the cleanest result.",
    ] {
        ui.horizontal(|ui| {
            ui.label(RichText::new("•").color(accent));
            ui.label(tip);
        });
    }

    ui.add_space(12.0);
    ui.separator();
    ui.add_space(8.0);

    // ── Reset + Close ─────────────────────────────────────────────────────────
    ui.horizontal(|ui| {
        if ui.add(
            egui::Button::new(RichText::new("Reset to Defaults").color(warn))
                .min_size(egui::vec2(150.0, 28.0)),
        ).on_hover_text("Clears saved device and profile settings. Cannot be undone.")
        .clicked()
        {
            *on_reset = true;
            *open = false;
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.add(
                egui::Button::new("Close").min_size(egui::vec2(80.0, 28.0)),
            ).clicked() {
                *open = false;
            }
        });
    });

    ui.add_space(8.0);
    ui.vertical_centered(|ui| {
        ui.label(RichText::new("BS-VChanger is open source. Built with Rust + egui.")
            .color(muted).small());
    });
    ui.add_space(4.0);
}
