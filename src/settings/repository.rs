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
///
/// **Atomic**, via a sibling `.tmp` and a rename. This matters more than it
/// looks: [`load`] falls back to `default()` on a parse error, so a crash or a
/// power cut part-way through a plain `fs::write` leaves truncated JSON that
/// presents as *"the app forgot every setting"* with nothing anywhere to say
/// why. `rename` on NTFS replaces the target in one step, so the file on disk is
/// only ever the old copy or the new one.
///
/// **Returns the error rather than swallowing it.** The previous version
/// discarded three separate failures (`let _ = create_dir_all`, `if let Ok(json)`,
/// `let _ = fs::write`), which is the same defect one layer down: a save could
/// fail every single time and the only symptom would be settings quietly
/// reverting on the next launch.
pub fn save(settings: &AppSettings) -> Result<(), String> {
    let path = settings_path();
    let dir = path
        .parent()
        .ok_or_else(|| format!("{} has no parent directory", path.display()))?;
    std::fs::create_dir_all(dir).map_err(|e| format!("creating {}: {e}", dir.display()))?;

    let json = serde_json::to_string_pretty(settings).map_err(|e| format!("serialising settings: {e}"))?;

    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, json).map_err(|e| format!("writing {}: {e}", tmp.display()))?;
    // Windows `rename` replaces an existing file, unlike some other platforms.
    std::fs::rename(&tmp, &path).map_err(|e| format!("replacing {}: {e}", path.display()))?;
    Ok(())
}

/// Deletes the settings file, effectively resetting to defaults on next launch.
pub fn delete() {
    let _ = std::fs::remove_file(settings_path());
}
