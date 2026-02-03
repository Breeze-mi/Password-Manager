use crate::db::Database;
use crate::models::{Entry, Group};
use rusqlite::params;
use rust_xlsxwriter::{Format, Workbook};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use tauri::{AppHandle, State};
use tauri_plugin_dialog::DialogExt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportData {
    version: String,
    export_date: String,
    groups: Vec<Group>,
    entries: Vec<Entry>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportResult {
    groups_imported: usize,
    entries_imported: usize,
}

/// Export all groups and entries as JSON string
#[tauri::command]
pub fn export_data(db: State<Database>) -> Result<String, String> {
    let (groups, entries) = query_all_data(&db)?;

    let export_data = ExportData {
        version: "1.0".to_string(),
        export_date: chrono::Utc::now().to_rfc3339(),
        groups,
        entries,
    };

    serde_json::to_string_pretty(&export_data).map_err(|e| e.to_string())
}

/// Open save dialog and write export data to file
#[tauri::command]
pub async fn save_export_file(app: AppHandle, content: String) -> Result<(), String> {
    let file_path = app
        .dialog()
        .file()
        .set_title("保存备份文件")
        .add_filter("JSON", &["json"])
        .set_file_name("one-password-backup.json")
        .blocking_save_file();

    match file_path {
        Some(path) => {
            fs::write(path.as_path().unwrap(), content).map_err(|e| format!("无法写入文件: {}", e))?;
            Ok(())
        }
        None => Err("用户取消保存".to_string()),
    }
}

/// Open file dialog and read import data
#[tauri::command]
pub async fn load_import_file(app: AppHandle) -> Result<String, String> {
    let file_path = app
        .dialog()
        .file()
        .set_title("选择备份文件")
        .add_filter("JSON", &["json"])
        .blocking_pick_file();

    match file_path {
        Some(path) => {
            let content = fs::read_to_string(path.as_path().unwrap()).map_err(|e| format!("无法读取文件: {}", e))?;
            Ok(content)
        }
        None => Err("用户取消导入".to_string()),
    }
}

/// Import data from JSON string
/// merge_mode: true = merge (skip duplicates), false = overwrite (clear existing data)
#[tauri::command]
pub fn import_data(db: State<Database>, json_data: String, merge_mode: bool) -> Result<ImportResult, String> {
    // Parse JSON
    let import_data: ExportData =
        serde_json::from_str(&json_data).map_err(|e| format!("无效的备份文件格式: {}", e))?;

    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    // Start transaction
    conn.execute("BEGIN TRANSACTION", [])
        .map_err(|e| e.to_string())?;

    // Overwrite mode: clear existing data
    if !merge_mode {
        conn.execute("DELETE FROM entries", [])
            .map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM groups", [])
            .map_err(|e| e.to_string())?;
    }

    let mut groups_imported = 0;
    let mut entries_imported = 0;

    // Import groups
    for group in import_data.groups {
        let result = conn.execute(
            "INSERT OR IGNORE INTO groups (id, name, icon, sort_order, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                group.id,
                group.name,
                group.icon,
                group.sort_order,
                group.created_at,
                group.updated_at,
            ],
        );

        if let Ok(rows) = result {
            if rows > 0 {
                groups_imported += 1;
            }
        }
    }

    // Import entries
    for entry in import_data.entries {
        let result = conn.execute(
            "INSERT OR IGNORE INTO entries (id, group_id, title, url, username, password, notes, is_favorite, sort_order, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            params![
                entry.id,
                entry.group_id,
                entry.title,
                entry.url,
                entry.username,
                entry.password,
                entry.notes,
                entry.is_favorite as i32,
                entry.sort_order,
                entry.created_at,
                entry.updated_at,
            ],
        );

        if let Ok(rows) = result {
            if rows > 0 {
                entries_imported += 1;
            }
        }
    }

    // Commit transaction
    conn.execute("COMMIT", []).map_err(|e| e.to_string())?;

    Ok(ImportResult {
        groups_imported,
        entries_imported,
    })
}

