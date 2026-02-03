use tauri::State;
use crate::db::Database;
use crate::models::{Entry, CreateEntryDto, UpdateEntryDto};

#[tauri::command]
pub fn get_entries(
    db: State<Database>,
    group_id: Option<String>,
    search: Option<String>,
    favorites_only: Option<bool>,
) -> Result<Vec<Entry>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut sql = String::from(
        "SELECT id, group_id, title, url, username, password, notes, is_favorite, sort_order, created_at, updated_at
         FROM entries WHERE 1=1"
    );
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref gid) = group_id {
        sql.push_str(" AND group_id = ?");
        params.push(Box::new(gid.clone()));
    }

    if let Some(true) = favorites_only {
        sql.push_str(" AND is_favorite = 1");
    }

    if let Some(ref keyword) = search {
        if !keyword.is_empty() {
            sql.push_str(" AND (title LIKE ? OR url LIKE ? OR username LIKE ?)");
            let pattern = format!("%{}%", keyword);
            params.push(Box::new(pattern.clone()));
            params.push(Box::new(pattern.clone()));
            params.push(Box::new(pattern));
        }
    }

    sql.push_str(" ORDER BY sort_order ASC, updated_at DESC");

    let params_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let entries = stmt
        .query_map(params_refs.as_slice(), |row| {
            Ok(Entry {
                id: row.get(0)?,
                group_id: row.get(1)?,
                title: row.get(2)?,
                url: row.get(3)?,
                username: row.get(4)?,
                password: row.get(5)?,
                notes: row.get(6)?,
                is_favorite: row.get::<_, i32>(7)? != 0,
                sort_order: row.get(8)?,
                created_at: row.get(9)?,
                updated_at: row.get(10)?,
            })
        })
        .map_err(|e| e.to_string())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| e.to_string())?;

    Ok(entries)
}

#[tauri::command]
pub fn create_entry(db: State<Database>, entry: CreateEntryDto) -> Result<Entry, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp();

    let url = entry.url.unwrap_or_default();
    let username = entry.username.unwrap_or_default();
    let password = entry.password.unwrap_or_default();
    let notes = entry.notes.unwrap_or_default();

    conn.execute(
        "INSERT INTO entries (id, group_id, title, url, username, password, notes, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
            id,
            entry.group_id,
            entry.title,
            url,
            username,
            password,
            notes,
            now,
            now,
        ],
    )
    .map_err(|e| e.to_string())?;

    Ok(Entry {
        id,
        group_id: entry.group_id,
        title: entry.title,
        url,
        username,
        password,
        notes,
        is_favorite: false,
        sort_order: 0,
        created_at: now,
        updated_at: now,
    })
}

#[tauri::command]
pub fn update_entry(
    db: State<Database>,
    id: String,
    entry: UpdateEntryDto,
) -> Result<Entry, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().timestamp();

    // Build dynamic UPDATE query
    let mut sets: Vec<String> = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref group_id) = entry.group_id {
        sets.push("group_id = ?".to_string());
        params.push(Box::new(group_id.clone()));
    }
    if let Some(ref title) = entry.title {
        sets.push("title = ?".to_string());
        params.push(Box::new(title.clone()));
    }
    if let Some(ref url) = entry.url {
        sets.push("url = ?".to_string());
        params.push(Box::new(url.clone()));
    }
    if let Some(ref username) = entry.username {
        sets.push("username = ?".to_string());
        params.push(Box::new(username.clone()));
    }
    if let Some(ref password) = entry.password {
        sets.push("password = ?".to_string());
        params.push(Box::new(password.clone()));
    }
    if let Some(ref notes) = entry.notes {
        sets.push("notes = ?".to_string());
        params.push(Box::new(notes.clone()));
    }
    if let Some(is_favorite) = entry.is_favorite {
        sets.push("is_favorite = ?".to_string());
        params.push(Box::new(is_favorite as i32));
    }
    if let Some(sort_order) = entry.sort_order {
        sets.push("sort_order = ?".to_string());
        params.push(Box::new(sort_order));
    }

    sets.push("updated_at = ?".to_string());
    params.push(Box::new(now));

    params.push(Box::new(id.clone()));

    let sql = format!("UPDATE entries SET {} WHERE id = ?", sets.join(", "));
    let params_refs: Vec<&dyn rusqlite::types::ToSql> = params.iter().map(|p| p.as_ref()).collect();

    conn.execute(&sql, params_refs.as_slice())
        .map_err(|e| e.to_string())?;

    // Return updated entry
    let entry = conn
        .query_row(
            "SELECT id, group_id, title, url, username, password, notes, is_favorite, sort_order, created_at, updated_at
             FROM entries WHERE id = ?1",
            [&id],
            |row| {
                Ok(Entry {
                    id: row.get(0)?,
                    group_id: row.get(1)?,
                    title: row.get(2)?,
                    url: row.get(3)?,
                    username: row.get(4)?,
                    password: row.get(5)?,
                    notes: row.get(6)?,
                    is_favorite: row.get::<_, i32>(7)? != 0,
                    sort_order: row.get(8)?,
                    created_at: row.get(9)?,
                    updated_at: row.get(10)?,
                })
            },
        )
        .map_err(|e| e.to_string())?;

    Ok(entry)
}

#[tauri::command]
pub fn delete_entry(db: State<Database>, id: String) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM entries WHERE id = ?1", [&id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn toggle_favorite(db: State<Database>, id: String) -> Result<bool, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().timestamp();

    let current: i32 = conn
        .query_row("SELECT is_favorite FROM entries WHERE id = ?1", [&id], |row| {
            row.get(0)
        })
        .map_err(|e| e.to_string())?;

    let new_val = if current == 0 { 1 } else { 0 };

    conn.execute(
        "UPDATE entries SET is_favorite = ?1, updated_at = ?2 WHERE id = ?3",
        rusqlite::params![new_val, now, id],
    )
    .map_err(|e| e.to_string())?;

    Ok(new_val != 0)
}
