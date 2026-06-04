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
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            input_device_name:   None,
            monitor_device_name: None,
            virtual_device_name: None,
            monitor_enabled:     true,
            virtual_enabled:     false,
            last_profile_name:   Some("Clean Voice".into()),
            theme:               ThemeChoice::Dark,
        }
    }
}
