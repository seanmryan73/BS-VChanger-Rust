# egui Layout Reference — BS-VChanger

**Framework:** egui 0.29 / eframe 0.29  
**Platform:** Windows native (no web target)  
**Source file:** `src/app.rs`

---

## Layout Overview

```
┌─────────────────────────────────────────────────────────┐
│  HEADER (TopBottomPanel::top)   title | theme | about   │
├────────────────┬───────────────────────┬────────────────┤
│                │                       │                │
│  PROFILES      │   EFFECT PANEL        │  DEVICES       │
│  SidePanel     │   CentralPanel        │  SidePanel     │
│  ::left()      │                       │  ::right()     │
│                │   [spectrum]          │                │
│  - passthrough │   [effect chain]      │  - input       │
│  - grouped     │                       │  - monitor     │
│    built-ins   │                       │  - virtual     │
│  - user list   │                       │  - mic vol     │
│  - save form   │                       │  - level meter │
│                │                       │  - start/stop  │
├────────────────┴───────────────────────┴────────────────┤
│  STATUS BAR (TopBottomPanel::bottom)  profile | status  │
└─────────────────────────────────────────────────────────┘
```

Panel declaration order matters in egui: Top/Bottom panels must be declared before Side panels, and Side panels before CentralPanel.

---

## AppState Struct

```rust
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
    status:            String,             // "Running" | "Stopped" | "Error"
    last_error:        Option<String>,
    audio_sample_rate: u32,

    // Profiles — built-ins first, user profiles appended after index `built_in_count`
    profiles:          Vec<VoiceProfile>,
    built_in_count:    usize,
    selected_profile:  Option<usize>,
    live_effects:      Vec<EffectConfig>,  // mutable working copy

    // User profile input
    new_profile_name:  String,

    // Spectrum + level
    spectrum:          SpectrumBuffer,
    spectrum_panel:    SpectrumPanel,
    input_level:       LevelBuffer,
    input_gain:        Arc<AtomicU32>,     // f32 bits stored atomically for audio thread sharing

    // UI state
    show_about:        bool,
    category_expanded: [bool; 4],          // one per profile group

    // Settings persistence
    auto_start:         bool,
    pending_auto_start: bool,
    settings_dirty:     bool,
}
```

**Key design decisions:**

- `live_effects` is a working copy cloned from the selected profile. Changes here are applied to the engine but not saved to `profiles[idx]` until the user clicks ↑ (update) or saves a new profile.
- `input_gain` is `Arc<AtomicU32>` so the audio thread can read it lock-free while the UI thread writes it.
- `settings_dirty` is a single dirty flag; settings are flushed to disk at the end of each `update()` frame where it is set.

---

## `eframe::App` impl — Update Loop

```rust
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

        // Panel rendering order: top → bottom → left → right → center
        show_header(self, ctx);
        show_status_bar(self, ctx);
        show_profile_panel(self, ctx);
        show_device_panel(self, ctx);
        show_effect_panel(self, ctx);

        if self.settings_dirty {
            settings::save(&self.current_settings());
            self.settings_dirty = false;
        }
        // Keep repainting while engine is running (spectrum/level animation)
        if self.engine.is_some() {
            ctx.request_repaint();
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        settings::save(&self.current_settings());
    }
}
```

---

## UI Helper Methods (App impl)

### State mutators called from UI handlers

```rust
fn select_profile(&mut self, idx: usize) {
    self.selected_profile = Some(idx);
    self.live_effects = self.profiles[idx].effects.clone();
    self.apply_chain();
    self.settings_dirty = true;
}

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
    // Adjust selection index when items shift
    if self.selected_profile == Some(abs) {
        self.selected_profile = None;
    } else if let Some(sel) = self.selected_profile {
        if sel > abs { self.selected_profile = Some(sel - 1); }
    }
    self.profiles.remove(abs);
    self.persist_user_profiles();
}

fn update_user_profile(&mut self, relative_idx: usize) {
    let abs = self.built_in_count + relative_idx;
    if abs < self.profiles.len() {
        self.profiles[abs].effects = self.live_effects.clone();
        self.persist_user_profiles();
    }
}

fn reset_profile(&mut self) {
    if let Some(idx) = self.selected_profile {
        self.live_effects = self.profiles[idx].effects.clone();
        self.apply_chain();
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
    self.theme.choice = ThemeChoice::BagpipesGreen;
    self.theme.apply(ctx);
    self.auto_start = false;
    self.input_gain.store(1.0f32.to_bits(), Ordering::Relaxed);
    self.stop_engine();
    self.last_error = None;
}
```

