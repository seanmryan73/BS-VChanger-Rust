// Shared pixel-art generator for the app icon: the runtime window icon
// (src/main.rs) and the embedded .ico resource (build.rs, via `include!` —
// build scripts can't depend on the main crate, so this file stays free of
// egui/crate-only imports).

const BG: [u8; 4] = [0x07, 0x07, 0x0B, 0xFF];
const FG: [u8; 4] = [0xFF, 0xE0, 0x00, 0xFF]; // golden yellow

const GLYPH_B: [[u8; 5]; 7] = [
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 1, 1, 1, 0],
];
const GLYPH_V: [[u8; 5]; 7] = [
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 0, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 1, 0, 0],
];

/// Draws the "BV" glyph pair on a `size`x`size` canvas, scaling the 32x32
/// reference layout (block scale 2, 4px gap, glyphs + gap centered
/// horizontally, vertically centered) up or down.
pub fn draw_icon_rgba(size: u32) -> Vec<u8> {
    let size = size as usize;
    let scale = size as f64 / 32.0;
    let mut rgba = vec![0u8; size * size * 4];
    for chunk in rgba.chunks_exact_mut(4) {
        chunk.copy_from_slice(&BG);
    }

    let block = ((2.0 * scale).round() as usize).max(1);
    let lw = 5 * block;
    let lh = 7 * block;
    let gap = ((4.0 * scale).round() as usize).max(1);
    let ox_b = size.saturating_sub(lw * 2 + gap) / 2;
    let ox_v = ox_b + lw + gap;
    let oy = size.saturating_sub(lh) / 2;

    for (glyph, x0) in [(&GLYPH_B, ox_b), (&GLYPH_V, ox_v)] {
        for (row, bits) in glyph.iter().enumerate() {
            for (col, &on) in bits.iter().enumerate() {
                if on == 0 {
                    continue;
                }
                for dy in 0..block {
                    for dx in 0..block {
                        let px = x0 + col * block + dx;
                        let py = oy + row * block + dy;
                        if px < size && py < size {
                            let i = (py * size + px) * 4;
                            rgba[i..i + 4].copy_from_slice(&FG);
                        }
                    }
                }
            }
        }
    }
    rgba
}
