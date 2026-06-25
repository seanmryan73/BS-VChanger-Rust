# Copilot Instructions

This repository follows the shared Rust desktop conventions documented in the Obsidian vault.

## Shared reference

Obsidian vault note: `c:\_repos\Obsidian\Notes\Claude\Reference\Rust-Desktop-Standards.md`

Also: `c:\_repos\Obsidian\Notes\Claude\Reference\Author-Version-Standards.md`

## Working rules

- Prefer small, targeted edits over broad rewrites.
- Match the repository's existing style and architecture.
- Treat the shared Rust desktop standards as the default unless this repo has a documented exception.
- Keep egui-family crate versions aligned when changing dependencies.
- Preserve Windows desktop packaging assumptions unless the task explicitly changes them.
- Use focused validation after edits: narrow tests, `cargo check`, or a targeted build step.

## When changing config

- Keep release profile settings intentional.
- Keep theme defaults consistent with the documented standard unless the repo note says otherwise.
- Do not introduce new dependencies without a concrete reason.

## Project-specific overrides

- egui/eframe is pinned at **0.29** — do not upgrade without an explicit reason.
- Audio processing runs on a CPAL/WASAPI callback thread; keep audio code lock-free (ringbuf only, no Mutex in the hot path).
- `nnnoiseless` is the only C-backed crate; it wraps Xiph RNNoise — no pure-Rust alternative at equivalent quality.
- All persistence goes to `%APPDATA%\BS-VChanger-Rust\` — never write to the EXE directory.
- Effect chain is modular (`AudioEffect` trait in `src/effects/audio_effect.rs`); add new effects by implementing the trait and registering in `src/profiles/effect_types.rs` and `src/profiles/factory.rs`.
- Primary use case is clean voice and noise reduction — default profiles should prioritise that, not entertainment effects.
