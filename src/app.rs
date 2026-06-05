use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::Mutex;
use eframe::egui::{self, Color32, Context, Pos2, RichText, Stroke, Ui};

use crate::audio::{
    devices,
    effects::EffectChain,
    engine::{RealtimeAudioEngine, StartConfig},
    level::LevelBuffer,
    spectrum::SpectrumBuffer,
};
use crate::profiles::{
    self, EffectConfig, EffectType, VoiceProfile,
    built_in,
    factory::build_chain,
};
use crate::settings::{self, AppSettings};
use crate::theme::{ThemeChoice, ThemeManager};
use crate::ui::{about_dialog, spectrum_panel::SpectrumPanel};

// ── Grouped profile categories (built-in only) ────────────────────────────────
const PROFILE_GROUPS: &[(&str, usize, usize)] = &[
    ("CLEAN VOICE",       0,  8),
    ("PITCH & CHARACTER", 8,  16),
    ("SPACE & ROOM",      16, 19),
    ("CREATIVE",          19, 24),
];

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
    engine:            Option<RealtimeAudioEngine>,
    effect_chain:      Arc<Mutex<EffectChain>>,
    status:            String,
    last_error:        Option<String>,
    audio_sample_rate: u32,

    // Profiles — built-ins first, user profiles appended after index `built_in_count`
    profiles:          Vec<VoiceProfile>,
    built_in_count:    usize,
    selected_profile:  Option<usize>,
    live_effects:      Vec<EffectConfig>,

    // New user profile input
    new_profile_name:  String,

    // Spectrum + level
    spectrum:          SpectrumBuffer,
    spectrum_panel:    SpectrumPanel,
    input_level:       LevelBuffer,

    // UI state
    show_about:        bool,
    category_expanded: [bool; 4],

    // Settings
    auto_start:        bool,
    pending_auto_start: bool,
    settings_dirty:    bool,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let saved = settings::load();

        let mut theme = ThemeManager::new();
        theme.choice = saved.theme;
        theme.apply(&cc.egui_ctx);

        let input_devices  = devices::list_input_devices();
        let output_devices = devices::list_output_devices();

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

        // Combine built-in + user profiles into one Vec
        let built_in_profiles = built_in::all();
        let built_in_count    = built_in_profiles.len();
        let user_profiles     = profiles::load_user_profiles();
        let mut all_profiles  = built_in_profiles;
        all_profiles.extend(user_profiles);

        let selected_profile = saved.last_profile_name
            .as_deref()
            .and_then(|name| all_profiles.iter().position(|p| p.name == name))
            .or(Some(0));

        let live_effects = selected_profile
            .and_then(|i| all_profiles.get(i))
            .map(|p| p.effects.clone())
            .unwrap_or_default();

        let devices_ready = !selected_input.is_empty();
        let pending_auto_start = saved.auto_start && devices_ready;

        let mut app = Self {
            theme,
            input_devices,
            output_devices,
            selected_input,
            selected_monitor,
            selected_virtual,
            monitor_enabled:   saved.monitor_enabled,
            virtual_enabled:   saved.virtual_enabled,
            engine:            None,
            effect_chain:      Arc::new(Mutex::new(EffectChain::default())),
            status:            "Stopped".into(),
            last_error:        None,
            audio_sample_rate: 48_000,
            profiles:          all_profiles,
            built_in_count,
            selected_profile,
            live_effects,
            new_profile_name:  String::new(),
            spectrum:          SpectrumBuffer::new(),
            spectrum_panel:    SpectrumPanel::new(),
            input_level:       LevelBuffer::new(),
            show_about:        false,
            category_expanded: saved.category_expanded,
            auto_start:        saved.auto_start,
            pending_auto_start,
            settings_dirty:    false,
        };
        app.apply_chain();
        app
    }

    // ── Chain ─────────────────────────────────────────────────────────────────

    fn apply_chain(&mut self) {
        *self.effect_chain.lock() = build_chain(&self.live_effects);
    }

    fn select_profile(&mut self, idx: usize) {
        self.selected_profile = Some(idx);
        self.live_effects = self.profiles[idx].effects.clone();
        self.apply_chain();
        self.settings_dirty = true;
    }

    // ── User profiles ─────────────────────────────────────────────────────────

    fn save_current_as_profile(&mut self) {
        let name = self.new_profile_name.trim().to_string();
        if name.is_empty() { return; }

        let profile = VoiceProfile::new(name, self.live_effects.clone());
        self.profiles.push(profile);
        self.new_profile_name.clear();
        self.persist_user_profiles();
    }

    fn delete_user_profile(&mut self, relative_idx: usize) {
        let abs = self.built_in_count + relative_idx;
        if abs >= self.profiles.len() { return; }
        if self.selected_profile == Some(abs) {
            self.selected_profile = None;
        } else if let Some(sel) = self.selected_profile {
            if sel > abs { self.selected_profile = Some(sel - 1); }
        }
        self.profiles.remove(abs);
        self.persist_user_profiles();
    }

    fn persist_user_profiles(&self) {
        let user = self.profiles[self.built_in_count..].to_vec();
        profiles::save_user_profiles(&user);
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
        self.spectrum.clear();
        self.input_level.reset();
        match RealtimeAudioEngine::start(
            &cfg,
            Arc::clone(&self.effect_chain),
            self.spectrum.clone(),
            self.input_level.clone(),
        ) {
            Ok(eng) => {
                self.audio_sample_rate = eng.sample_rate;
                self.engine            = Some(eng);
                self.status            = "Running".into();
                self.last_error        = None;
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
        self.input_level.reset();
    }

    fn poll_engine_errors(&mut self) {
        if let Some(ref eng) = self.engine {
            if let Some(err) = eng.take_error() {
                self.last_error = Some(err);
                self.engine     = None;
                self.status     = "Error".into();
                self.input_level.reset();
            }
        }
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
            category_expanded:   self.category_expanded,
            auto_start:          self.auto_start,
        }
    }

    fn reset_to_defaults(&mut self, ctx: &Context) {
        settings::delete();
        self.monitor_enabled  = true;
        self.virtual_enabled  = false;
        self.selected_virtual = String::new();
        self.selected_input   = self.input_devices.first().cloned().unwrap_or_default();
        self.selected_monitor = self.output_devices.first().cloned().unwrap_or_default();
        self.selected_profile = Some(0);
        if let Some(p) = self.profiles.first() { self.live_effects = p.effects.clone(); }
        self.apply_chain();
        self.theme.choice = ThemeChoice::Earth;
        self.theme.apply(ctx);
        self.auto_start = false;
        self.stop_engine();
        self.last_error = None;
    }
}

