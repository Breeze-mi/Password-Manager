use tauri::State;
use crate::db::Database;

#[tauri::command]
pub fn check_initialized(db: State<Database>) -> Result<bool, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let result: Option<String> = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'master_password_hash'",
            [],
            |row| row.get(0),
        )
        .ok();
    Ok(result.is_some())
}

#[tauri::command]
pub fn setup_password(db: State<Database>, password: String) -> Result<(), String> {
    if password.len() < 4 {
        return Err("密码长度不能少于4位".to_string());
    }

    let hash = bcrypt::hash(&password, 10).map_err(|e| e.to_string())?;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('master_password_hash', ?1)",
        [&hash],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn verify_password(db: State<Database>, password: String) -> Result<bool, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let hash: String = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'master_password_hash'",
            [],
            |row| row.get(0),
        )
        .map_err(|_| "未设置主密码".to_string())?;

    bcrypt::verify(&password, &hash).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn change_password(
    db: State<Database>,
    old_password: String,
    new_password: String,
) -> Result<(), String> {
    // Verify old password first
    let is_valid = verify_password(db.clone(), old_password)?;
    if !is_valid {
        return Err("旧密码不正确".to_string());
    }

    if new_password.len() < 4 {
        return Err("新密码长度不能少于4位".to_string());
    }

    let hash = bcrypt::hash(&new_password, 10).map_err(|e| e.to_string())?;
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('master_password_hash', ?1)",
        [&hash],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
