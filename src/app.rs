use eframe::egui::{self, Context};
use crate::theme::ThemeManager;

pub struct App {
    theme: ThemeManager,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let theme = ThemeManager::new();
        theme.apply(&cc.egui_ctx);
        Self { theme }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("BS-VChanger");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let label = match self.theme.choice {
                        crate::theme::ThemeChoice::Dark => "Neon",
                        crate::theme::ThemeChoice::Neon => "Dark",
                    };
                    if ui.button(label).clicked() {
                        self.theme.toggle();
                        self.theme.apply(ctx);
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(180.0);
                ui.label(egui::RichText::new("Stage 1: Scaffold").size(24.0));
                ui.add_space(8.0);
                ui.label("Window is running. Audio and UI coming in Stage 2–5.");
            });
        });
    }
}
