# BS-VChanger-Rust — Build Plan

## What We're Building
A Windows-native real-time voice changer in Rust. Microphone audio is captured via WASAPI, passed through a chain of DSP effects, and routed to a monitor output (speakers/headphones) and/or a virtual audio device (VB-CABLE) for use in Discord, OBS, etc. The UI is built with egui and ships as a single EXE.

This is a clean-slate rewrite. No code, profiles, or settings are shared with the C# version.

---

## Tech Stack

| Layer | Crate | Purpose |
|---|---|---|
| GUI | `eframe`, `egui` | Immediate-mode native window |
| Audio I/O | `cpal` | WASAPI capture + playback on Windows |
| Ring buffer | `ringbuf` | Lock-free audio thread ↔ UI thread bridge |
| Pitch shifting | `rubato` | Asynchronous resampling for pitch effects |
| Noise suppression | `nnnoiseless` | RNNoise port |
| FFT (visualizer) | `rustfft` | Spectrum analyzer |
| Serialization | `serde`, `serde_json` | Profile and settings persistence |
| Thread safety | `parking_lot` | Mutexes for shared audio state |
| Native dialogs | `rfd` | Error popups, file pickers |
| App icon | `winres` (build dep) | Embed `.ico` into EXE |

---

## Module Structure

```
src/
  main.rs                    # eframe entry point, no-console subsystem
  app.rs                     # Top-level App struct (egui::App impl), wires all subsystems
  theme.rs                   # AppTheme, ThemeManager, Dark + Neon palette definitions

  audio/
    mod.rs
    engine.rs                # RealtimeAudioEngine — owns cpal streams, effect chain
    devices.rs               # Device enumeration helpers
    ring_buffer.rs           # Thread-safe f32 sample ring buffer
    spectrum.rs              # SpectrumBuffer — feeds FFT data to UI thread

    effects/
      mod.rs
      trait.rs               # AudioEffect trait: process(&mut self, &mut [f32], u32)
      chain.rs               # EffectChain — ordered Vec<Box<dyn AudioEffect>>
      gain.rs
      clean_mic.rs
      bandpass.rs
      compressor.rs
      de_esser.rs
      noise_suppression.rs   # nnnoiseless integration
      pitch.rs               # rubato resampling
      echo.rs
      reverb.rs              # Schroeder reverb (comb + allpass)
      chorus.rs
      flanger.rs
      tremolo.rs
      vibrato.rs
      distortion.rs
      lofi.rs
      ring_mod.rs
      robot.rs

  profiles/
    mod.rs
    profile.rs               # VoiceProfile, EffectConfig structs (serde)
    effect_types.rs          # EffectType enum
    built_in.rs              # 24 built-in profiles
    repository.rs            # Load/save profiles.json to %APPDATA%

  settings/
    mod.rs
    app_settings.rs          # AppSettings struct (devices, theme, last profile)
    repository.rs            # Load/save settings.json to %APPDATA%

  ui/
    mod.rs
    main_window.rs           # Top-level layout orchestrator
    device_panel.rs          # Input/output device dropdowns + Start/Stop
    profile_panel.rs         # Profile list, select, save, delete
    effect_panel.rs          # Effect chain editor — toggles, sliders per effect
    spectrum_panel.rs        # Real-time FFT bar chart (egui custom painter)
    about_dialog.rs          # Modal with version info

build.rs                     # winres — embed app icon into EXE
assets/
  icon.ico                   # Application icon
```

---

## Stage 1 — Project Scaffold
**Goal:** `cargo run` opens a blank dark window with correct structure and all dependencies declared.

### Tasks
- [ ] `Cargo.toml` with all crate dependencies and Windows-specific features
- [ ] `build.rs` with `winres` for icon embedding
- [ ] Module tree created (empty `mod.rs` stubs)
- [ ] `main.rs` with `#![windows_subsystem = "windows"]` and eframe launch
- [ ] Minimal `App` struct renders window title "BS-VChanger" with dark egui visuals

### Done When
`cargo run` opens a dark window without errors or console.

---

## Stage 2 — Audio I/O Engine
**Goal:** Mic audio captured and passed through to output device in real time (passthrough, no effects yet).

### Tasks
- [ ] `devices.rs` — enumerate all WASAPI input and output devices, return `Vec<(String, DeviceId)>`
- [ ] `RealtimeAudioEngine` with `start()` / `stop()` methods
- [ ] Lock-free ring buffer bridging input cpal callback → output cpal callback
- [ ] Two output targets: **monitor** (speakers) and **virtual** (VB-CABLE), both optional
- [ ] Handle device errors gracefully (stream error callback → UI notification)
- [ ] Sample rate negotiation between input and output devices

