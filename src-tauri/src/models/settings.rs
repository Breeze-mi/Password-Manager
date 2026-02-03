use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub auto_lock_minutes: i32,
    pub clear_clipboard_seconds: i32,
    pub theme: String,
}