---

## Panel Functions

All five panel functions are free functions (not methods) that take `(&mut App, &Context)`. This avoids borrow conflicts when panels need to call multiple `&mut self` methods.

### `show_header`

```rust
fn show_header(app: &mut App, ctx: &Context) {
    let theme = app.theme.current();
    egui::TopBottomPanel::top("header")
        .min_height(46.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(4.0);
                ui.label(RichText::new("BS").size(22.0).strong().color(theme.accent));
                ui.label(RichText::new("▸ VCHANGER").size(12.0).color(theme.text_muted));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // About button
                    if ui.add(egui::Button::new(
                        RichText::new("?").color(theme.text_muted)
                    ).frame(false)).on_hover_text("About").clicked() {
                        app.show_about = true;
                    }
                    ui.add_space(8.0);
                    // Theme selector
                    let mut choice = app.theme.choice;
                    egui::ComboBox::from_id_salt("theme_combo")
                        .selected_text(choice.label())
                        .width(120.0)
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
```

### `show_status_bar`

```rust
fn show_status_bar(app: &App, ctx: &Context) {
    let theme = app.theme.current();
    egui::TopBottomPanel::bottom("status_bar")
        .min_height(22.0)
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(8.0);
                // Active profile name
                if let Some(idx) = app.selected_profile {
                    if let Some(p) = app.profiles.get(idx) {
                        ui.label(RichText::new(&p.name).small().color(theme.text_muted));
                        ui.add(egui::Separator::default().vertical());
                    }
                }
                // Engine status indicator
                let (dot_color, status_str) = match app.status.as_str() {
                    "Running" => (
                        Color32::from_rgb(0x40, 0xcc, 0x70),
                        format!("● Running  {}kHz", app.audio_sample_rate / 1000),
                    ),
                    "Error" => (Color32::from_rgb(0xe0, 0x50, 0x50), "✖ Error".to_string()),
                    _       => (Color32::from_rgb(0x44, 0x44, 0x60), "○ Stopped".to_string()),
                };
                ui.label(RichText::new(status_str).small().color(dot_color));
                // Inline error message
                if let Some(ref err) = app.last_error {
                    ui.add(egui::Separator::default().vertical());
                    ui.label(
                        RichText::new(format!("⚠ {err}"))
                            .small()
                            .color(Color32::from_rgb(0xe0, 0x70, 0x50)),
                    );
                }
            });
        });
}
```

### `show_profile_panel`

