# BS-VChanger-Rust — Agent Context

Real-time voice changer for Windows. Captures mic audio via WASAPI (`cpal`), processes it through a configurable chain of 17 DSP effects, and routes output to monitor and/or virtual (VB-CABLE) destinations. Primary use case: clean voice and noise reduction. Ships as a single ~5 MB standalone EXE, no installer.

> **Agent context.** Lines beginning with `@` are absolute file paths (one per line; paths may
> contain spaces). Claude Code loads them automatically. All other agents (Codex, Copilot, etc.): read
> every `@` file now, before doing any work. The *Read before you…* table is **not** loaded up front:
> open the named note when your task matches the row, before you change anything in that area.

## Always loaded

@c:\_repos\Obsidian\Notes\Claude\Reference\Author-Version-Standards.md
@c:\_repos\Obsidian\Notes\Claude\Reference\Core-Rules.md
@c:\_repos\Obsidian\Notes\Claude\Projects\BS-VChanger-Rust Claude Context.md

## Read before you…

| Before you… | Read (path under `c:\_repos\Obsidian\Notes\Claude\`) |
|---|---|
| change UI, layout, themes, settings persistence or app architecture | `Reference/Rust-Desktop-Standards.md` (the matching section) |
| call a Windows API, touch audio, capture or displays, or spawn a process | `Reference/Windows-Platform-Gotchas.md` (the matching section) |
| recommend an architecture, a dependency or a pattern | search `Reference/Cross-Project-Anti-Patterns.md` |

Until 2026-09-19 these standards were `@`-loaded in full at every session start. See
`Reference/Multi-Tool-AI-Setup.md` rule 6 for the two-tier loading and its budget.

## Pinned dependency versions — do not change without a concrete reason

| Crate | Pinned version | Why |
|-------|---------------|-----|
| egui / eframe | `0.29` | Intentionally pinned; upgrade all egui-family crates together or not at all |
| cpal | `0.15` | WASAPI audio I/O; pinned for stability |
| nnnoiseless | `0.5` | RNNoise, **pure Rust** — a port, not a binding. `default-features = false` (see below) |

## Critical constraints

- **egui/eframe pinned at 0.29** — do not apply 0.34 API patterns here.
- **Lock-free audio callback** — the `cpal` hot path uses only `ringbuf` and `try_lock()`. Never use blocking mutexes in the audio callback. If a lock can't be acquired, skip that batch.
- **RNNoise @ 48 kHz only** — `nnnoiseless` requires exactly 48 kHz; the effect is bypassed at other sample rates. Do not break this guard.
- **6-theme custom enum** — based on the `Rust-Desktop-Standards.md` theme standard; do not replace with egui's ThemeManager. Themes: CoralStorm, Shibui, Kasane, ColdSteel (default), Jizo, HotSteel.
- **No unsafe code** — `#![deny(unsafe_code)]` in `main.rs`.
- **`nnnoiseless` is pure Rust, not a C binding.** This file claimed the opposite until 2026-08-13,
  and the claim had propagated from `Rust-Desktop-Standards.md`'s C-exceptions table. Verified with
  `cargo tree -p nnnoiseless -e normal,build`: the deps are `anyhow`, `clap` and `dasp` — no `cc`, no
  `bindgen`, no C build script. MSVC is needed to link **any** Rust on Windows, not because of this
  crate. As written it would send someone installing Visual Studio Build Tools to fix an unrelated
  build failure.
- **`nnnoiseless` carries `default-features = false`.** Its default `bin` feature drags in
  **clap 3.2.25 → atty 0.2.14**, an EOL CLI stack with a known advisory, into a GUI app that never
  parses an argument. Without the flag the tree is `anyhow` + `clap` + `dasp`; with it, just
  `easyfft` + `once_cell`.
- **VB-CABLE is a runtime requirement** — virtual output routing requires VB-CABLE installed. App still runs in monitor-only mode without it.

## Working rules

- Follow Rust-Desktop-Standards.md unless the project note documents a deliberate exception.
- Prefer minimal, targeted edits.
- Effect chain rebuild: UI calls `apply_chain()` to rebuild from `live_effects` config — do not mutate the chain in place.

## After this session

Run **`/wrapup`** (`c:\_repos\Obsidian\Notes\Claude\Skills\wrapup\SKILL.md`). It is the procedure; this section only adds to it. This repo's project note: `Projects/BS-VChanger-Rust Claude Context.md`.
