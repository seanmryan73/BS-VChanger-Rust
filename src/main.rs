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
            .with_inner_size([1280.0, 820.0])
            .with_min_inner_size([900.0, 560.0])
            .with_icon(bs_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "BS-VChanger",
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )
}

/// Generates a 32×32 RGBA icon with pixel-art "BS" on a dark background.
fn bs_icon() -> eframe::egui::IconData {
    const SZ: usize = 32;
    let mut px = vec![0u8; SZ * SZ * 4];

    // Background: dark navy
    for chunk in px.chunks_mut(4) {
        chunk[0] = 0x12;
        chunk[1] = 0x12;
        chunk[2] = 0x1e;
        chunk[3] = 0xff;
    }

    // Foreground: bright cyan (matches Neon accent, visible on both themes)
    let fg: [u8; 4] = [0x00, 0xe5, 0xc8, 0xff];

    // 5×7 pixel-art bitmaps
    let b: [[u8; 5]; 7] = [
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 0],
    ];
    let s: [[u8; 5]; 7] = [
        [0, 1, 1, 1, 1],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [0, 1, 1, 1, 0],
        [0, 0, 0, 0, 1],
        [0, 0, 0, 0, 1],
        [1, 1, 1, 1, 0],
    ];

    let scale: usize = 2;
    let lw = 5 * scale;
    let lh = 7 * scale;
    let gap: usize = 4;
    let ox_b = (SZ - lw * 2 - gap) / 2;
    let ox_s = ox_b + lw + gap;
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
    stamp(&s, ox_s);

    eframe::egui::IconData { rgba: px, width: SZ as u32, height: SZ as u32 }
}
