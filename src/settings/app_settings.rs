use serde::{Deserialize, Serialize};
use crate::theme::ThemeChoice;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub input_device_name:   Option<String>,
    pub monitor_device_name: Option<String>,
    pub virtual_device_name: Option<String>,
    pub monitor_enabled:     bool,
    pub virtual_enabled:     bool,
    pub last_profile_name:   Option<String>,
    pub theme:               ThemeChoice,
    #[serde(default = "default_category_expanded")]
    pub category_expanded:   [bool; 4],
    #[serde(default)]
    pub auto_start:          bool,
    #[serde(default = "default_input_gain")]
    pub input_gain:          f32,
}

fn default_category_expanded() -> [bool; 4] {
    [true; 4]
}

fn default_input_gain() -> f32 { 1.0 }

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            input_device_name:   None,
            monitor_device_name: None,
            virtual_device_name: None,
            monitor_enabled:     true,
            virtual_enabled:     false,
            last_profile_name:   Some("Clean Voice".into()),
            theme:               ThemeChoice::Earth,
            category_expanded:   [true; 4],
            auto_start:          false,
            input_gain:          1.0,
        }
    }
}
