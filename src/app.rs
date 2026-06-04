use std::sync::Arc;
use parking_lot::Mutex;
use eframe::egui::{self, Color32, Context, RichText, Ui};

use crate::audio::{
    devices,
    effects::EffectChain,
    engine::{RealtimeAudioEngine, StartConfig},
};
use crate::profiles::{
    EffectConfig, EffectType, VoiceProfile,
    built_in,
    factory::build_chain,
};
use crate::settings::{self, AppSettings};
use crate::theme::{ThemeChoice, ThemeManager};
use crate::ui::about_dialog;

pub struct App {
    theme: ThemeManager,

    // Devices
    input_devices:    Vec<String>,
    output_devices:   Vec<String>,
    selected_input:   String,
    selected_monitor: String,
    selected_virtual: String,
    monitor_enabled:  bool,
    virtual_enabled:  bool,

    // Engine
    engine:       Option<RealtimeAudioEngine>,
    effect_chain: Arc<Mutex<EffectChain>>,
    status:       String,
    last_error:   Option<String>,

    // Profiles
    profiles:         Vec<VoiceProfile>,
    selected_profile: Option<usize>,
    live_effects:     Vec<EffectConfig>,

    // UI state
    show_about:       bool,

    // Persistence — true when unsaved changes exist
    settings_dirty:   bool,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let saved = settings::load();

        let mut theme = ThemeManager::new();
        theme.choice = saved.theme;
        theme.apply(&cc.egui_ctx);

        let input_devices  = devices::list_input_devices();
        let output_devices = devices::list_output_devices();

        // Restore device selections, falling back to first available
        let selected_input = saved.input_device_name
            .filter(|n| input_devices.contains(n))
            .or_else(|| input_devices.first().cloned())
            .unwrap_or_default();

        let selected_monitor = saved.monitor_device_name
            .filter(|n| output_devices.contains(n))
            .or_else(|| output_devices.first().cloned())
            .unwrap_or_default();

        let selected_virtual = saved.virtual_device_name
            .filter(|n| output_devices.contains(n))
            .unwrap_or_default();

        let profiles = built_in::all();

        // Restore last profile by name, default to index 0
        let selected_profile = saved.last_profile_name
            .as_deref()
            .and_then(|name| profiles.iter().position(|p| p.name == name))
            .or(Some(0));

        let live_effects = selected_profile
            .and_then(|i| profiles.get(i))
            .map(|p| p.effects.clone())
            .unwrap_or_default();

        let mut app = Self {
            theme,
            input_devices,
            output_devices,
            selected_input,
            selected_monitor,
            selected_virtual,
            monitor_enabled:  saved.monitor_enabled,
            virtual_enabled:  saved.virtual_enabled,
            engine:           None,
            effect_chain:     Arc::new(Mutex::new(EffectChain::default())),
            status:           "Stopped".into(),
            last_error:       None,
            profiles,
            selected_profile,
            live_effects,
            show_about:       false,
            settings_dirty:   false,
        };
        app.apply_chain();
        app
    }

    // ── Settings ──────────────────────────────────────────────────────────────

    fn current_settings(&self) -> AppSettings {
        AppSettings {
            input_device_name:   Some(self.selected_input.clone()).filter(|s| !s.is_empty()),
            monitor_device_name: Some(self.selected_monitor.clone()).filter(|s| !s.is_empty()),
            virtual_device_name: Some(self.selected_virtual.clone()).filter(|s| !s.is_empty()),
            monitor_enabled:     self.monitor_enabled,
            virtual_enabled:     self.virtual_enabled,
            last_profile_name:   self.selected_profile.and_then(|i| self.profiles.get(i)).map(|p| p.name.clone()),
            theme:               self.theme.choice,
        }
    }

    fn reset_to_defaults(&mut self, ctx: &Context) {
        settings::delete();
        let defaults = AppSettings::default();

        self.monitor_enabled = defaults.monitor_enabled;
        self.virtual_enabled = defaults.virtual_enabled;
        self.selected_virtual = String::new();

        // Restore device defaults (first available)
        self.selected_input   = self.input_devices.first().cloned().unwrap_or_default();
        self.selected_monitor = self.output_devices.first().cloned().unwrap_or_default();

        // Restore profile default
        self.selected_profile = Some(0);
        if let Some(p) = self.profiles.first() {
            self.live_effects = p.effects.clone();
        }
        self.apply_chain();

        // Restore theme
        self.theme.choice = defaults.theme;
        self.theme.apply(ctx);

        // Stop engine if running
        self.stop_engine();

        self.last_error = None;
    }

    // ── Chain management ──────────────────────────────────────────────────────

    fn apply_chain(&mut self) {
        *self.effect_chain.lock() = build_chain(&self.live_effects);
    }

    fn select_profile(&mut self, idx: usize) {
        self.selected_profile = Some(idx);
        self.live_effects = self.profiles[idx].effects.clone();
        self.apply_chain();
        self.settings_dirty = true;
    }

    // ── Transport ─────────────────────────────────────────────────────────────

    fn start_engine(&mut self) {
        let cfg = StartConfig {
            input_name:   self.selected_input.clone(),
            monitor_name: self.monitor_enabled.then(|| self.selected_monitor.clone()),
            virtual_name: self.virtual_enabled
                .then(|| self.selected_virtual.clone())
                .filter(|s| !s.is_empty()),
        };
        match RealtimeAudioEngine::start(&cfg, Arc::clone(&self.effect_chain)) {
            Ok(eng) => {
                self.engine     = Some(eng);
                self.status     = "Running".into();
                self.last_error = None;
            }
            Err(e) => {
                self.last_error = Some(e);
                self.status     = "Error".into();
            }
        }
    }

    fn stop_engine(&mut self) {
        self.engine = None;
        self.status = "Stopped".into();
    }

    fn poll_engine_errors(&mut self) {
        if let Some(ref eng) = self.engine {
            if let Some(err) = eng.take_error() {
                self.last_error = Some(err);
                self.engine     = None;
                self.status     = "Error".into();
            }
        }
    }
}

