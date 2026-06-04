use std::path::PathBuf;
use super::AppSettings;

fn settings_path() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".into());
    PathBuf::from(appdata)
        .join("BS-VChanger-Rust")
        .join("settings.json")
}

/// Loads settings from `%APPDATA%\BS-VChanger-Rust\settings.json`.
/// Returns `AppSettings::default()` if the file is missing or unreadable.
pub fn load() -> AppSettings {
    let path = settings_path();
    std::fs::read_to_string(&path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

/// Saves settings to `%APPDATA%\BS-VChanger-Rust\settings.json`.
/// Creates the directory if it doesn't exist. Silently ignores write errors.
pub fn save(settings: &AppSettings) {
    let path = settings_path();
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if let Ok(json) = serde_json::to_string_pretty(settings) {
        let _ = std::fs::write(&path, json);
    }
}

/// Deletes the settings file, effectively resetting to defaults on next launch.
pub fn delete() {
    let _ = std::fs::remove_file(settings_path());
}