// ── egui::App ─────────────────────────────────────────────────────────────────

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Auto-start fires on the first frame so the UI is fully up
        if self.pending_auto_start {
            self.pending_auto_start = false;
            self.start_engine();
        }

        self.poll_engine_errors();

        let sr = self.audio_sample_rate;
        self.spectrum_panel.update(&self.spectrum, sr);

        let mut do_reset = false;
        about_dialog::show(ctx, &mut self.show_about, &mut do_reset);
        if do_reset { self.reset_to_defaults(ctx); }

        show_header(self, ctx);
        show_profile_panel(self, ctx);
        show_device_panel(self, ctx);
        show_effect_panel(self, ctx);

        if self.settings_dirty {
            settings::save(&self.current_settings());
            self.settings_dirty = false;
        }
        if self.engine.is_some() {
            ctx.request_repaint();
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        settings::save(&self.current_settings());
    }
}

// ── Shared helpers ────────────────────────────────────────────────────────────

fn section_header(ui: &mut Ui, label: &str, accent: Color32) {
    ui.add_space(6.0);
    ui.label(RichText::new(label).strong().small().color(accent));
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 1.0), egui::Sense::hover());
    ui.painter().line_segment(
        [Pos2::new(rect.left(), rect.center().y), Pos2::new(rect.right(), rect.center().y)],
        Stroke::new(1.0, accent.linear_multiply(0.30)),
    );
    ui.add_space(6.0);
}

/// Horizontal level meter bar (for mic input).
fn level_meter(ui: &mut Ui, level: f32, width: f32, accent: Color32, track: Color32) {
    let height = 8.0;
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    ui.painter().rect_filled(rect, 3.0, track);

    let fill_frac = level.min(1.0);
    if fill_frac > 0.002 {
        let fill_rect = egui::Rect::from_min_size(rect.min, egui::vec2(rect.width() * fill_frac, height));
        let color = vu_color(fill_frac, accent);
        ui.painter().rect_filled(fill_rect, 3.0, color);
    }

    // Clip indicator: right-edge flash when level > 1.0
    if level > 1.0 {
        let clip_rect = egui::Rect::from_min_size(
            Pos2::new(rect.right() - 6.0, rect.top()),
            egui::vec2(6.0, height),
        );
        ui.painter().rect_filled(clip_rect, 1.0, Color32::from_rgb(0xff, 0x22, 0x22));
    }
}

