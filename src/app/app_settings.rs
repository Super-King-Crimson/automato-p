use serde::{Serialize, Deserialize};
use super::error::PlainTextError;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    sound_path: Option<String>
}

impl AppSettings {
    pub fn default() -> AppSettings {
        AppSettings {
            sound_path: Some(String::from("./user/alarm.mp3")),
        }
    }

    pub fn change_sound(&mut self, path: String) -> Result<(), PlainTextError> {
        let p = Path::new(&path);

        if p.exists() {
            if let Some(ext) = p.extension() {
                if ext == "mp3" {
                    self.sound_path = Some(path);
                    Ok(())
                } else {
                    Err(PlainTextError(format!("Bad file extension (found {}, expected mp3)", ext.to_str().unwrap())))
                }
            } else {
                Err(PlainTextError("File did not have a file extension (expected mp3)".to_string()))
            }
        } else {
            Err(PlainTextError(format!("File at path '{path}' not found")))
        }
    }

    pub fn get_sound_path(&self) -> Option<&str> {
        self.sound_path.as_deref()
    }
}