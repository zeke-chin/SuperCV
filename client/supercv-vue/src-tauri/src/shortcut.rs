use std::sync::Mutex;
use tauri::{AppHandle, GlobalShortcutManager, Window};

pub struct MainGlobalShortcut {
	shortcut: Mutex<Option<String>>,
	window: Window,
}

impl MainGlobalShortcut {
	pub fn new(window: Window) -> Self {
		Self {
			shortcut: Mutex::new(None),
			window,
		}
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
		shortcut_manager
			.register(new_shortcut, move || {
				let window = window.clone();
				tauri::async_runtime::spawn(async move {
					if window.is_visible().unwrap() {
						window.hide().unwrap();
					} else {
						window.show().unwrap();
						window.set_focus().unwrap();
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
