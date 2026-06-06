# BS-VChanger

Real-time voice changer for Windows. Built with Rust + egui.

---

## Features

- **26 built-in voice profiles** across 4 categories: Clean Voice, Pitch & Character, Space & Room, Creative
- **17 DSP effects**: noise suppression, pitch shift, compressor, de-esser, reverb, echo, chorus, flanger, distortion, lo-fi, ring mod, robot, and more
- **21 colour themes**: BagPipes, planetary (Mercury → Pluto), mythological (Zeus → Dionysus)
- **Real-time spectrum analyser** with frequency labels and reflection effect
- **Dual output routing**: monitor (speakers/headphones) + virtual (VB-CABLE for Teams/Discord/OBS)
- **Custom profiles**: save, update, and delete your own presets
- **Single EXE**: no installer, no DLLs, no runtime dependencies

---

## Is it a standalone EXE?

**Yes.** The release build produces a single `bs-vchanger.exe` (~5 MB) with no installer
and no extra files. Copy the EXE anywhere and run it. It uses only Windows system APIs
(WASAPI for audio, Direct3D/OpenGL via the GPU driver for rendering) which are always
present on Windows 10/11.

---

## Quick Start

1. Select your **microphone** in the INPUT dropdown.
2. For calls: enable **VIRTUAL OUTPUT**, select "CABLE Input", then set "CABLE Output" as
   the mic in Teams/Discord/Zoom.
3. For personal monitoring: enable **MONITOR** and pick your headphones.
4. Pick a profile from the left panel.
5. Click **START**.

> **Warning:** Don't enable Monitor and Virtual at the same time when on a call.
> Teams/Zoom AEC will cancel the audio as echo. See the in-app Help for details.

---

## VB-CABLE (Virtual Output)

Install [VB-CABLE](https://vb-audio.com/Cable/) (free) to route processed audio into
Teams, Discord, Zoom, OBS, or any other app. Once installed:

- Set **VIRTUAL OUTPUT** → "CABLE Input" in BS-VChanger.
- Set the microphone → "CABLE Output" in your meeting/streaming app.

---

## Data Files

All persistent data is written to `%APPDATA%\BS-VChanger-Rust\`:

```
C:\Users\<you>\AppData\Roaming\BS-VChanger-Rust\
    settings.json        ← devices, theme, mic volume, last profile, auto-start
    user_profiles.json   ← your saved custom profiles
```

- The folder is created automatically on first launch.
- Deleting `settings.json` resets everything to defaults (or use "Reset to Defaults" in the Help dialog).
- Deleting `user_profiles.json` removes your saved profiles; built-in profiles are unaffected.
- The EXE writes nothing to its own directory.

---

## System Requirements

- Windows 10 or Windows 11 (64-bit)
- A microphone (any WASAPI-compatible device)
- GPU with Direct3D 11 or OpenGL 3.3 support (any modern GPU)
- [VB-CABLE](https://vb-audio.com/Cable/) for virtual mic routing (optional but recommended)

---

## Build Requirements

Install the Rust toolchain from [rustup.rs](https://rustup.rs/):

```powershell
rustc --version   # 1.75+
cargo --version
```

---

## Build & Run

```powershell
# Debug build (larger, includes debug info)
cargo build
cargo run

# Release build (optimised, stripped, ~5 MB)
cargo build --release
cargo run --release
```

---

## Executable Locations

| Build | Path |
|---|---|
| Debug | `target\debug\bs-vchanger.exe` |
| Release | `target\release\bs-vchanger.exe` |

---

## Distributing

1. `cargo build --release`
2. Copy `target\release\bs-vchanger.exe` to the recipient.
3. That's it — they also need [VB-CABLE](https://vb-audio.com/Cable/) for virtual mic routing.

---

## Built-in Profiles

| Category | Profiles |
|---|---|
| **Clean Voice** | BagPipe Clean, BagPipe Podcast, Clean Voice, Noise Reduction, Studio Voice, Conference Call, Podcast, Clarity Boost, Broadcast |
| **Pitch & Character** | Gentle Compress, Deep Voice, High Voice, Chipmunk, Giant, Telephone, Radio, Walkie-Talkie |
| **Space & Room** | Megaphone, Echo Chamber, Cathedral |
| **Creative** | Underwater, Robot, Alien, Daemon, Vintage |

---

## Tech Stack

| Layer | Crate | Purpose |
|---|---|---|
| GUI | `eframe` / `egui` | Immediate-mode native window |
| Audio I/O | `cpal` | WASAPI capture + playback |
| Ring buffer | `ringbuf` | Lock-free audio thread bridge |
| Pitch shifting | `rubato` | Asynchronous resampling |
| Noise suppression | `nnnoiseless` | RNNoise port |
| FFT | `rustfft` | Spectrum analyser |
| Serialisation | `serde` / `serde_json` | Profile and settings persistence |
