# BS-VChanger-Rust — Claude Context

## What this repo is

Real-time voice changer for Windows. Captures mic audio via WASAPI, processes it through a configurable chain of 17 DSP effects, and routes output to monitor (speakers) and/or virtual (VB-CABLE) destinations. Primary use case: clean voice and noise reduction for calls/streaming — not entertainment effects.

## Shared reference notes

@c:\_repos\Obsidian\Notes\Claude\Reference\Author-Version-Standards.md
@c:\_repos\Obsidian\Notes\Claude\Reference\Rust-Desktop-Standards.md

## Project context

@c:\_repos\Obsidian\Notes\Claude\Projects\BS-VChanger-Rust Claude Context.md

## Pinned dependency versions — do not change without a concrete reason

| Crate | Pinned version | Why |
|-------|---------------|-----|
| egui / eframe | `0.29` | Intentionally pinned; upgrade all egui-family crates together or not at all |
| nnnoiseless | current | Only C-backed crate — wraps Xiph RNNoise; no pure-Rust alternative at this noise quality |

## Key constraints

- **Audio thread is lock-free** — `ringbuf` only in the hot path; no `Mutex` allowed on the audio callback thread.
- **`nnnoiseless` (RNNoise)** is the only C-backed crate — acceptable; no pure-Rust alternative exists at this quality level.
- **Windows-only:** WASAPI + Direct3D/OpenGL via GPU driver.
- **App data:** All persistent data goes to `%APPDATA%\BS-VChanger-Rust\` — never the EXE directory.
- **19-theme system:** Existing multi-theme set (NeonVoid, CyberRift, AcidRain, CoralStorm, Lucky, etc.); do not replace with the standard 5-theme `ThemeManager`.
- **`#![deny(unsafe_code)]` is missing** — should be added; the app has zero unsafe blocks.

## Working rules

- Follow Rust-Desktop-Standards.md unless this repo documents a deliberate exception.
- Prefer minimal, targeted edits.
- Keep egui-family crate versions aligned when changing dependencies (all must be `0.29`).
- Do not introduce new dependencies without a concrete reason.

## After this session

When the session ends or the user says to wrap up, update the project context note:
`c:\_repos\Obsidian\Notes\Claude\Projects\BS-VChanger-Rust Claude Context.md`

Update these sections:
- **Current constraints** — add any new version pins, banned patterns, or architecture rules discovered
- **Fix history** — add bugs fixed with root cause (one line each: date · symptom · cause · fix)
- **Next actions** — replace with the current list
- **frontmatter `version:`** — set to today's date (YYYY.MM.DD)
