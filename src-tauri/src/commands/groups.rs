use tauri::State;
use crate::db::Database;
use crate::models::Group;

#[tauri::command]
pub fn get_groups(db: State<Database>) -> Result<Vec<Group>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT id, name, icon, sort_order, created_at, updated_at
             FROM groups ORDER BY sort_order ASC"
        )
        .map_err(|e| e.to_string())?;

    let groups = stmt
        .query_map([], |row| {
            Ok(Group {
                id: row.get(0)?,
                name: row.get(1)?,
                icon: row.get(2)?,
                sort_order: row.get(3)?,
                created_at: row.get(4)?,
                updated_at: row.get(5)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(groups)
}

#[tauri::command]
pub fn create_group(
    db: State<Database>,
    name: String,
    icon: Option<String>,
) -> Result<Group, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp();
    let icon = icon.unwrap_or_else(|| "📁".to_string());

    // Get max sort_order
    let max_order: i32 = conn
        .query_row("SELECT COALESCE(MAX(sort_order), -1) FROM groups", [], |row| {
            row.get(0)
        })
        .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO groups (id, name, icon, sort_order, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![id, name, icon, max_order + 1, now, now],
    )
    .map_err(|e| e.to_string())?;

    Ok(Group {
        id,
        name,
        icon,
        sort_order: max_order + 1,
        created_at: now,
        updated_at: now,
    })
}

#[tauri::command]
pub fn update_group(
    db: State<Database>,
    id: String,
    name: Option<String>,
    icon: Option<String>,
) -> Result<Group, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().timestamp();

    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref name) = name {
        sets.push("name = ?".to_string());
        params.push(Box::new(name.clone()));
    }
    if let Some(ref icon) = icon {
        sets.push("icon = ?".to_string());
        params.push(Box::new(icon.clone()));
    }

    sets.push("updated_at = ?".to_string());
    params.push(Box::new(now));
    params.push(Box::new(id.clone()));

    let sql = format!("UPDATE groups SET {} WHERE id = ?", sets.join(", "));
    let params_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    conn.execute(&sql, params_refs.as_slice())
        .map_err(|e| e.to_string())?;

    let group = conn
        .query_row(
            "SELECT id, name, icon, sort_order, created_at, updated_at FROM groups WHERE id = ?1",
            [&id],
            |row| {
                Ok(Group {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    icon: row.get(2)?,
                    sort_order: row.get(3)?,
                    created_at: row.get(4)?,
                    updated_at: row.get(5)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(group)
}

#[tauri::command]
pub fn delete_group(db: State<Database>, id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Set entries' group_id to NULL
    conn.execute("UPDATE entries SET group_id = NULL WHERE group_id = ?1", [&id])
        .map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM groups WHERE id = ?1", [&id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Get count of entries for each group
#[tauri::command]
pub fn get_group_entry_counts(db: State<Database>) -> Result<Vec<(String, i32)>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT group_id, COUNT(*) FROM entries WHERE group_id IS NOT NULL GROUP BY group_id")
        .map_err(|e| e.to_string())?;

    let counts = stmt
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(counts)
}