// ── egui::App ─────────────────────────────────────────────────────────────────

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.poll_engine_errors();

        // About modal — evaluated before panels so it can overlay everything
        let mut do_reset = false;
        about_dialog::show(ctx, &mut self.show_about, &mut do_reset);
        if do_reset {
            self.reset_to_defaults(ctx);
        }

        show_header(self, ctx);
        show_profile_panel(self, ctx);
        show_device_panel(self, ctx);
        show_effect_panel(self, ctx);

        // Persist settings whenever something changed
        if self.settings_dirty {
            settings::save(&self.current_settings());
            self.settings_dirty = false;
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Final save on clean exit
        settings::save(&self.current_settings());
    }
}

// ── Header ────────────────────────────────────────────────────────────────────

fn show_header(app: &mut App, ctx: &Context) {
    egui::TopBottomPanel::top("header").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("BS-VChanger");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("About").clicked() {
                    app.show_about = true;
                }
                ui.add_space(8.0);
                let theme_label = match app.theme.choice {
                    ThemeChoice::Dark => "Neon",
                    ThemeChoice::Neon => "Dark",
                };
                if ui.button(theme_label).clicked() {
                    app.theme.toggle();
                    app.theme.apply(ctx);
                    app.settings_dirty = true;
                }
            });
        });
    });
}

// ── Left panel: Profile list ──────────────────────────────────────────────────

fn show_profile_panel(app: &mut App, ctx: &Context) {
    egui::SidePanel::left("profiles")
        .min_width(170.0)
        .max_width(220.0)
        .show(ctx, |ui| {
            ui.add_space(6.0);
            ui.label(RichText::new("PROFILES").strong().small());
            ui.add_space(4.0);
            ui.separator();

            egui::ScrollArea::vertical().show(ui, |ui| {
                for i in 0..app.profiles.len() {
                    let selected = app.selected_profile == Some(i);
                    let name = app.profiles[i].name.clone();
                    if ui.selectable_label(selected, &name).clicked() && !selected {
                        app.select_profile(i);
                    }
                }
            });
        });
}

// ── Right panel: Devices + Transport ─────────────────────────────────────────