fn vu_color(norm: f32, accent: Color32) -> Color32 {
    let yellow = Color32::from_rgb(0xe8, 0xb8, 0x00);
    let red    = Color32::from_rgb(0xe8, 0x30, 0x30);
    let base = if norm < 0.65 {
        let dim = Color32::from_rgba_unmultiplied(accent.r() / 3, accent.g() / 3, accent.b() / 3, 100);
        lerp_color(dim, accent, norm / 0.65)
    } else if norm < 0.82 {
        lerp_color(accent, yellow, (norm - 0.65) / 0.17)
    } else {
        lerp_color(yellow, red, (norm - 0.82) / 0.18)
    };
    Color32::from_rgba_unmultiplied(base.r(), base.g(), base.b(), lerp_u8(80, 230, norm))
}

fn lerp_color(a: Color32, b: Color32, t: f32) -> Color32 {
    Color32::from_rgb(lerp_u8(a.r(), b.r(), t), lerp_u8(a.g(), b.g(), t), lerp_u8(a.b(), b.b(), t))
}

fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t).clamp(0.0, 255.0) as u8
}

// ── Header ────────────────────────────────────────────────────────────────────

fn show_header(app: &mut App, ctx: &Context) {
    egui::TopBottomPanel::top("header").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("BS-VChanger");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("About").clicked() { app.show_about = true; }
                ui.add_space(8.0);
                let mut choice = app.theme.choice;
                egui::ComboBox::from_id_salt("theme_combo")
                    .selected_text(choice.label())
                    .width(90.0)
                    .show_ui(ui, |ui| {
                        for &t in ThemeChoice::ALL {
                            ui.selectable_value(&mut choice, t, t.label());
                        }
                    });
                if choice != app.theme.choice {
                    app.theme.choice = choice;
                    app.theme.apply(ctx);
                    app.settings_dirty = true;
                }
            });
        });
    });
}

// ── Left panel: Profile list ──────────────────────────────────────────────────

fn show_profile_panel(app: &mut App, ctx: &Context) {
    let theme = app.theme.current();

    egui::SidePanel::left("profiles")
        .min_width(170.0)
        .max_width(220.0)
        .show(ctx, |ui| {
            section_header(ui, "PROFILES", theme.accent);

            egui::ScrollArea::vertical()
                .max_height(ui.available_height() - 70.0)
                .show(ui, |ui| {
                    // ── Built-in profiles with collapsible groups ──────────────
                    for (group_idx, &(group_name, start, end)) in PROFILE_GROUPS.iter().enumerate() {
                        if group_idx > 0 { ui.add_space(4.0); }
                        let expanded = app.category_expanded[group_idx];
                        let arrow = if expanded { "▼" } else { "▶" };
                        let hdr = ui.add(egui::Button::new(
                            RichText::new(format!("{arrow}  {group_name}"))
                                .color(theme.text_muted).small().strong()
                        ).frame(false));
                        if hdr.clicked() {
                            app.category_expanded[group_idx] = !expanded;
                            app.settings_dirty = true;
                        }
                        if expanded {
                            for i in start..end.min(app.profiles.len()) {
                                let selected = app.selected_profile == Some(i);
                                let name = app.profiles[i].name.clone();
                                let resp = ui.selectable_label(
                                    selected,
                                    RichText::new(format!("    {name}"))
                                        .color(if selected { Color32::WHITE } else { theme.text }),
                                );
                                if resp.clicked() && !selected { app.select_profile(i); }
                            }
                        }
                    }

                    // ── User profiles ──────────────────────────────────────────
                    let user_count = app.profiles.len() - app.built_in_count;
                    if user_count > 0 {
                        ui.add_space(6.0);
                        ui.label(RichText::new("MY PROFILES").color(theme.text_muted).small().strong());
                        ui.add_space(2.0);

                        let mut to_delete: Option<usize> = None;
                        for rel in 0..user_count {
                            let abs = app.built_in_count + rel;
                            let selected = app.selected_profile == Some(abs);
                            let name = app.profiles[abs].name.clone();
                            ui.horizontal(|ui| {
                                let resp = ui.selectable_label(
                                    selected,
                                    RichText::new(format!("    {name}"))
                                        .color(if selected { Color32::WHITE } else { theme.text }),
                                );
                                if resp.clicked() && !selected { app.select_profile(abs); }
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.add(egui::Button::new(
                                        RichText::new("×").color(Color32::from_rgb(0xcc, 0x44, 0x44))
                                    ).frame(false)).clicked() {
                                        to_delete = Some(rel);
                                    }
                                });
                            });
                        }
                        if let Some(rel) = to_delete { app.delete_user_profile(rel); }
                    }
                });

            // ── Save current profile ───────────────────────────────────────────
            ui.separator();
            ui.add_space(4.0);
            ui.label(RichText::new("Save current as:").color(theme.text_muted).small());
            ui.horizontal(|ui| {
                ui.add(egui::TextEdit::singleline(&mut app.new_profile_name)
                    .desired_width(110.0)
                    .hint_text("Profile name…"));
                if ui.add(
                    egui::Button::new(RichText::new("Save").color(Color32::WHITE))
                        .fill(if app.new_profile_name.trim().is_empty() {
                            theme.slider_track
                        } else {
                            theme.selection_bg
                        })
                ).clicked() {
                    app.save_current_as_profile();
                }
            });
            ui.add_space(4.0);
        });
}

