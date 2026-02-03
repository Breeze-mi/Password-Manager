use tauri::State;
use crate::db::Database;
use crate::models::Settings;

#[tauri::command]
pub fn get_settings(db: State<Database>) -> Result<Settings, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let get_setting = |key: &str, default: &str| -> String {
        conn.query_row(
            "SELECT value FROM settings WHERE key = ?1",
            [key],
            |row| row.get(0),
        )
        .unwrap_or_else(|_| default.to_string())
    };

    Ok(Settings {
        auto_lock_minutes: get_setting("auto_lock_minutes", "5")
            .parse()
            .unwrap_or(5),
        clear_clipboard_seconds: get_setting("clear_clipboard_seconds", "30")
            .parse()
            .unwrap_or(30),
        theme: get_setting("theme", "system"),
    })
}

#[tauri::command]
pub fn update_settings(db: State<Database>, settings: Settings) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('auto_lock_minutes', ?1)",
        [&settings.auto_lock_minutes.to_string()],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('clear_clipboard_seconds', ?1)",
        [&settings.clear_clipboard_seconds.to_string()],
    )
    .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('theme', ?1)",
        [&settings.theme],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
