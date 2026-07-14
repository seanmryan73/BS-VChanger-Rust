// Anti-aliased "sound wave" pictogram icon: five rounded-capsule bars in a
// symmetric mountain profile, used by the runtime window icon (src/main.rs)
// and the embedded .ico resource (build.rs, via `include!` — build scripts
// can't depend on the main crate, so this file stays free of egui imports).
//
// Unlike sibling apps' two-letter block-glyph icons, this one is rendered
// with 4x4 supersampled anti-aliasing so the rounded bar caps stay smooth at
// every requested size. See Reference/App-Icon-Standards.md § AA Pictogram
// Style for the shared rounded-rect/supersampling approach this implements.

const BG: [u8; 3] = [0x07, 0x07, 0x0B];
const FG: [u8; 3] = [0xFF, 0x4D, 0x8D]; // rose-pink
const BORDER: [u8; 3] = [0x4D, 0xFF, 0xBF]; // spring-green — 180° hue rotation of FG

struct Bar {
    cx: f64,
    cy: f64,
    half_w: f64,
    half_h: f64,
    r: f64,
}

/// Rounded-rect (stadium) coverage test: is point (x, y) inside?
fn inside_rounded_rect(x: f64, y: f64, b: &Bar) -> bool {
    let qx = ((x - b.cx).abs() - (b.half_w - b.r)).max(0.0);
    let qy = ((y - b.cy).abs() - (b.half_h - b.r)).max(0.0);
    (qx * qx + qy * qy).sqrt() <= b.r
}

/// Draws the 5-bar sound-wave pictogram plus the standard complementary-color
/// top/right border on a `size`x`size` canvas. Geometry is defined as
/// fractions of a 256-unit reference (matching the proportions approved in
/// preview) and scaled to any requested size; anti-aliasing comes from a 4x4
/// supersample per output pixel rather than nearest-neighbor block scaling.
pub fn draw_icon_rgba(size: u32) -> Vec<u8> {
    let size_f = size as f64;
    let scale = size_f / 256.0;

    let heights = [70.0, 110.0, 150.0, 110.0, 70.0];
    let bar_w = 22.0 * scale;
    let gap = 14.0 * scale;
    let baseline = 195.0 * scale;
    let total_w = heights.len() as f64 * bar_w + (heights.len() as f64 - 1.0) * gap;
    let x0 = (size_f - total_w) / 2.0;

    let bars: Vec<Bar> = heights
        .iter()
        .enumerate()
        .map(|(i, h)| {
            let h = h * scale;
            let cx = x0 + i as f64 * (bar_w + gap) + bar_w / 2.0;
            let half_h = h / 2.0;
            let cy = baseline - half_h;
            Bar {
                cx,
                cy,
                half_w: bar_w / 2.0,
                half_h,
                r: bar_w / 2.0,
            }
        })
        .collect();

    let size = size as usize;
    let mut rgba = vec![0u8; size * size * 4];
    const S: i32 = 4; // supersample factor

    for py in 0..size {
        for px in 0..size {
            let mut cover = 0u32;
            for sy in 0..S {
                for sx in 0..S {
                    let x = px as f64 + (sx as f64 + 0.5) / S as f64;
                    let y = py as f64 + (sy as f64 + 0.5) / S as f64;
                    if bars.iter().any(|b| inside_rounded_rect(x, y, b)) {
                        cover += 1;
                    }
                }
            }
            let t = cover as f64 / (S * S) as f64;
            let i = (py * size + px) * 4;
            for c in 0..3 {
                rgba[i + c] = (BG[c] as f64 * (1.0 - t) + FG[c] as f64 * t).round() as u8;
            }
            rgba[i + 3] = 0xFF;
        }
    }

    let thickness = (size / 16).max(1).min(size);
    for py in 0..thickness {
        for px in 0..size {
            let i = (py * size + px) * 4;
            rgba[i..i + 3].copy_from_slice(&BORDER);
        }
    }
    for py in 0..size {
        for px in (size - thickness)..size {
            let i = (py * size + px) * 4;
            rgba[i..i + 3].copy_from_slice(&BORDER);
        }
    }

    rgba
}