// ── Right panel: Devices + Transport ─────────────────────────────────────────

fn show_device_panel(app: &mut App, ctx: &Context) {
    let theme = app.theme.current();

    egui::SidePanel::right("devices")
        .min_width(220.0)
        .max_width(300.0)
        .show(ctx, |ui| {
            section_header(ui, "DEVICES", theme.accent);

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

            ui.add_space(4.0);

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
            if app.selected_monitor != prev_mon { app.settings_dirty = true; }

            ui.add_space(4.0);

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

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(6.0);

            // Input level meter
            let level = app.input_level.get();
            ui.label(RichText::new("Input Level").color(theme.text_muted).small());
            level_meter(ui, level, 190.0, theme.accent, theme.slider_track);
            ui.add_space(8.0);

            // Start / Stop
            let running = app.engine.is_some();
            let (btn_label, btn_color) = if running {
                ("■  Stop",  Color32::from_rgb(0xc0, 0x38, 0x38))
            } else {
                ("▶  Start", Color32::from_rgb(0x2a, 0x9a, 0x52))
            };
            if ui.add(
                egui::Button::new(RichText::new(btn_label).strong().color(Color32::WHITE))
                    .fill(btn_color)
                    .min_size(egui::vec2(190.0, 36.0)),
            ).clicked() {
                if running { app.stop_engine(); } else { app.start_engine(); }
            }

            ui.add_space(6.0);

            // Status line — shows state + sample rate when running
            let (dot_color, dot, status_text) = match app.status.as_str() {
                "Running" => (Color32::from_rgb(0x40, 0xcc, 0x70), "●", format!("Running  {} kHz", app.audio_sample_rate / 1000)),
                "Error"   => (Color32::from_rgb(0xe0, 0x50, 0x50), "✖", "Error".into()),
                _         => (Color32::from_rgb(0x66, 0x66, 0x80), "○", "Stopped".into()),
            };
            ui.horizontal(|ui| {
                ui.label(RichText::new(dot).color(dot_color).strong());
                ui.label(RichText::new(status_text).color(dot_color));
            });

            if let Some(ref err) = app.last_error.clone() {
                ui.add_space(4.0);
                ui.colored_label(Color32::from_rgb(0xe0, 0x70, 0x50), format!("⚠ {err}"));
            }

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            // Auto-start toggle
            let prev_auto = app.auto_start;
            ui.checkbox(&mut app.auto_start, "Start automatically on launch");
            if app.auto_start != prev_auto { app.settings_dirty = true; }
        });
}

// ── Center panel: Spectrum + Effect chain ────────────────────────────────────

