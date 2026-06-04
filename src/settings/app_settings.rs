use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::theme::ThemeChoice;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppSettings {
    pub input_device_name:   Option<String>,
    pub monitor_device_name: Option<String>,
    pub virtual_device_name: Option<String>,
    pub last_profile_id:     Option<Uuid>,
    pub theme:               ThemeChoice,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            input_device_name:   None,
            monitor_device_name: None,
            virtual_device_name: None,
            last_profile_id:     None,
            theme:               ThemeChoice::Dark,
        }
    }
}
