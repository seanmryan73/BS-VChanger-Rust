use eframe::egui::{self, Color32, Context, RichText, ScrollArea, Ui};

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn show(ctx: &Context, open: &mut bool, on_reset: &mut bool) {
    if !*open {
        return;
    }

    egui::Window::new("BS-VChanger — Help & About")
        .collapsible(false)
        .resizable(false)
        .fixed_size([500.0, 760.0])
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                draw_content(ui, open, on_reset);
            });
        });
}

fn draw_content(ui: &mut Ui, open: &mut bool, on_reset: &mut bool) {
    let accent = Color32::from_rgb(0x00, 0xe5, 0xc8);
    let muted  = Color32::from_rgb(0x77, 0x88, 0x99);
    let warn   = Color32::from_rgb(0xe0, 0x90, 0x50);
    let dim    = Color32::from_rgb(0x55, 0x66, 0x77);

    // ── Title ─────────────────────────────────────────────────────────────────
    ui.vertical_centered(|ui| {
        ui.add_space(8.0);
        ui.label(RichText::new("BS").size(28.0).strong().color(accent));
        ui.label(RichText::new("▸ VCHANGER").size(13.0).color(muted));
        ui.add_space(4.0);
        ui.label(RichText::new(format!("v{VERSION}  •  Real-time voice changer for Windows"))
            .color(muted).small());
        ui.add_space(2.0);
        ui.label(
            RichText::new("26 profiles  •  21 themes  •  17 effects  •  single EXE")
                .color(dim).small(),
        );
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
        ("1.", "Select your microphone in the INPUT dropdown."),
        ("2.", "Enable VIRTUAL OUTPUT and select \"CABLE Input\"."),
        ("3.", "Leave MONITOR off — see Troubleshooting below for why."),
        ("4.", "In your meeting app, set the mic to \"CABLE Output\"."),
        ("5.", "Choose a profile and click START."),
    ] {
        ui.horizontal(|ui| {
            ui.label(RichText::new(n).color(accent).strong());
            ui.label(step);
        });
    }
    ui.add_space(6.0);
    ui.label(RichText::new("For personal monitoring / testing:").color(accent).small().strong());
    ui.add_space(2.0);
    for (n, step) in [
        ("1.", "Select your microphone in the INPUT dropdown."),
        ("2.", "Enable MONITOR and select your headphones or speakers."),
        ("3.", "Choose a profile and click START — you'll hear your processed voice live."),
    ] {
        ui.horizontal(|ui| {
            ui.label(RichText::new(n).color(accent).strong());
            ui.label(step);
        });
    }

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(8.0);

    // ── Profiles ──────────────────────────────────────────────────────────────
    ui.label(RichText::new("Built-in Profiles").strong());
    ui.add_space(6.0);

    let groups: &[(&str, &[(&str, &str)])] = &[
        ("CLEAN VOICE", &[
            ("BagPipe Clean",   "Noise suppression + compression. The everyday go-to."),
            ("BagPipe Podcast", "Podcast chain: noise gate, de-esser, compressor, gain."),
            ("Clean Voice",     "Simple clean pass: noise reduction + light compression."),
            ("Noise Reduction", "RNNoise denoiser only — removes room/background noise."),
            ("Studio Voice",    "Full broadcast chain: EQ, de-esser, compressor."),
            ("Conference Call", "Optimised for meeting clarity — tight EQ and compression."),
            ("Podcast",         "Warm, present sound with gentle de-essing."),
            ("Clarity Boost",   "Presence EQ lift — cuts mud, adds articulation."),
            ("Broadcast",       "High-ratio compression for consistent loudness."),
        ]),
        ("PITCH & CHARACTER", &[
            ("Gentle Compress", "Light compression only — natural dynamics."),
            ("Deep Voice",      "Pitch -4 semitones. Deeper, more authoritative."),
            ("High Voice",      "Pitch +4 semitones. Lighter, brighter character."),
            ("Chipmunk",        "Pitch +12 semitones. Cartoon-chipmunk effect."),
            ("Giant",           "Pitch -8 semitones + reverb. Massive and imposing."),
            ("Telephone",       "Narrow bandpass + lo-fi. Classic phone-call sound."),
            ("Radio",           "AM radio character with mild soft distortion."),
            ("Walkie-Talkie",   "Hard-clipped, filtered, low-bit — handheld radio."),
        ]),
        ("SPACE & ROOM", &[
            ("Megaphone",    "Harsh bandpass + overdrive. Outdoor announcement feel."),
            ("Echo Chamber", "Short delay + reverb. Empty room ambience."),
            ("Cathedral",    "Long, lush reverb + chorus. Enormous space."),
        ]),
        ("CREATIVE", &[
            ("Underwater", "Low bandpass + chorus + reverb. Submerged and eerie."),
            ("Robot",      "Comb-filter pitch modulation for a robotic buzz."),
            ("Alien",      "Pitch shift + ring modulator. Not from around here."),
            ("Daemon",     "Pitch -10 + dark ring mod + heavy reverb."),
            ("Vintage",    "Lo-fi bit reduction + echo. Worn tape machine."),
        ]),
    ];

    for &(group_name, profiles) in groups {
        ui.label(RichText::new(group_name).small().strong().color(muted));
        ui.add_space(2.0);
        for &(name, desc) in profiles {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(format!("  {name}")).strong().color(accent));
                ui.label(RichText::new("—").color(dim));
                ui.label(RichText::new(desc).small());
            });
        }
        ui.add_space(6.0);
    }

    ui.add_space(4.0);
    ui.separator();
    ui.add_space(8.0);

    // ── Themes ────────────────────────────────────────────────────────────────
    ui.label(RichText::new("Themes  (21 colour palettes)").strong());
    ui.add_space(4.0);

    let theme_groups: &[(&str, &str)] = &[
        ("BagPipes",   "BagPipes Green · BagPipes Pink · BagPipes Purple"),
        ("Planets",    "Mercury · Venus · Earth · Mars · Jupiter · Saturn · Uranus · Neptune · Pluto"),
        ("Mythology",  "Zeus · Hades · Poseidon · Aphrodite · Ares · Athena · Apollo · Artemis · Dionysus"),
    ];
    for &(group, names) in theme_groups {
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(group).strong().color(accent));
            ui.label(RichText::new("—").color(dim));
            ui.label(RichText::new(names).small().color(muted));
        });
    }
    ui.add_space(4.0);
    ui.label(RichText::new(
        "Switch themes from the dropdown in the top-right corner. Your choice persists across restarts."
    ).small().color(muted));

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(8.0);

    // ── Effect Chain Tips ─────────────────────────────────────────────────────
    ui.label(RichText::new("Effect Chain Tips").strong());
    ui.add_space(4.0);
    for tip in [
        "Each effect has a coloured LED dot — bright when active, dark when bypassed.",
        "Click the arrow on any effect to expand its parameters.",
        "Uncheck an effect to bypass it without removing it from the chain.",
        "Noise Suppression works only at 48 kHz (the standard Windows WASAPI rate).",
        "Pitch Shift changes pitch in semitones — ±12 covers a full octave.",
        "Stack Compressor after Noise Suppression for the cleanest result.",
        "The spectrum analyser shows frequency content in real time, with a mirror\n  \
         reflection below the floor line while the engine is running.",
    ] {
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("•").color(accent));
            ui.label(tip);
        });
    }

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(8.0);

    // ── Virtual Output ────────────────────────────────────────────────────────
    ui.label(RichText::new("Virtual Output (VB-CABLE)").strong());
    ui.add_space(4.0);
    ui.label(
        "The VIRTUAL OUTPUT routes processed audio to a virtual microphone that \
         Teams, Discord, Zoom, OBS and other apps can use as their input source.",
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
            "VB-CABLE passes audio internally — you won't hear it \
             through your own speakers. Enable MONITOR for personal feedback."
        ).color(warn));
    });

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(8.0);

    // ── Troubleshooting ───────────────────────────────────────────────────────
    ui.label(RichText::new("Troubleshooting").strong());
    ui.add_space(4.0);

    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("⚠").color(warn).strong());
        ui.label(RichText::new("MONITOR ON + VIRTUAL ON = Teams/Zoom gets silence.").strong());
    });
    ui.add_space(2.0);
    ui.label(
        "When Monitor is active, processed audio plays through your speakers. \
         Teams/Zoom AEC compares what the speakers play against what CABLE Output \
         receives — they match, so AEC cancels it as echo, leaving silence.",
    );
    ui.add_space(4.0);
    ui.label(RichText::new("Fixes:").strong().color(accent));
    for fix in [
        "For calls: disable MONITOR, use VIRTUAL only.",
        "For personal feedback: disable VIRTUAL, use MONITOR only.",
        "Need both: turn off echo cancellation in your meeting app.",
    ] {
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("•").color(accent));
            ui.label(fix);
        });
    }

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(8.0);

    // ── Files & Data ──────────────────────────────────────────────────────────
    ui.label(RichText::new("Files & Data").strong());
    ui.add_space(4.0);

    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Standalone:").strong());
        ui.label("Single EXE, no installer, no DLLs. Copy it anywhere and run it.");
    });
    ui.add_space(6.0);

    ui.label(RichText::new("Data files:").color(muted).small());
    ui.add_space(2.0);
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| "%APPDATA%".into());
    ui.label(RichText::new(format!("{appdata}\\BS-VChanger-Rust\\"))
        .color(accent).monospace().small());
    ui.add_space(4.0);

    for (file, desc) in [
        ("settings.json",       "Devices, theme, mic volume, last profile, auto-start"),
        ("user_profiles.json",  "Your saved custom profiles"),
    ] {
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(file).monospace().color(accent).small());
            ui.label(RichText::new(format!("— {desc}")).color(muted).small());
        });
    }
    ui.add_space(4.0);
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new("Tip:").strong().color(muted).small());
        ui.label(RichText::new(
            "Delete settings.json to fully reset, or use \"Reset to Defaults\" below."
        ).small());
    });

    ui.add_space(10.0);
    ui.separator();
    ui.add_space(8.0);

    // ── Credits ───────────────────────────────────────────────────────────────
    ui.label(RichText::new("About").strong());
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
    ui.horizontal(|ui| {
        ui.label(RichText::new("Stack").strong().color(muted).small());
        ui.label(RichText::new("Rust · egui · WASAPI · RNNoise · rubato").color(muted));
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
        ui.label(RichText::new("Built with Rust + egui  •  Ships as a single Windows EXE")
            .color(dim).small());
    });
    ui.add_space(4.0);
}
