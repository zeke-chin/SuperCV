use std::sync::Mutex;
use tauri::{AppHandle, GlobalShortcutManager, Window};

use std::sync::Arc;
#[cfg(target_os = "linux")]
use x11rb::protocol::xproto::Screen;
#[cfg(target_os = "linux")]
use x11rb::rust_connection::RustConnection;

use crate::utils;
pub struct MainGlobalShortcut {
    shortcut: Mutex<Option<String>>,
    window: Window,
    #[cfg(target_os = "linux")]
    x11_conn: Option<Arc<RustConnection>>,
    #[cfg(target_os = "linux")]
    x11_screen: Option<Screen>,
}

impl MainGlobalShortcut {
    pub fn new(window: Window) -> Self {
        #[cfg(target_os = "linux")]
        {
            Self {
                shortcut: Mutex::new(None),
                window,
                x11_conn: None,
                x11_screen: None,
            }
        }

        #[cfg(not(target_os = "linux"))]
        {
            Self {
                shortcut: Mutex::new(None),
                window,
            }
        }
    }

    #[cfg(target_os = "linux")]
    pub fn set_x11(&mut self, conn: Arc<RustConnection>, screen: Screen) {
        self.x11_conn = Some(conn);
        self.x11_screen = Some(screen);
    }

    pub fn register(&self, app_handle: &AppHandle, new_shortcut: &str) -> Result<bool, String> {
        let mut shortcut_manager = app_handle.global_shortcut_manager();
        let mut current_shortcut = self.shortcut.lock().unwrap();

        // 如果有之前的快捷键，先注销它
        if let Some(old_shortcut) = current_shortcut.as_ref() {
            shortcut_manager.unregister(old_shortcut).map_err(|e| e.to_string())?;
        }

        // 注册新的快捷键
        let window = self.window.clone();
        #[cfg(target_os = "linux")]
        let (x11_conn, x11_screen) = (self.x11_conn.clone(), self.x11_screen.clone());

        shortcut_manager
            .register(new_shortcut, move || {
                let window = window.clone();
                #[cfg(target_os = "linux")]
                let (x11_conn, x11_screen) = (x11_conn.clone(), x11_screen.clone());
                tauri::async_runtime::spawn(async move {
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();

                        #[cfg(target_os = "linux")]
                        if let (Some(conn), Some(screen)) = (x11_conn.as_ref(), x11_screen.as_ref()) {
                            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                            if let Err(e) = utils::x11_window::activate_window(&conn, &screen, "SuperCV") {
                                eprintln!("Failed to activate window: {}", e);
                            }
                        }
                    }
                });
            })
            .map_err(|e| e.to_string())?;

        // 更新当前注册的快捷键
        *current_shortcut = Some(new_shortcut.to_string());

        Ok(true)
    }
}

// Tauri命令
#[tauri::command]
pub fn rs_invoke_register_global_shortcut(
    app_handle: AppHandle,
    shortcut: String,
    state: tauri::State<'_, MainGlobalShortcut>,
) -> Result<bool, String> {
    state.register(&app_handle, &shortcut)
}

// #[tauri::command]
// pub fn rs_invoke_register_global_shortcut(app_handle: tauri::AppHandle, shortcut: &str) -> Result<(), String> {
// 	let main_window = app_handle.get_window("main").unwrap();
// 	let main_global_shortcut = MainGlobalShortcut::new(main_window);

// 	// 先注销原有快捷键
// 	app_handle.global_shortcut_manager().unregister_all().map_err(|e| e.to_string())?;

// 	// 注册新快捷键
// 	main_global_shortcut.register(&app_handle, shortcut).map_err(|e| e.to_string())
// }