```rust
fn show_profile_panel(app: &mut App, ctx: &Context) {
    let theme = app.theme.current();
    egui::SidePanel::left("profiles")
        .min_width(190.0)
        .max_width(260.0)
        .show(ctx, |ui| {
            section_header(ui, "PROFILES", theme.accent);

            egui::ScrollArea::vertical()
                .max_height(ui.available_height() - 70.0)   // reserve space for save form
                .show(ui, |ui| {
                    // Passthrough entry (index 0)
                    let selected = app.selected_profile == Some(0);
                    let resp = ui.selectable_label(
                        selected,
                        RichText::new("Passthrough")
                            .color(if selected { Color32::WHITE } else { theme.text }),
                    );
                    if selected {
                        // Accent bar on left edge of selected item
                        ui.painter().rect_filled(
                            egui::Rect::from_min_size(
                                resp.rect.left_top(), egui::vec2(3.0, resp.rect.height())
                            ),
                            0.0, theme.accent,
                        );
                    }
                    if resp.clicked() && !selected { app.select_profile(0); }

                    // Built-in profiles in collapsible groups
                    // PROFILE_GROUPS: &[(&str label, usize start, usize end)]
                    for (group_idx, &(group_name, start, end)) in PROFILE_GROUPS.iter().enumerate() {
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
                                let resp = ui.selectable_label(selected,
                                    RichText::new(format!("  {name}"))
                                        .color(if selected { Color32::WHITE } else { theme.text }),
                                );
                                if selected {
                                    ui.painter().rect_filled(
                                        egui::Rect::from_min_size(
                                            resp.rect.left_top(), egui::vec2(3.0, resp.rect.height())
                                        ),
                                        0.0, theme.accent,
                                    );
                                }
                                if resp.clicked() && !selected { app.select_profile(i); }
                            }
                        }
                    }

                    // User profiles with delete (×) and update (↑) buttons
                    let user_count = app.profiles.len() - app.built_in_count;
                    if user_count > 0 {
                        ui.label(RichText::new("MY PROFILES").color(theme.text_muted).small().strong());
                        let mut to_delete: Option<usize> = None;
                        let mut to_update: Option<usize> = None;
                        for rel in 0..user_count {
                            let abs = app.built_in_count + rel;
                            let selected = app.selected_profile == Some(abs);
                            let is_dirty = selected && app.live_effects != app.profiles[abs].effects;
                            ui.horizontal(|ui| {
                                let resp = ui.selectable_label(selected,
                                    RichText::new(format!("  {}", app.profiles[abs].name.clone()))
                                        .color(if selected { Color32::WHITE } else { theme.text }),
                                );
                                if resp.clicked() && !selected { app.select_profile(abs); }
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    if ui.add(egui::Button::new(
                                        RichText::new("×").color(Color32::from_rgb(0xcc, 0x44, 0x44))
                                    ).frame(false)).clicked() { to_delete = Some(rel); }
                                    if is_dirty && ui.add(egui::Button::new(
                                        RichText::new("↑").color(Color32::from_rgb(0x44, 0xaa, 0x55))
                                    ).frame(false)).on_hover_text("Save changes to this profile")
                                    .clicked() { to_update = Some(rel); }
                                });
                            });
                        }
                        // Defer mutations to avoid mid-iteration borrow conflict
                        if let Some(rel) = to_delete { app.delete_user_profile(rel); }
                        if let Some(rel) = to_update { app.update_user_profile(rel); }
                    }
                });

            // Save-as form below the scroll area
            ui.separator();
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
                ).clicked() { app.save_current_as_profile(); }
            });
        });
}
```

### `show_device_panel`

