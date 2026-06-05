# BS-VChanger-Rust

Real-time voice changer for Windows. Built with Rust + egui.

---

## Is it a standalone EXE?

**Yes.** The release build produces a single `bs-vchanger.exe` with no installer,
no runtime dependencies, and no extra DLLs to ship alongside it. Copy the EXE
anywhere and run it. It uses only Windows system APIs (WASAPI for audio, Direct3D/OpenGL
via the GPU driver for rendering) which are always present on Windows 10/11.

---

## Data Files

All persistent data is written to `%APPDATA%\BS-VChanger-Rust\` — typically:

```
C:\Users\<you>\AppData\Roaming\BS-VChanger-Rust\
    settings.json        ← device selection, theme, last profile, mic volume, etc.
    user_profiles.json   ← any profiles you save inside the app
```

- The folder is created automatically on first launch.
- Deleting `settings.json` resets everything to defaults (same as "Reset to Defaults" in the About dialog).
- Deleting `user_profiles.json` removes your saved profiles; built-in profiles are unaffected.
- The EXE itself writes nothing to its own directory — safe to run from read-only locations.

---

## Build Requirements

Install the Rust toolchain from [rustup.rs](https://rustup.rs/), then verify:

```powershell
rustc --version
cargo --version
```

---

## Build And Run

```powershell
# Debug build + run (slower, larger, includes debug info)
cargo build
cargo run

# Release build + run (optimised, stripped, ~5 MB)
cargo build --release
cargo run --release
```

---

## Executable Locations

Assuming the repo is at `C:\_repos\BS-VChanger-Rust`:

| Build | Path |
|---|---|
| Debug | `C:\_repos\BS-VChanger-Rust\target\debug\bs-vchanger.exe` |
| Release | `C:\_repos\BS-VChanger-Rust\target\release\bs-vchanger.exe` |
| Cargo install | `%USERPROFILE%\.cargo\bin\bs-vchanger.exe` |

To install the binary into your Cargo bin directory (and run it from anywhere):

```powershell
cargo install --path .
bs-vchanger
```

Make sure `%USERPROFILE%\.cargo\bin` is on your `PATH` (Rust setup does this automatically).

---

## Distributing

To share the app with someone who does not have Rust installed:

1. Build the release EXE: `cargo build --release`
2. Copy `target\release\bs-vchanger.exe` to wherever you like.
3. That's it — no installer, no DLL folder, no setup needed.

The recipient also needs [VB-CABLE](https://vb-audio.com/Cable/) installed if they
want to route processed audio into Teams, Discord, or OBS.
