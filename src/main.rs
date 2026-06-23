#![windows_subsystem = "windows"]
#![deny(unsafe_code)]
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

/// Generates a 32×32 RGBA icon with pixel-art "BV" in golden yellow on near-black.
fn build_icon() -> eframe::egui::IconData {
    const SZ: usize = 32;
    let mut px = vec![0u8; SZ * SZ * 4];

    for chunk in px.chunks_mut(4) {
        chunk[0] = 0x07;
        chunk[1] = 0x07;
        chunk[2] = 0x0b;
        chunk[3] = 0xff;
    }

    let fg: [u8; 4] = [0xff, 0xe0, 0x00, 0xff]; // golden yellow

    let b: [[u8; 5]; 7] = [
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 0],
    ];
    let v: [[u8; 5]; 7] = [
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [0, 1, 0, 1, 0],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
    ];

    let scale: usize = 2;
    let lw = 5 * scale;
    let lh = 7 * scale;
    let gap: usize = 4;
    let ox_b = (SZ - lw * 2 - gap) / 2;
    let ox_v = ox_b + lw + gap;
    let oy   = (SZ - lh) / 2;

    let mut stamp = |bitmap: &[[u8; 5]; 7], ox: usize| {
        for (row, cols) in bitmap.iter().enumerate() {
            for (col, &bit) in cols.iter().enumerate() {
                if bit == 0 { continue; }
                for sy in 0..scale {
                    for sx in 0..scale {
                        let x = ox + col * scale + sx;
                        let y = oy + row * scale + sy;
                        if x < SZ && y < SZ {
                            let i = (y * SZ + x) * 4;
                            px[i..i + 4].copy_from_slice(&fg);
                        }
                    }
                }
            }
        }
    };

    stamp(&b, ox_b);
    stamp(&v, ox_v);

    eframe::egui::IconData { rgba: px, width: SZ as u32, height: SZ as u32 }
}