```rust
fn show_device_panel(app: &mut App, ctx: &Context) {
    let theme   = app.theme.current();
    let running = app.engine.is_some();

    egui::SidePanel::right("devices")
        .min_width(240.0)
        .max_width(310.0)
        .show(ctx, |ui| {
            section_header(ui, "DEVICES", theme.accent);

            // Snapshot device state before rendering to detect changes
            let snap_input           = app.selected_input.clone();
            let snap_monitor_enabled = app.monitor_enabled;
            let snap_monitor         = app.selected_monitor.clone();
            let snap_virtual_enabled = app.virtual_enabled;
            let snap_virtual         = app.selected_virtual.clone();

            // Input device
            ui.label(RichText::new("INPUT").color(theme.text_muted).small().strong());
            egui::ComboBox::from_id_salt("input_dev")
                .selected_text(&app.selected_input)
                .width(210.0)
                .show_ui(ui, |ui| {
                    for name in app.input_devices.clone() {
                        ui.selectable_value(&mut app.selected_input, name.clone(), &name);
                    }
                });

            // Monitor device (with enable checkbox)
            ui.label(RichText::new("MONITOR").color(theme.text_muted).small().strong());
            ui.horizontal(|ui| {
                ui.checkbox(&mut app.monitor_enabled, "");
                ui.add_enabled_ui(app.monitor_enabled, |ui| {
                    egui::ComboBox::from_id_salt("monitor_dev")
                        .selected_text(&app.selected_monitor)
                        .width(185.0)
                        .show_ui(ui, |ui| {
                            for name in app.output_devices.clone() {
                                ui.selectable_value(&mut app.selected_monitor, name.clone(), &name);
                            }
                        });
                });
            });

            // Virtual output device (with enable checkbox)
            ui.label(RichText::new("VIRTUAL OUTPUT").color(theme.text_muted).small().strong());
            ui.horizontal(|ui| {
                ui.checkbox(&mut app.virtual_enabled, "");
                ui.add_enabled_ui(app.virtual_enabled, |ui| {
                    egui::ComboBox::from_id_salt("virtual_dev")
                        .selected_text(if app.selected_virtual.is_empty() {
                            "— select —"
                        } else {
                            &app.selected_virtual
                        })
                        .width(185.0)
                        .show_ui(ui, |ui| {
                            for name in app.output_devices.clone() {
                                ui.selectable_value(&mut app.selected_virtual, name.clone(), &name);
                            }
                        });
                });
            });

            // Restart engine automatically if a device selection changed while running
            let device_changed = app.selected_input   != snap_input
                || app.monitor_enabled != snap_monitor_enabled
                || app.selected_monitor != snap_monitor
                || app.virtual_enabled  != snap_virtual_enabled
                || app.selected_virtual != snap_virtual;
            if device_changed {
                app.settings_dirty = true;
                if app.engine.is_some() {
                    app.stop_engine();
                    app.start_engine();
                }
            }

            // Mic volume slider with reset button
            let mut gain = f32::from_bits(app.input_gain.load(Ordering::Relaxed));
            let prev_gain = gain;
            ui.horizontal(|ui| {
                ui.label(RichText::new("MIC VOLUME").color(theme.text_muted).small().strong());
                if gain != 1.0 && ui.add(
                    egui::Button::new(RichText::new("↺").small().color(theme.text_muted)).frame(false)
                ).on_hover_text("Reset to 1.00×").clicked() { gain = 1.0; }
            });
            ui.add(egui::Slider::new(&mut gain, 0.0f32..=4.0f32)
                .text("×").fixed_decimals(2).clamping(egui::SliderClamping::Always));
            if gain != prev_gain {
                app.input_gain.store(gain.to_bits(), Ordering::Relaxed);
                app.settings_dirty = true;
            }

            // Level meter
            ui.label(RichText::new("INPUT LEVEL").color(theme.text_muted).small().strong());
            level_meter(ui, app.input_level.get(), 210.0, theme.accent, theme.slider_track);

            // Start / Stop button
            let (btn_label, btn_color) = if running {
                ("■  STOP",  Color32::from_rgb(0xaa, 0x28, 0x28))
            } else {
                ("▶  START", Color32::from_rgb(0x1e, 0x80, 0x3e))
            };
            let btn_resp = ui.add(
                egui::Button::new(
                    RichText::new(btn_label).strong().size(15.0).color(Color32::WHITE)
                )
                .fill(btn_color)
                .min_size(egui::vec2(210.0, 44.0)),
            );
            if btn_resp.clicked() {
                if running { app.stop_engine(); } else { app.start_engine(); }
            }
            // Animated glow ring using sin pulse
            if running {
                let t     = ctx.input(|i| i.time) as f32;
                let pulse = (t * std::f32::consts::PI * 1.5).sin() * 0.5 + 0.5;
                let alpha = (45.0 + pulse * 110.0) as u8;
                ui.painter().rect_stroke(
                    btn_resp.rect.expand(3.0),
                    6.0,
                    egui::Stroke::new(2.0, Color32::from_rgba_unmultiplied(
                        theme.accent.r(), theme.accent.g(), theme.accent.b(), alpha,
                    )),
                );
            }

            ui.checkbox(&mut app.auto_start, "Start automatically on launch");
        });
}
```

### `show_effect_panel`

