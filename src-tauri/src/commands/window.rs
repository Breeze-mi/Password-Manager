use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

#[tauri::command]
pub async fn toggle_quick_access(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("quick-access") {
        // Window exists, toggle visibility
        let is_visible = window.is_visible().map_err(|e| e.to_string())?;
        if is_visible {
            window.hide().map_err(|e| e.to_string())?;
        } else {
            window.show().map_err(|e| e.to_string())?;
            window.set_focus().map_err(|e| e.to_string())?;
        }
    } else {
        // Create new window
        let window = WebviewWindowBuilder::new(
            &app,
            "quick-access",
            WebviewUrl::App("quick-access.html".into()),
        )
        .title("Quick Access")
        .inner_size(420.0, 500.0)
        .min_inner_size(350.0, 300.0)
        .always_on_top(true)
        .center()
        .decorations(false)
        .resizable(true)
        .skip_taskbar(true)
        .shadow(true)
        .build()
        .map_err(|e| e.to_string())?;

        // Set focus
        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn show_quick_access(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("quick-access") {
        window.show().map_err(|e| e.to_string())?;
        window.set_focus().map_err(|e| e.to_string())?;
    } else {
        // Create new window
        let window = WebviewWindowBuilder::new(
            &app,
            "quick-access",
            WebviewUrl::App("quick-access.html".into()),
        )
        .title("Quick Access")
        .inner_size(420.0, 500.0)
        .min_inner_size(350.0, 300.0)
        .always_on_top(true)
        .center()
        .decorations(false)
        .resizable(true)
        .skip_taskbar(true)
        .shadow(true)
        .build()
        .map_err(|e| e.to_string())?;

        window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn hide_quick_access(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("quick-access") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn close_quick_access(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("quick-access") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}