### Done When
Select mic + output, click Start, hear mic passthrough. No crackling or dropout at default buffer size.

---

## Stage 3 — Effect Trait & All 16 Effects
**Goal:** Complete effect library, each independently testable, composable into a chain.

### `AudioEffect` Trait
```rust
pub trait AudioEffect: Send {
    fn process(&mut self, samples: &mut [f32], sample_rate: u32);
    fn name(&self) -> &'static str;
    fn reset(&mut self);
}
```

### Effects to Implement
| Effect | Approach |
|---|---|
| `GainEffect` | Amplitude scale `f32` multiplier |
| `CleanMicEffect` | Single-pole high-pass + normalize |
| `BandpassFilterEffect` | Biquad bandpass (RBJ cookbook) |
| `CompressorEffect` | Peak envelope + gain reduction |
| `DeEsserEffect` | Bandpass detect (4–8 kHz) + gain duck |
| `NoiseSuppressionEffect` | `nnnoiseless` RNNoise, 48 kHz frame |
| `PitchResampleEffect` | `rubato` resample + ring buffer |
| `EchoEffect` | Delay line with feedback coefficient |
| `ReverbEffect` | 4 comb + 2 allpass (Schroeder) |
| `ChorusEffect` | LFO-modulated delay line + wet mix |
| `FlangerEffect` | Short LFO delay (1–10 ms) |
| `TremoloEffect` | Amplitude × sin LFO |
| `VibratoEffect` | Pitch modulation via short delay |
| `DistortionEffect` | Soft clip `tanh`, hard clip option |
| `LoFiEffect` | Bit depth reduce + downsample |
| `RingModEffect` | Multiply by sin carrier |
| `RobotModulationEffect` | Full-wave rectify + comb pitch |

### Tasks
- [ ] `trait.rs` — define trait + a `Box<dyn AudioEffect>` helper
- [ ] `chain.rs` — `EffectChain::process()` iterates enabled effects
- [ ] Implement all 16 effects (each in its own file)
- [ ] Unit tests for: Gain, Echo, Bandpass, Pitch, Compressor

### Done When
`EffectChain` can be constructed with arbitrary effects and applied to a `&mut [f32]` buffer. All unit tests pass.

---

## Stage 4 — Profiles & Settings
**Goal:** App restores last state on relaunch; built-in profiles load; user profiles persist.

### Data Shapes
```rust
// profiles.json entry
pub struct VoiceProfile {
    pub id: Uuid,
    pub name: String,
    pub effects: Vec<EffectConfig>,
    pub built_in: bool,
}

pub struct EffectConfig {
    pub effect_type: EffectType,
    pub enabled: bool,
    pub params: HashMap<String, f64>,
}

// settings.json
pub struct AppSettings {
    pub input_device_name: Option<String>,
    pub monitor_device_name: Option<String>,
    pub virtual_device_name: Option<String>,
    pub last_profile_id: Option<Uuid>,
    pub theme: ThemeChoice,   // Dark | Neon
}
```

### Tasks
- [ ] `EffectType` enum (16 variants) with serde rename
- [ ] `VoiceProfile` + `EffectConfig` with serde derive
- [ ] `ProfileRepository` — load/save `%APPDATA%\BS-VChanger-Rust\profiles.json`
- [ ] Seed with 24 built-in profiles (hard-coded, not editable)
- [ ] `AppSettings` with serde defaults
- [ ] `SettingsRepository` — load/save `%APPDATA%\BS-VChanger-Rust\settings.json`
- [ ] Both repos create their directory on first run if it doesn't exist

### Done When
After changing device or profile and restarting, the app returns to the same state. User can save a custom profile and it survives restart. Built-in profiles cannot be deleted.

---

## Stage 5 — Core UI
**Goal:** Fully wired, usable application — all 24 profiles selectable, effects audible, devices switchable.

### Layout (three-column)
```
┌─────────────────────────────────────────────────────────┐
│  BS-VChanger          [Dark | Neon]        [About]      │
├──────────────┬──────────────────────┬───────────────────┤
│ PROFILES     │ EFFECT CHAIN         │ DEVICES           │
│              │                      │                   │
│ [Built-in]   │ ┌──────────────────┐ │ Input:  [------▼] │
│  • Studio    │ │ ✓ Noise Suppress │ │ Monitor:[------▼] │
│  • Robot     │ │ ✓ Pitch   -5 st  │ │ Virtual:[------▼] │
│  • Chipmunk  │ │ ✗ Reverb         │ │                   │
│  • ...       │ │ ✓ Gain    0.8    │ │  [ ▶ START ]      │
│              │ └──────────────────┘ │  Status: Running  │
│ [User]       │  [+ Add Effect]      │                   │
│  • My Voice  │                      │                   │
│  [+ New]     │                      │                   │
└──────────────┴──────────────────────┴───────────────────┘
│ SPECTRUM VISUALIZER                                     │
└─────────────────────────────────────────────────────────┘
```