```rust
fn show_effect_panel(app: &mut App, ctx: &Context) {
    let theme  = app.theme.current();
    let accent = theme.accent;
    let active = app.engine.is_some();

    egui::CentralPanel::default().show(ctx, |ui| {
        // Spectrum visualizer — takes upper 46% of available height
        let spec_height = (ui.available_height() * 0.46).clamp(180.0, 320.0);
        ui.allocate_ui(egui::vec2(ui.available_width(), spec_height), |ui| {
            app.spectrum_panel.show(ui, accent, active);
        });

        // Effect chain header with optional Reset button
        ui.horizontal(|ui| {
            ui.label(RichText::new("EFFECT CHAIN").strong().small().color(theme.accent));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some(idx) = app.selected_profile {
                    let is_dirty = app.live_effects != app.profiles[idx].effects;
                    if is_dirty && ui.add(
                        egui::Button::new(RichText::new("Reset").color(Color32::WHITE).small())
                            .fill(Color32::from_rgb(0x88, 0x44, 0x22))
                    ).on_hover_text("Discard changes and reload profile defaults").clicked() {
                        app.reset_profile();
                    }
                }
            });
        });

        if app.live_effects.is_empty() {
            ui.centered_and_justified(|ui| {
                let msg = if app.selected_profile == Some(0) {
                    "Passthrough — audio is unmodified"
                } else { "No effects in this profile" };
                ui.label(RichText::new(msg).color(theme.text_muted));
            });
            return;
        }

        let mut chain_dirty = false;
        egui::ScrollArea::vertical().show(ui, |ui| {
            for idx in 0..app.live_effects.len() {
                let enabled     = app.live_effects[idx].enabled;
                let effect_type = app.live_effects[idx].effect_type;
                let label       = effect_display_name(effect_type);
                let summary     = effect_summary(effect_type, &app.live_effects[idx].params);

                // Card fill: subtle accent tint when enabled
                let card_fill = if enabled {
                    Color32::from_rgba_unmultiplied(accent.r(), accent.g(), accent.b(), 9)
                } else {
                    Color32::from_rgb(0x0f, 0x0f, 0x15)
                };

                egui::Frame::none()
                    .fill(card_fill)
                    .rounding(egui::Rounding::same(4.0))
                    .inner_margin(egui::Margin::same(4.0))
                    .show(ui, |ui| {
                        let cfg = &mut app.live_effects[idx];
                        let id  = egui::Id::new(("fx", idx));
                        egui::collapsing_header::CollapsingState::load_with_default_open(
                            ctx, id, false
                        )
                        .show_header(ui, |ui| {
                            // LED dot: outer glow + inner circle
                            let (led_r, _) = ui.allocate_exact_size(
                                egui::vec2(12.0, 12.0), egui::Sense::hover()
                            );
                            if cfg.enabled {
                                ui.painter().circle_filled(
                                    led_r.center(), 6.5,
                                    Color32::from_rgba_unmultiplied(
                                        accent.r(), accent.g(), accent.b(), 38
                                    ),
                                );
                            }
                            ui.painter().circle_filled(
                                led_r.center(), 4.0,
                                if cfg.enabled { accent } else { Color32::from_rgb(0x26, 0x26, 0x30) },
                            );

                            let prev     = cfg.enabled;
                            let text_col = if prev { theme.text } else { theme.text_muted };
                            ui.checkbox(&mut cfg.enabled,
                                RichText::new(label).strong().color(text_col));
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
                    });
                ui.add_space(2.0);
            }
        });

        if chain_dirty { app.apply_chain(); }
    });
}
```

---

## Shared Rendering Helpers

### `section_header` — labeled divider line

```rust
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
```

### `level_meter` — segmented VU bar

