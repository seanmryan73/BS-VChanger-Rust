// Author  : Sean Ryan <seanmryan@gmail.com>
// Company : BagPipes
// Version : 2026.07.27

#![windows_subsystem = "windows"]
#![deny(unsafe_code)]
#![allow(dead_code, unused_variables, unused_imports)]

mod app;
mod audio;
mod icon_art;
mod profiles;
mod settings;
mod theme;
mod ui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("BS-VChanger")
            .with_inner_size([1280.0, 820.0])
            .with_min_inner_size([900.0, 560.0])
            .with_icon(build_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "BS-VChanger",
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}

/// Generates a 32×32 RGBA icon: an anti-aliased rose-pink sound-wave pictogram on near-black.
fn build_icon() -> eframe::egui::IconData {
    const SIZE: u32 = 32;
    eframe::egui::IconData { rgba: icon_art::draw_icon_rgba(SIZE), width: SIZE, height: SIZE }
}