fn show_device_panel(app: &mut App, ctx: &Context) {
    egui::SidePanel::right("devices")
        .min_width(220.0)
        .max_width(300.0)
        .show(ctx, |ui| {
            ui.add_space(6.0);
            ui.label(RichText::new("DEVICES").strong().small());
            ui.add_space(4.0);
            ui.separator();
            ui.add_space(6.0);

            // Input
            ui.label("Input");
            let prev_input = app.selected_input.clone();
            egui::ComboBox::from_id_salt("input_dev")
                .selected_text(&app.selected_input)
                .width(190.0)
                .show_ui(ui, |ui| {
                    for name in app.input_devices.clone() {
                        ui.selectable_value(&mut app.selected_input, name.clone(), &name);
                    }
                });
            if app.selected_input != prev_input { app.settings_dirty = true; }

            ui.add_space(6.0);

            // Monitor
            ui.checkbox(&mut app.monitor_enabled, "Monitor");
            let prev_mon = app.selected_monitor.clone();
            ui.add_enabled_ui(app.monitor_enabled, |ui| {
                egui::ComboBox::from_id_salt("monitor_dev")
                    .selected_text(&app.selected_monitor)
                    .width(190.0)
                    .show_ui(ui, |ui| {
                        for name in app.output_devices.clone() {
                            ui.selectable_value(&mut app.selected_monitor, name.clone(), &name);
                        }
                    });
            });
            if app.selected_monitor != prev_mon || app.monitor_enabled != app.monitor_enabled {
                app.settings_dirty = true;
            }

            ui.add_space(6.0);

            // Virtual
            ui.checkbox(&mut app.virtual_enabled, "Virtual");
            let prev_virt = app.selected_virtual.clone();
            ui.add_enabled_ui(app.virtual_enabled, |ui| {
                egui::ComboBox::from_id_salt("virtual_dev")
                    .selected_text(if app.selected_virtual.is_empty() { "— select —" } else { &app.selected_virtual })
                    .width(190.0)
                    .show_ui(ui, |ui| {
                        for name in app.output_devices.clone() {
                            ui.selectable_value(&mut app.selected_virtual, name.clone(), &name);
                        }
                    });
            });
            if app.selected_virtual != prev_virt { app.settings_dirty = true; }

            ui.add_space(16.0);
            ui.separator();
            ui.add_space(8.0);

            // Transport
            let running = app.engine.is_some();
            let (btn_label, btn_color) = if running {
                ("■  Stop",  Color32::from_rgb(0xe0, 0x50, 0x50))
            } else {
                ("▶  Start", Color32::from_rgb(0x50, 0xc0, 0x70))
            };

            if ui.add(
                egui::Button::new(RichText::new(btn_label).strong())
                    .fill(btn_color)
                    .min_size(egui::vec2(190.0, 36.0)),
            ).clicked() {
                if running { app.stop_engine(); } else { app.start_engine(); }
            }

            ui.add_space(6.0);
            let (status_color, status_text) = match app.status.as_str() {
                "Running" => (Color32::from_rgb(0x50, 0xc0, 0x70), "● Running"),
                "Error"   => (Color32::from_rgb(0xe0, 0x50, 0x50), "✖ Error"),
                _         => (Color32::from_rgb(0x88, 0x88, 0x99), "○ Stopped"),
            };
            ui.label(RichText::new(status_text).color(status_color).strong());

            if let Some(ref err) = app.last_error.clone() {
                ui.add_space(4.0);
                ui.colored_label(Color32::from_rgb(0xe0, 0x70, 0x50), format!("⚠ {}", err));
            }
        });
}

// ── Center panel: Effect chain editor ────────────────────────────────────────

fn show_effect_panel(app: &mut App, ctx: &Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add_space(6.0);
        ui.label(RichText::new("EFFECT CHAIN").strong().small());
        ui.add_space(4.0);
        ui.separator();
        ui.add_space(4.0);

        if app.live_effects.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("No effects in this profile")
                    .color(Color32::from_rgb(0x66, 0x66, 0x77)));
            });
            return;
        }

        let mut chain_dirty = false;

        egui::ScrollArea::vertical().show(ui, |ui| {
            for idx in 0..app.live_effects.len() {
                let cfg = &mut app.live_effects[idx];
                let label = effect_display_name(cfg.effect_type);
                let has_params = effect_has_params(cfg.effect_type);

                let id = egui::Id::new(("fx", idx));
                egui::collapsing_header::CollapsingState::load_with_default_open(ctx, id, false)
                    .show_header(ui, |ui| {
                        let prev = cfg.enabled;
                        ui.checkbox(&mut cfg.enabled, RichText::new(label).strong());
                        if cfg.enabled != prev { chain_dirty = true; }
                        if !has_params {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new("no params")
                                    .color(Color32::from_rgb(0x55, 0x55, 0x66)).small());
                            });
                        }
                    })
                    .body(|ui| {
                        if effect_params_ui(ui, cfg.effect_type, &mut cfg.params) {
                            chain_dirty = true;
                        }
                    });

                ui.separator();
            }
        });

        if chain_dirty {
            app.apply_chain();
        }
    });
}

// ── Effect helpers ────────────────────────────────────────────────────────────

