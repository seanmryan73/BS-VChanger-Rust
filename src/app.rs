use std::sync::Arc;
use parking_lot::Mutex;
use eframe::egui::{self, Color32, Context, RichText};

use crate::audio::{
    devices,
    effects::EffectChain,
    engine::{RealtimeAudioEngine, StartConfig},
};
use crate::theme::{ThemeChoice, ThemeManager};

pub struct App {
    theme: ThemeManager,

    // Device lists (populated once at startup)
    input_devices:  Vec<String>,
    output_devices: Vec<String>,

    // User selections
    selected_input:   String,
    selected_monitor: String,
    selected_virtual: String,
    monitor_enabled:  bool,
    virtual_enabled:  bool,

    // Audio engine
    engine:       Option<RealtimeAudioEngine>,
    effect_chain: Arc<Mutex<EffectChain>>,

    // Status
    status:       String,
    last_error:   Option<String>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let theme = ThemeManager::new();
        theme.apply(&cc.egui_ctx);

        let input_devices  = devices::list_input_devices();
        let output_devices = devices::list_output_devices();

        let selected_input   = input_devices.first().cloned().unwrap_or_default();
        let selected_monitor = output_devices.first().cloned().unwrap_or_default();
        let selected_virtual = String::new();

        Self {
            theme,
            input_devices,
            output_devices,
            selected_input,
            selected_monitor,
            selected_virtual,
            monitor_enabled: true,
            virtual_enabled: false,
            engine: None,
            effect_chain: Arc::new(Mutex::new(EffectChain::default())),
            status: "Stopped".into(),
            last_error: None,
        }
    }

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

    fn show_header(&mut self, ctx: &Context) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("BS-VChanger");
                ui.add_space(8.0);
                ui.label(
                    RichText::new("Stage 2: Audio I/O")
                        .color(Color32::from_rgb(0x88, 0x88, 0x99))
                        .small(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let label = match self.theme.choice {
                        ThemeChoice::Dark => "Neon",
                        ThemeChoice::Neon => "Dark",
                    };
                    if ui.button(label).clicked() {
                        self.theme.toggle();
                        self.theme.apply(ctx);
                    }
                });
            });
        });
    }

    fn show_device_panel(&mut self, ui: &mut egui::Ui) {
        ui.group(|ui| {
            ui.label(RichText::new("DEVICES").strong());
            ui.add_space(6.0);

            // Input
            ui.horizontal(|ui| {
                ui.label("Input:");
                egui::ComboBox::from_id_salt("input_dev")
                    .selected_text(&self.selected_input)
                    .width(320.0)
                    .show_ui(ui, |ui| {
                        for name in &self.input_devices.clone() {
                            ui.selectable_value(
                                &mut self.selected_input,
                                name.clone(),
                                name,
                            );
                        }
                    });
            });

            // Monitor output
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.monitor_enabled, "Monitor:");
                ui.add_enabled_ui(self.monitor_enabled, |ui| {
                    egui::ComboBox::from_id_salt("monitor_dev")
                        .selected_text(&self.selected_monitor)
                        .width(295.0)
                        .show_ui(ui, |ui| {
                            for name in &self.output_devices.clone() {
                                ui.selectable_value(
                                    &mut self.selected_monitor,
                                    name.clone(),
                                    name,
                                );
                            }
                        });
                });
            });

            // Virtual (VB-CABLE) output
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.virtual_enabled, "Virtual:");
                ui.add_enabled_ui(self.virtual_enabled, |ui| {
                    egui::ComboBox::from_id_salt("virtual_dev")
                        .selected_text(if self.selected_virtual.is_empty() {
                            "— select —"
                        } else {
                            &self.selected_virtual
                        })
                        .width(295.0)
                        .show_ui(ui, |ui| {
                            for name in &self.output_devices.clone() {
                                ui.selectable_value(
                                    &mut self.selected_virtual,
                                    name.clone(),
                                    name,
                                );
                            }
                        });
                });
            });
        });
    }

    fn show_transport(&mut self, ui: &mut egui::Ui) {
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            let running = self.engine.is_some();

            let (btn_label, btn_color) = if running {
                ("■  Stop", Color32::from_rgb(0xe0, 0x50, 0x50))
            } else {
                ("▶  Start", Color32::from_rgb(0x50, 0xc0, 0x70))
            };

            if ui
                .add(
                    egui::Button::new(RichText::new(btn_label).strong())
                        .fill(btn_color)
                        .min_size(egui::vec2(120.0, 36.0)),
                )
                .clicked()
            {
                if running {
                    self.stop_engine();
                } else {
                    self.start_engine();
                }
            }

            ui.add_space(16.0);

            let (status_color, status_text) = match self.status.as_str() {
                "Running" => (Color32::from_rgb(0x50, 0xc0, 0x70), "● Running"),
                "Error"   => (Color32::from_rgb(0xe0, 0x50, 0x50), "✖ Error"),
                _         => (Color32::from_rgb(0x88, 0x88, 0x99), "○ Stopped"),
            };
            ui.label(RichText::new(status_text).color(status_color).strong());
        });

        if let Some(ref err) = self.last_error.clone() {
            ui.add_space(6.0);
            ui.colored_label(Color32::from_rgb(0xe0, 0x70, 0x50), format!("⚠ {}", err));
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.poll_engine_errors();

        self.show_header(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(24.0);
            self.show_device_panel(ui);
            self.show_transport(ui);

            ui.add_space(40.0);
            ui.separator();
            ui.add_space(12.0);
            ui.colored_label(
                Color32::from_rgb(0x55, 0x55, 0x66),
                "Effect chain and profile panel coming in Stage 5.",
            );
        });
    }
}