```rust
fn level_meter(ui: &mut Ui, level: f32, width: f32, accent: Color32, track: Color32) {
    let height  = 14.0;
    let n_segs  = 22usize;
    let gap     = 1.5f32;
    let seg_w   = (width - gap * (n_segs - 1) as f32) / n_segs as f32;
    let (rect, _) = ui.allocate_exact_size(egui::vec2(width, height), egui::Sense::hover());
    let fill_count = (level.min(1.05) * n_segs as f32) as usize;
    for i in 0..n_segs {
        let x   = rect.left() + i as f32 * (seg_w + gap);
        let seg = egui::Rect::from_min_size(Pos2::new(x, rect.top()), egui::vec2(seg_w, height));
        let color = if i < fill_count.min(n_segs) {
            let norm = (i + 1) as f32 / n_segs as f32;
            vu_color(norm, accent)
        } else { track };
        ui.painter().rect_filled(seg, 2.0, color);
    }
    // Clip indicator (hard red) when level > 1.0
    if level > 1.0 {
        let clip = egui::Rect::from_min_size(
            Pos2::new(rect.right() - seg_w, rect.top()),
            egui::vec2(seg_w, height),
        );
        ui.painter().rect_filled(clip, 2.0, Color32::from_rgb(0xff, 0x20, 0x20));
    }
}
```

### Color helpers

```rust
fn vu_color(norm: f32, accent: Color32) -> Color32 {
    // dim → accent (0..0.65) → yellow (0.65..0.82) → red (0.82..1.0)
    let yellow = Color32::from_rgb(0xe8, 0xb8, 0x00);
    let red    = Color32::from_rgb(0xe8, 0x30, 0x30);
    let base = if norm < 0.65 {
        let dim = Color32::from_rgba_unmultiplied(accent.r()/3, accent.g()/3, accent.b()/3, 100);
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
```

---

## Effect Metadata Helpers

### `effect_display_name`

Maps `EffectType` enum to human-readable strings:

| EffectType | Display |
|---|---|
| `Gain` | "Gain" |
| `CleanMic` | "Clean Mic" |
| `NoiseSuppression` | "Noise Suppression" |
| `PitchShift` | "Pitch Shift" |
| `BandpassFilter` | "Bandpass Filter" |
| `Compressor` | "Compressor" |
| `DeEsser` | "De-Esser" |
| `Echo` | "Echo" |
| `Reverb` | "Reverb" |
| `Chorus` | "Chorus" |
| `Flanger` | "Flanger" |
| `Tremolo` | "Tremolo" |
| `Vibrato` | "Vibrato" |
| `Distortion` | "Distortion" |
| `LoFi` | "Lo-Fi" |
| `RingMod` | "Ring Mod" |
| `Robot` | "Robot" |

### `effect_summary`

One-line param summary for collapsed effect headers. Pattern: `format!("{key:.precision}{suffix}", ...)`.

```rust
fn effect_summary(t: EffectType, params: &HashMap<String, f64>) -> String {
    let p = |key: &str, default: f64| -> f64 { *params.get(key).unwrap_or(&default) };
    match t {
        EffectType::Gain             => format!("{:.2}×", p("gain", 1.0)),
        EffectType::PitchShift       => { let st = p("semitones", 0.0);
                                          if st >= 0.0 { format!("+{st:.1} st") } else { format!("{st:.1} st") } }
        EffectType::BandpassFilter   => format!("{:.0} Hz", p("center_freq", 2000.0)),
        EffectType::Compressor       => format!("{:.0}:{:.0}  thr {:.2}", 1, p("ratio",4.0) as u32, p("threshold",0.5)),
        EffectType::Echo             => format!("{:.2}s  fb {:.2}", p("delay_secs",0.3), p("feedback",0.3)),
        EffectType::Reverb           => format!("room {:.2}  wet {:.2}", p("room_size",0.5), p("wet",0.3)),
        EffectType::Chorus
        | EffectType::Flanger        => format!("{:.1} Hz  wet {:.2}", p("rate",1.5), p("wet",0.5)),
        EffectType::Tremolo
        | EffectType::Vibrato        => format!("{:.1} Hz", p("rate",5.0)),
        EffectType::Distortion       => format!("drive {:.1}", p("drive",2.0)),
        EffectType::LoFi             => format!("{:.0}-bit", p("bit_depth",8.0)),
        EffectType::RingMod          => format!("{:.0} Hz", p("carrier_freq",200.0)),
        EffectType::Robot            => format!("{:.0} Hz", p("pitch_hz",100.0)),
        EffectType::CleanMic         => format!("{:.0} Hz", p("cutoff_hz",20.0)),
        EffectType::NoiseSuppression => { let thr = p("threshold", 0.0);
                                          if thr > 0.001 { format!("{:.0}%  gate {:.0}%", p("strength",1.0)*100.0, thr*100.0) }
                                          else { format!("{:.0}%", p("strength",1.0)*100.0) } }
        EffectType::DeEsser          => format!("thr {:.2}", p("threshold",0.3)),
    }
}
```