fn effect_display_name(t: EffectType) -> &'static str {
    match t {
        EffectType::Gain             => "Gain",
        EffectType::CleanMic         => "Clean Mic",
        EffectType::NoiseSuppression => "Noise Suppression",
        EffectType::PitchShift       => "Pitch Shift",
        EffectType::BandpassFilter   => "Bandpass Filter",
        EffectType::Compressor       => "Compressor",
        EffectType::DeEsser          => "De-Esser",
        EffectType::Echo             => "Echo",
        EffectType::Reverb           => "Reverb",
        EffectType::Chorus           => "Chorus",
        EffectType::Flanger          => "Flanger",
        EffectType::Tremolo          => "Tremolo",
        EffectType::Vibrato          => "Vibrato",
        EffectType::Distortion       => "Distortion",
        EffectType::LoFi             => "Lo-Fi",
        EffectType::RingMod          => "Ring Mod",
        EffectType::Robot            => "Robot",
    }
}

fn effect_has_params(t: EffectType) -> bool {
    !matches!(t, EffectType::CleanMic | EffectType::NoiseSuppression)
}

fn effect_params_ui(
    ui: &mut Ui,
    t: EffectType,
    params: &mut std::collections::HashMap<String, f64>,
) -> bool {
    let mut changed = false;

    macro_rules! slider {
        ($label:expr, $key:expr, $lo:expr, $hi:expr, $default:expr) => {{
            let mut v = *params.entry($key.into()).or_insert($default) as f32;
            if ui.add(egui::Slider::new(&mut v, ($lo as f32)..=($hi as f32)).text($label)).changed() {
                params.insert($key.into(), v as f64);
                changed = true;
            }
        }};
    }

    match t {
        EffectType::CleanMic | EffectType::NoiseSuppression => {}

        EffectType::Gain =>
            { slider!("Gain", "gain", 0.0, 4.0, 1.0); }

        EffectType::PitchShift =>
            { slider!("Semitones", "semitones", -12.0, 12.0, 0.0); }

        EffectType::BandpassFilter => {
            slider!("Center Hz", "center_freq", 200.0, 8000.0, 2000.0);
            slider!("Q",         "q",           0.1,   10.0,   1.0);
        }

        EffectType::Compressor => {
            slider!("Threshold",  "threshold", 0.05, 1.0,  0.5);
            slider!("Ratio",      "ratio",     1.0,  20.0, 4.0);
            slider!("Attack (s)", "attack",    0.001,0.1,  0.005);
            slider!("Release (s)","release",   0.01, 1.0,  0.1);
        }

        EffectType::DeEsser => {
            slider!("Threshold", "threshold", 0.05, 1.0, 0.3);
            slider!("Reduction", "reduction", 0.0,  1.0, 0.3);
        }

        EffectType::Echo => {
            slider!("Delay (s)", "delay_secs", 0.05, 2.0,  0.3);
            slider!("Feedback",  "feedback",   0.0,  0.95, 0.3);
            slider!("Wet",       "wet",        0.0,  1.0,  0.5);
        }

        EffectType::Reverb => {
            slider!("Room Size", "room_size", 0.0, 1.0, 0.5);
            slider!("Wet",       "wet",       0.0, 1.0, 0.3);
        }

        EffectType::Chorus => {
            slider!("Rate (Hz)", "rate",  0.1,   8.0,  1.5);
            slider!("Depth",     "depth", 0.001, 0.02, 0.003);
            slider!("Wet",       "wet",   0.0,   1.0,  0.5);
        }

        EffectType::Flanger => {
            slider!("Rate (Hz)", "rate",     0.1,  5.0,  0.5);
            slider!("Depth (s)", "depth",    0.001,0.01, 0.005);
            slider!("Feedback",  "feedback", 0.0,  0.95, 0.5);
            slider!("Wet",       "wet",      0.0,  1.0,  0.5);
        }

        EffectType::Tremolo => {
            slider!("Rate (Hz)", "rate",  0.5, 20.0, 5.0);
            slider!("Depth",     "depth", 0.0,  1.0, 0.7);
        }

        EffectType::Vibrato => {
            slider!("Rate (Hz)", "rate",  0.5, 20.0,  5.0);
            slider!("Depth (s)", "depth", 0.001, 0.01, 0.003);
        }

        EffectType::Distortion => {
            slider!("Drive",     "drive",     1.0, 10.0, 2.0);
            slider!("Hard clip", "hard_clip", 0.0,  1.0, 0.0);
        }

        EffectType::LoFi => {
            slider!("Bit Depth",  "bit_depth",  4.0, 16.0, 8.0);
            slider!("Downsample", "downsample", 1.0,  8.0, 2.0);
        }

        EffectType::RingMod =>
            { slider!("Carrier Hz", "carrier_freq", 20.0, 2000.0, 200.0); }

        EffectType::Robot =>
            { slider!("Pitch Hz", "pitch_hz", 50.0, 500.0, 100.0); }
    }

    changed
}
