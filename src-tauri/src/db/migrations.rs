use super::connection::Database;

pub fn run(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let conn = db.conn.lock().map_err(|e| format!("Lock error: {}", e))?;

    // Create settings table
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );"
    )?;

    // Create groups table
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS groups (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            icon TEXT NOT NULL DEFAULT '📁',
            sort_order INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );"
    )?;

    // Create entries table
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS entries (
            id TEXT PRIMARY KEY,
            group_id TEXT,
            title TEXT NOT NULL,
            url TEXT NOT NULL DEFAULT '',
            username TEXT NOT NULL DEFAULT '',
            password TEXT NOT NULL DEFAULT '',
            notes TEXT NOT NULL DEFAULT '',
            is_favorite INTEGER NOT NULL DEFAULT 0,
            sort_order INTEGER NOT NULL DEFAULT 0,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL,
            FOREIGN KEY (group_id) REFERENCES groups(id) ON DELETE SET NULL
        );"
    )?;

    // Create indexes
    conn.execute_batch(
        "CREATE INDEX IF NOT EXISTS idx_entries_group ON entries(group_id);
         CREATE INDEX IF NOT EXISTS idx_entries_title ON entries(title);
         CREATE INDEX IF NOT EXISTS idx_entries_favorite ON entries(is_favorite);"
    )?;

    // Insert default settings if not exists
    conn.execute(
        "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
        ("auto_lock_minutes", "5"),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
        ("clear_clipboard_seconds", "30"),
    )?;
    conn.execute(
        "INSERT OR IGNORE INTO settings (key, value) VALUES (?1, ?2)",
        ("theme", "system"),
    )?;

    // Insert default groups if no groups exist
    let group_count: i64 = conn.query_row(
        "SELECT COUNT(*) FROM groups",
        [],
        |row| row.get(0),
    )?;

    if group_count == 0 {
        let now = chrono::Utc::now().timestamp();
        let default_groups = [
            ("🏢", "工作", 0),
            ("🏠", "个人", 1),
            ("🏦", "银行", 2),
            ("🎮", "娱乐", 3),
        ];

        for (icon, name, order) in &default_groups {
            let id = uuid::Uuid::new_v4().to_string();
            conn.execute(
                "INSERT INTO groups (id, name, icon, sort_order, created_at, updated_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                (&id, name, icon, order, &now, &now),
            )?;
        }
    }

    Ok(())
}