### Tasks
- [ ] `main_window.rs` — three-column layout using `egui::SidePanel` + `egui::CentralPanel`
- [ ] `device_panel.rs` — three `egui::ComboBox` dropdowns, Start/Stop button, status label
- [ ] `profile_panel.rs` — scrollable list, click to select, Save/Delete for user profiles
- [ ] `effect_panel.rs` — per-effect row: checkbox (enable), name, collapsible param sliders
- [ ] Wire profile selection → `EffectChain` hot-swap (atomic replace, no audio dropout)
- [ ] Wire Start/Stop → `RealtimeAudioEngine`
- [ ] Wire device selection → engine restart

### Done When
Select "Robot" profile → Start → hear robot voice. Switch to "Chipmunk" → hear chipmunk without stopping. All 24 profiles audible.

---

## Stage 6 — Spectrum Visualizer
**Goal:** Real-time animated frequency spectrum in the bottom panel without impacting audio.

### Tasks
- [ ] `SpectrumBuffer` — secondary ring buffer tapped from engine output (not on the hot path)
- [ ] UI thread reads latest frame, runs `rustfft`, maps to 64 log-spaced frequency bins
- [ ] `spectrum_panel.rs` — egui `Painter` draws filled rectangles, colored by theme accent
- [ ] Smooth bar decay (lerp toward zero between frames)
- [ ] Disable gracefully when audio is stopped (flat line)

### Done When
Spectrum animates fluidly during audio. No glitching or added latency on the audio thread.

---

## Stage 7 — Themes
**Goal:** Two polished themes, persistent choice, instant switching.

### Dark Theme
- Background: `#1a1a1e`
- Panel: `#25252b`
- Accent: `#5b9cf6` (blue)
- Text: `#e0e0e0`
- Slider fill: `#3a6bc8`

### Neon Theme
- Background: `#0d0d14`
- Panel: `#14141f`
- Accent: `#00f5d4` (cyan) / `#f700ff` (magenta) alternating
- Text: `#ffffff`
- Slider fill: `#7b00ff`
- Panel border: accent color (glow illusion via colored outline)

### Tasks
- [ ] `AppTheme` struct with all color fields as `egui::Color32`
- [ ] `ThemeManager::apply(&theme, &mut egui::Style)` — overwrites egui visuals
- [ ] Theme toggle button in header bar
- [ ] Spectrum bars use accent color from active theme
- [ ] Persist `ThemeChoice` in `AppSettings`

### Done When
Toggle between Dark and Neon instantly repaints entire UI. Choice survives restart.

---

## Stage 8 — Polish & Release
**Goal:** Single-binary Windows EXE, clean install experience, ready to ship.

### Tasks
- [ ] `assets/icon.ico` — create or source a suitable icon
- [ ] `build.rs` — `winres::WindowsResource` embeds icon and version metadata
- [ ] `rfd` error dialogs for: device not found, audio stream failure
- [ ] Release profile in `Cargo.toml`:
  ```toml
  [profile.release]
  opt-level = 3
  lto = "thin"
  codegen-units = 1
  strip = true
  ```
- [ ] Verify EXE runs on clean Windows 11 machine (no VC++ redist, no .NET)
- [ ] `README.md` — install steps, VB-CABLE note, supported Windows versions
- [ ] Final smoke test: all 24 profiles audible, theme switch works, settings persist

### Done When
`cargo build --release` produces a single EXE under 20 MB. Runs on clean Windows without any runtime dependencies.

---

## Build Order Summary

```
Stage 1  Scaffold         → window opens
Stage 2  Audio I/O        → mic passthrough works
Stage 3  Effects          → DSP chain applied to audio
Stage 4  Profiles         → data persists, 24 built-ins
Stage 5  Core UI          → fully wired, usable app
Stage 6  Spectrum         → visualizer running
Stage 7  Themes           → Dark + Neon, persistent
Stage 8  Polish/Release   → single EXE, ship-ready
```

Each stage produces a working, runnable build. No stage is "scaffolding only" after Stage 1.