### `effect_params_ui`

Renders sliders for each effect type. Uses a local macro to reduce repetition:

```rust
macro_rules! slider {
    ($label:expr, $key:expr, $lo:expr, $hi:expr, $default:expr) => {{
        let mut v = *params.entry($key.into()).or_insert($default) as f32;
        if ui.add(egui::Slider::new(&mut v, ($lo as f32)..=($hi as f32)).text($label)).changed() {
            params.insert($key.into(), v as f64);
            changed = true;
        }
    }};
}
```

Parameter ranges by effect:

| Effect | Parameters (key, range, default) |
|---|---|
| CleanMic | cutoff_hz [20–400, 20] |
| NoiseSuppression | strength [0–1, 1.0], threshold [0–1, 0.0] |
| Gain | gain [0–4, 1.0] |
| PitchShift | semitones [−12–12, 0] |
| BandpassFilter | center_freq [200–8000, 2000], q [0.1–10, 1.0] |
| Compressor | threshold [0.05–1, 0.5], ratio [1–20, 4], attack [0.001–0.1, 0.005], release [0.01–1, 0.1] |
| DeEsser | threshold [0.05–1, 0.3], reduction [0–1, 0.3] |
| Echo | delay_secs [0.05–2, 0.3], feedback [0–0.95, 0.3], wet [0–1, 0.5] |
| Reverb | room_size [0–1, 0.5], wet [0–1, 0.3] |
| Chorus | rate [0.1–8, 1.5], depth [0.001–0.02, 0.003], wet [0–1, 0.5] |
| Flanger | rate [0.1–5, 0.5], depth [0.001–0.01, 0.005], feedback [0–0.95, 0.5], wet [0–1, 0.5] |
| Tremolo | rate [0.5–20, 5.0], depth [0–1, 0.7] |
| Vibrato | rate [0.5–20, 5.0], depth [0.001–0.01, 0.003] |
| Distortion | drive [1–10, 2.0], hard_clip [0–1, 0.0] |
| LoFi | bit_depth [4–16, 8.0], downsample [1–8, 2.0] |
| RingMod | carrier_freq [20–2000, 200] |
| Robot | pitch_hz [50–500, 100] |

---

## Patterns Worth Noting

**Deferred mutations during iteration** — When iterating over a list and a button may delete or update an entry, collect the index into an `Option` and apply after the loop:
```rust
let mut to_delete: Option<usize> = None;
for i in 0..items.len() {
    if ui.button("×").clicked() { to_delete = Some(i); }
}
if let Some(i) = to_delete { items.remove(i); }
```

**Snapshot-then-compare for change detection** — Clone device selections before rendering; compare after to detect any change without needing per-widget callbacks:
```rust
let snap = app.selected_input.clone();
// ... render combo box mutating app.selected_input ...
if app.selected_input != snap { /* handle change */ }
```

**Atomic f32 for audio thread sharing** — Store gain as `f32::to_bits()` in an `Arc<AtomicU32>` so the audio thread reads it lock-free:
```rust
// Write (UI thread):
app.input_gain.store(gain.to_bits(), Ordering::Relaxed);
// Read (audio thread):
let gain = f32::from_bits(self.input_gain.load(Ordering::Relaxed));
```

**Animated glow using `ctx.input(|i| i.time)`** — Drive a sin-wave alpha pulse off the frame timestamp:
```rust
let t     = ctx.input(|i| i.time) as f32;
let pulse = (t * PI * 1.5).sin() * 0.5 + 0.5;
let alpha = (45.0 + pulse * 110.0) as u8;
```
