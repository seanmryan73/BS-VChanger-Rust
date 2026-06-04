#![windows_subsystem = "windows"]
#![allow(dead_code, unused_variables, unused_imports)]

mod app;
mod audio;
mod profiles;
mod settings;
mod theme;
mod ui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("BS-VChanger")
            .with_inner_size([1100.0, 700.0])
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "BS-VChanger",
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}
