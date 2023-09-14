use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub sound_path: Option<String>
}

impl AppSettings {
    pub fn default() -> AppSettings {
        AppSettings {
            sound_path: None,
        }
    }
}