fn show_effect_panel(app: &mut App, ctx: &Context) {
    let theme  = app.theme.current();
    let accent = theme.accent;
    let active = app.engine.is_some();

    egui::CentralPanel::default().show(ctx, |ui| {
        // Spectrum fills top ~40% of the center panel
        let spec_height = (ui.available_height() * 0.40).clamp(140.0, 260.0);
        ui.allocate_ui(egui::vec2(ui.available_width(), spec_height), |ui| {
            app.spectrum_panel.show(ui, accent, active);
        });

        ui.add_space(4.0);
        section_header(ui, "EFFECT CHAIN", theme.accent);

        if app.live_effects.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label(RichText::new("No effects in this profile").color(theme.text_muted));
            });
            return;
        }

        let mut chain_dirty = false;

        egui::ScrollArea::vertical().show(ui, |ui| {
            for idx in 0..app.live_effects.len() {
                let cfg     = &mut app.live_effects[idx];
                let label   = effect_display_name(cfg.effect_type);
                let summary = effect_summary(cfg.effect_type, &cfg.params);

                let id = egui::Id::new(("fx", idx));
                egui::collapsing_header::CollapsingState::load_with_default_open(ctx, id, false)
                    .show_header(ui, |ui| {
                        let prev = cfg.enabled;
                        ui.checkbox(&mut cfg.enabled, RichText::new(label).strong());
                        if cfg.enabled != prev { chain_dirty = true; }
                        if !summary.is_empty() {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(RichText::new(&summary).color(theme.text_muted).small());
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

        if chain_dirty { app.apply_chain(); }
    });
}

// ── Effect metadata ───────────────────────────────────────────────────────────

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

/// One-line summary of key params shown in the collapsed effect header.
fn effect_summary(t: EffectType, params: &HashMap<String, f64>) -> String {
    let p = |key: &str, default: f64| -> f64 { *params.get(key).unwrap_or(&default) };

    match t {
        EffectType::Gain =>
            format!("{:.2}×", p("gain", 1.0)),
        EffectType::NoiseSuppression =>
            format!("{:.0}%", p("strength", 1.0) * 100.0),
        EffectType::PitchShift => {
            let st = p("semitones", 0.0);
            if st >= 0.0 { format!("+{st:.1} st") } else { format!("{st:.1} st") }
        }
        EffectType::BandpassFilter =>
            format!("{:.0} Hz", p("center_freq", 2000.0)),
        EffectType::Compressor =>
            format!("{:.0}:{:.0}  thr {:.2}", 1, p("ratio", 4.0) as u32, p("threshold", 0.5)),
        EffectType::DeEsser =>
            format!("thr {:.2}", p("threshold", 0.3)),
        EffectType::Echo =>
            format!("{:.2}s  fb {:.2}", p("delay_secs", 0.3), p("feedback", 0.3)),
        EffectType::Reverb =>
            format!("room {:.2}  wet {:.2}", p("room_size", 0.5), p("wet", 0.3)),
        EffectType::Chorus | EffectType::Flanger =>
            format!("{:.1} Hz  wet {:.2}", p("rate", 1.5), p("wet", 0.5)),
        EffectType::Tremolo | EffectType::Vibrato =>
            format!("{:.1} Hz", p("rate", 5.0)),
        EffectType::Distortion =>
            format!("drive {:.1}", p("drive", 2.0)),
        EffectType::LoFi =>
            format!("{:.0}-bit", p("bit_depth", 8.0)),
        EffectType::RingMod =>
            format!("{:.0} Hz", p("carrier_freq", 200.0)),
        EffectType::Robot =>
            format!("{:.0} Hz", p("pitch_hz", 100.0)),
        EffectType::CleanMic => String::new(),
    }
}

fn effect_params_ui(ui: &mut Ui, t: EffectType, params: &mut HashMap<String, f64>) -> bool {
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
        EffectType::CleanMic => {}

        EffectType::NoiseSuppression =>
            { slider!("Strength", "strength", 0.0, 1.0, 1.0); }

        EffectType::Gain =>
            { slider!("Gain", "gain", 0.0, 4.0, 1.0); }

        EffectType::PitchShift =>
            { slider!("Semitones", "semitones", -12.0, 12.0, 0.0); }

        EffectType::BandpassFilter => {
            slider!("Center Hz", "center_freq", 200.0, 8000.0, 2000.0);
            slider!("Q",         "q",           0.1,   10.0,   1.0);
        }
        EffectType::Compressor => {
            slider!("Threshold",   "threshold", 0.05, 1.0,  0.5);
            slider!("Ratio",       "ratio",     1.0,  20.0, 4.0);
            slider!("Attack (s)",  "attack",    0.001,0.1,  0.005);
            slider!("Release (s)", "release",   0.01, 1.0,  0.1);
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
            slider!("Rate (Hz)", "rate",     0.1,   5.0,  0.5);
            slider!("Depth (s)", "depth",    0.001, 0.01, 0.005);
            slider!("Feedback",  "feedback", 0.0,   0.95, 0.5);
            slider!("Wet",       "wet",      0.0,   1.0,  0.5);
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
            slider!("Hard Clip", "hard_clip", 0.0,  1.0, 0.0);
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
