# BS-VChanger-Rust

## Installer

- [rustup.rs](http://rustup.rs/) - The Rust toolchain installer.

On Windows, install Rust from rustup, then open a new terminal and verify:

```powershell
rustc --version
cargo --version
```

## Build And Run

From the project root:

```powershell
cd C:\_repos\BS-VChanger-Rust
```

Build (debug):

```powershell
cargo build
```

Run (debug):

```powershell
cargo run
```

Build release:

```powershell
cargo build --release
```

Run release:

```powershell
cargo run --release
```

## Install The App

If you want Cargo to install the binary into your Cargo bin directory:

```powershell
cargo install --path .
```

After install, run it from anywhere with:

```powershell
bs-vchanger
```

## Executable Locations

Assuming the repo is at `C:\_repos\BS-VChanger-Rust`:

- Debug build exe:
	- `C:\_repos\BS-VChanger-Rust\target\debug\bs-vchanger.exe`
- Release build exe:
	- `C:\_repos\BS-VChanger-Rust\target\release\bs-vchanger.exe`
- Cargo-installed exe:
	- `%USERPROFILE%\\.cargo\\bin\\bs-vchanger.exe`

Notes:

- `cargo --build` is invalid. Use `cargo build`.
- If `%USERPROFILE%\\.cargo\\bin` is not on `PATH`, either add it or run the full exe path directly.

