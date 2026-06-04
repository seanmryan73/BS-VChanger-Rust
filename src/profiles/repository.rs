use std::path::PathBuf;
use super::VoiceProfile;

fn profiles_path() -> PathBuf {
    let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".into());
    PathBuf::from(appdata)
        .join("BS-VChanger-Rust")
        .join("user_profiles.json")
}

/// Loads user-created profiles from `%APPDATA%\BS-VChanger-Rust\user_profiles.json`.
/// Returns an empty Vec if the file is missing or unreadable.
pub fn load_user_profiles() -> Vec<VoiceProfile> {
    std::fs::read_to_string(profiles_path())
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
        .unwrap_or_default()
}

/// Saves the given user profiles to disk, overwriting any existing file.
pub fn save_user_profiles(profiles: &[VoiceProfile]) {
    let path = profiles_path();
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    if let Ok(json) = serde_json::to_string_pretty(profiles) {
        let _ = std::fs::write(&path, json);
    }
}