/// Helper: query all groups and entries from database
fn query_all_data(db: &State<Database>) -> Result<(Vec<Group>, Vec<Entry>), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, name, icon, sort_order, created_at, updated_at FROM groups ORDER BY sort_order")
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

    let mut stmt = conn
        .prepare(
            "SELECT id, group_id, title, url, username, password, notes, is_favorite, sort_order, created_at, updated_at
             FROM entries ORDER BY sort_order"
        )
        .map_err(|e| e.to_string())?;

    let entries = stmt
        .query_map([], |row| {
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

    Ok((groups, entries))
}

fn format_timestamp(ts: i64) -> String {
    chrono::DateTime::from_timestamp(ts, 0)
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_default()
}

/// Export data as Excel (.xlsx) binary
#[tauri::command]
pub fn export_excel(db: State<Database>) -> Result<Vec<u8>, String> {
    let (groups, entries) = query_all_data(&db)?;

    // Build group id -> name map
    let group_map: HashMap<String, String> = groups
        .iter()
        .map(|g| (g.id.clone(), format!("{} {}", g.icon, g.name)))
        .collect();

    let mut workbook = Workbook::new();

    // Header format
    let header_fmt = Format::new().set_bold();

    // --- Sheet 1: Entries ---
    let sheet = workbook.add_worksheet();
    sheet.set_name("密码条目").map_err(|e| e.to_string())?;

    let headers = ["标题", "网址", "用户名", "密码", "备注", "分组", "收藏", "创建时间", "更新时间"];
    for (col, h) in headers.iter().enumerate() {
        sheet.write_string_with_format(0, col as u16, *h, &header_fmt)
            .map_err(|e| e.to_string())?;
    }

    // Set column widths
    let widths = [20.0, 30.0, 20.0, 20.0, 30.0, 15.0, 6.0, 20.0, 20.0];
    for (col, w) in widths.iter().enumerate() {
        sheet.set_column_width(col as u16, *w).map_err(|e| e.to_string())?;
    }

    for (i, entry) in entries.iter().enumerate() {
        let row = (i + 1) as u32;
        let group_name = entry.group_id.as_ref()
            .and_then(|gid| group_map.get(gid))
            .map(|s| s.as_str())
            .unwrap_or("未分组");

        sheet.write_string(row, 0, &entry.title).map_err(|e| e.to_string())?;
        sheet.write_string(row, 1, &entry.url).map_err(|e| e.to_string())?;
        sheet.write_string(row, 2, &entry.username).map_err(|e| e.to_string())?;
        sheet.write_string(row, 3, &entry.password).map_err(|e| e.to_string())?;
        sheet.write_string(row, 4, &entry.notes).map_err(|e| e.to_string())?;
        sheet.write_string(row, 5, group_name).map_err(|e| e.to_string())?;
        sheet.write_string(row, 6, if entry.is_favorite { "是" } else { "否" }).map_err(|e| e.to_string())?;
        sheet.write_string(row, 7, &format_timestamp(entry.created_at)).map_err(|e| e.to_string())?;
        sheet.write_string(row, 8, &format_timestamp(entry.updated_at)).map_err(|e| e.to_string())?;
    }

    // --- Sheet 2: Groups ---
    let sheet2 = workbook.add_worksheet();
    sheet2.set_name("分组").map_err(|e| e.to_string())?;

    let headers2 = ["名称", "图标", "创建时间", "更新时间"];
    for (col, h) in headers2.iter().enumerate() {
        sheet2.write_string_with_format(0, col as u16, *h, &header_fmt)
            .map_err(|e| e.to_string())?;
    }

    let widths2 = [20.0, 8.0, 20.0, 20.0];
    for (col, w) in widths2.iter().enumerate() {
        sheet2.set_column_width(col as u16, *w).map_err(|e| e.to_string())?;
    }

    for (i, group) in groups.iter().enumerate() {
        let row = (i + 1) as u32;
        sheet2.write_string(row, 0, &group.name).map_err(|e| e.to_string())?;
        sheet2.write_string(row, 1, &group.icon).map_err(|e| e.to_string())?;
        sheet2.write_string(row, 2, &format_timestamp(group.created_at)).map_err(|e| e.to_string())?;
        sheet2.write_string(row, 3, &format_timestamp(group.updated_at)).map_err(|e| e.to_string())?;
    }

    // Save to buffer
    let buf = workbook.save_to_buffer().map_err(|e| e.to_string())?;
    Ok(buf)
}

/// Open save dialog and write Excel data to file
#[tauri::command]
pub async fn save_export_excel_file(app: AppHandle, content: Vec<u8>) -> Result<(), String> {
    let file_path = app
        .dialog()
        .file()
        .set_title("保存 Excel 文件")
        .add_filter("Excel", &["xlsx"])
        .set_file_name("one-password-backup.xlsx")
        .blocking_save_file();

    match file_path {
        Some(path) => {
            fs::write(path.as_path().unwrap(), content).map_err(|e| format!("无法写入文件: {}", e))?;
            Ok(())
        }
        None => Err("用户取消保存".to_string()),
    }
}
