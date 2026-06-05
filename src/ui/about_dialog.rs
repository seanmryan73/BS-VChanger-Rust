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
        .fixed_size([480.0, 680.0])
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

    ui.label(RichText::new("For calls (Teams / Zoom / Discord):").color(accent).small().strong());
    ui.add_space(2.0);
    for (n, step) in [
        ("1.", "Select your microphone in the Input dropdown."),
        ("2.", "Enable Virtual and select \"CABLE Input\" as the output."),
        ("3.", "Leave Monitor OFF — see the Troubleshooting section below for why."),
        ("4.", "In your meeting app, set the microphone to \"CABLE Output\"."),
        ("5.", "Choose a profile and click Start."),
    ] {
        ui.horizontal(|ui| {
            ui.label(RichText::new(n).color(accent).strong());
            ui.label(step);
        });
    }
    ui.add_space(6.0);
    ui.label(RichText::new("For personal listening / testing:").color(accent).small().strong());
    ui.add_space(2.0);
    for (n, step) in [
        ("1.", "Select your microphone in the Input dropdown."),
        ("2.", "Enable Monitor and select your headphones or speakers."),
        ("3.", "Choose a profile and click Start — you will hear your processed voice."),
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
         that Teams, Discord, Zoom, OBS and other apps can use as their input.",
    );
    ui.add_space(4.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Setup:").strong());
        ui.label("Install VB-CABLE from vb-audio.com (free). Select \
                  \"CABLE Input\" as the Virtual output here, then set \
                  \"CABLE Output\" as the microphone in your meeting app.");
    });
    ui.add_space(4.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Note:").strong().color(warn));
        ui.label(RichText::new(
            "VB-CABLE passes audio internally — you will not hear it \
             through your own speakers. Use Monitor for personal feedback."
        ).color(warn));
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

    // ── Troubleshooting ───────────────────────────────────────────────────────
    ui.label(RichText::new("Troubleshooting").strong());
    ui.add_space(4.0);

    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("⚠").color(warn).strong());
        ui.label(RichText::new("Monitor ON + Virtual ON = Teams/Zoom gets silence.").strong());
    });
    ui.add_space(2.0);
    ui.label(
        "When Monitor is active, your processed voice plays through your speakers. \
         Teams and Zoom run echo cancellation (AEC) that compares what your \
         speakers are playing against what your microphone (CABLE Output) is \
         receiving. Because both carry the same signal, AEC identifies it as \
         echo and cancels it — leaving silence.",
    );
    ui.add_space(4.0);
    ui.label(RichText::new("Fix:").strong().color(accent));
    for fix in [
        "For calls — disable Monitor, use Virtual only.",
        "For personal feedback — disable Virtual, use Monitor only.",
        "If you need both: disable echo cancellation in Teams (Settings → Devices \
         → Noise suppression → Off / Echo cancellation → Off).",
    ] {
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("•").color(accent));
            ui.label(fix);
        });
    }

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(8.0);

    // ── Who Made This? ───────────────────────────────────────────────────────
    ui.label(RichText::new("Who Made This?").strong());
    ui.add_space(6.0);

    ui.vertical_centered(|ui| {
        ui.label(RichText::new("♫  BagPipes Software  ♫").size(18.0).strong().color(accent));
        ui.add_space(2.0);
        ui.label(
            RichText::new("\"Making noise sound better, one Rust panic at a time\"")
                .color(muted).italics().small(),
        );
    });

    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label(RichText::new("Author").strong().color(muted).small());
        ui.label(RichText::new("Sean Ryan").strong());
    });
    ui.horizontal(|ui| {
        ui.label(RichText::new("Contact").strong().color(muted).small());
        ui.label(RichText::new("seanmryan@gmail.com").color(accent));
    });

    ui.add_space(8.0);

    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("P.S.").strong().color(warn));
        ui.label(
            "The \"BS\" in BS-VChanger stands for BagPipes Software. \
             We know what you were thinking. We respect it.",
        );
    });

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
