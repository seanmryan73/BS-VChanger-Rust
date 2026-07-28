# BS-VChanger-Rust — Agent Context

Real-time voice changer for Windows. Captures mic audio via WASAPI (`cpal`), processes it through a configurable chain of 17 DSP effects, and routes output to monitor and/or virtual (VB-CABLE) destinations. Primary use case: clean voice and noise reduction. Ships as a single ~5 MB standalone EXE, no installer.

> **Agent context — required reading.** Lines beginning with `@` below are absolute file paths (one complete path per line; paths may contain spaces). Claude Code auto-imports them. All other agents (Codex, Copilot, etc.): read every `@` file now, before doing any work.

## Shared reference notes

@c:\_repos\Obsidian\Notes\Claude\Reference\Author-Version-Standards.md
@c:\_repos\Obsidian\Notes\Claude\Reference\Rust-Desktop-Standards.md
@c:\_repos\Obsidian\Notes\Claude\Reference\Windows-Platform-Gotchas.md

## Project context

@c:\_repos\Obsidian\Notes\Claude\Projects\BS-VChanger-Rust Claude Context.md

## Pinned dependency versions — do not change without a concrete reason

| Crate | Pinned version | Why |
|-------|---------------|-----|
| egui / eframe | `0.29` | Intentionally pinned; upgrade all egui-family crates together or not at all |
| cpal | `0.15` | WASAPI audio I/O; pinned for stability |
| nnnoiseless | `0.5` | RNNoise C binding; requires MSVC build tools installed |

## Critical constraints

- **egui/eframe pinned at 0.29** — do not apply 0.34 API patterns here.
- **Lock-free audio callback** — the `cpal` hot path uses only `ringbuf` and `try_lock()`. Never use blocking mutexes in the audio callback. If a lock can't be acquired, skip that batch.
- **RNNoise @ 48 kHz only** — `nnnoiseless` requires exactly 48 kHz; the effect is bypassed at other sample rates. Do not break this guard.
- **5-theme custom enum** — matches the `Rust-Desktop-Standards.md` 5-theme standard; do not replace with egui's ThemeManager. Themes: CoralStorm, Shibui, Kasane (default), ColdSteel, Jizo.
- **No unsafe code** — `#![deny(unsafe_code)]` in `main.rs`.
- **`nnnoiseless` requires MSVC** — C compiler required; `cargo build` fails without Visual Studio Build Tools.
- **VB-CABLE is a runtime requirement** — virtual output routing requires VB-CABLE installed. App still runs in monitor-only mode without it.

## Working rules

- Follow Rust-Desktop-Standards.md unless the project note documents a deliberate exception.
- Prefer minimal, targeted edits.
- Effect chain rebuild: UI calls `apply_chain()` to rebuild from `live_effects` config — do not mutate the chain in place.

## After this session

When the session ends or the user says to wrap up, update the project context note:
`c:\_repos\Obsidian\Notes\Claude\Projects\BS-VChanger-Rust Claude Context.md`

Update these sections:
- **Current constraints** — add any new version pins, banned patterns, or architecture rules discovered
- **Fix history** — add bugs fixed with root cause (one line each: date · symptom · cause · fix)
- **Next actions** — replace with the current list
- **frontmatter `version:`** — set to today's date (YYYY.MM.DD)
