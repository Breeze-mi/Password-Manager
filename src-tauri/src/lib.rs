mod commands;
mod db;
mod models;

use db::Database;
use tauri::{
    image::Image,
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WindowEvent,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize database
    let database = Database::new().expect("Failed to initialize database");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // Create system tray
            #[cfg(desktop)]
            {
                create_tray(app)?;
            }

            // Register global shortcut plugin with handler
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{Code, Modifiers, ShortcutState};

                let handle = app.handle().clone();
                let shortcut_plugin = tauri_plugin_global_shortcut::Builder::new()
                    .with_shortcut("ctrl+shift+p")
                    .map(|builder| {
                        builder.with_handler(move |_app, shortcut, event| {
                            if event.state == ShortcutState::Pressed {
                                if shortcut.matches(Modifiers::CONTROL | Modifiers::SHIFT, Code::KeyP) {
                                    println!("Quick Access shortcut triggered!");
                                    let handle = handle.clone();
                                    tauri::async_runtime::spawn(async move {
                                        if let Err(e) = commands::window::show_quick_access(handle).await {
                                            eprintln!("Failed to show quick access: {}", e);
                                        }
                                    });
                                }
                            }
                        })
                        .build()
                    });

                match shortcut_plugin {
                    Ok(plugin) => {
                        if let Err(e) = app.handle().plugin(plugin) {
                            eprintln!("Warning: Failed to register global shortcut plugin: {}. The shortcut may already be in use.", e);
                        }
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to create global shortcut: {}. The shortcut may already be in use.", e);
                    }
                }
            }

            Ok(())
        })
        .on_window_event(|window, event| {
            // Handle window close event - hide to tray instead of closing
            if let WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main" {
                    // Prevent the window from closing
                    api.prevent_close();
                    // Hide the window instead
                    let _ = window.hide();
                }
            }
        })
        .manage(database)
        .invoke_handler(tauri::generate_handler![
            // Auth commands
            commands::auth::check_initialized,
            commands::auth::setup_password,
            commands::auth::verify_password,
            commands::auth::change_password,
            // Entry commands
            commands::entries::get_entries,
            commands::entries::create_entry,
            commands::entries::update_entry,
            commands::entries::delete_entry,
            commands::entries::toggle_favorite,
            // Group commands
            commands::groups::get_groups,
            commands::groups::create_group,
            commands::groups::update_group,
            commands::groups::delete_group,
            commands::groups::get_group_entry_counts,
            // Settings commands
            commands::settings::get_settings,
            commands::settings::update_settings,
            // Window commands
            commands::window::toggle_quick_access,
            commands::window::show_quick_access,
            commands::window::hide_quick_access,
            commands::window::close_quick_access,
            // Export/Import commands
            commands::export::export_data,
            commands::export::save_export_file,
            commands::export::load_import_file,
            commands::export::import_data,
            commands::export::export_excel,
            commands::export::save_export_excel_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(desktop)]
fn create_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Create tray menu items
    let show_item = MenuItem::with_id(app, "show", "显示主窗口", true, None::<&str>)?;
    let lock_item = MenuItem::with_id(app, "lock", "锁定", true, None::<&str>)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;

    // Create the menu
    let menu = Menu::with_items(app, &[&show_item, &lock_item, &separator, &quit_item])?;

    // Load tray icon
    let icon = Image::from_path("icons/32x32.png")
        .or_else(|_| Image::from_path("src-tauri/icons/32x32.png"))
        .unwrap_or_else(|_| Image::from_bytes(include_bytes!("../icons/32x32.png")).expect("Failed to load tray icon"));

    // Create the tray icon
    let _tray = TrayIconBuilder::new()
        .icon(icon)
        .menu(&menu)
        .tooltip("One-Password")
        .on_menu_event(|app, event| {
            match event.id.as_ref() {
                "show" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "lock" => {
                    // Emit lock event to frontend
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.emit("lock-app", ());
                    }
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            // Handle tray icon click - show window on left click
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}
