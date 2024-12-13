#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use std::sync::Arc;

use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};
#[allow(unused_imports)]
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

use crate::clipboard_helper::{
	rs_invoke_get_clipboards, rs_invoke_get_user_config, rs_invoke_open_settings, rs_invoke_search_clipboards, rs_invoke_set_clipboards,
	rs_invoke_set_user_config, ClipboardHelper,
};
use crate::shortcut::{rs_invoke_register_global_shortcut, MainGlobalShortcut};

mod api;
mod clipboard_helper;
mod core;
mod db;
mod shortcut;
mod utils;

#[tokio::main]
async fn main() {
	let clipboard_helper = ClipboardHelper::new(None, Some(2)).await;
	let clipboard_helper = Arc::new(clipboard_helper);
	// let clipboard_helper_clone = clipboard_helper.clone();

	let quit = CustomMenuItem::new("quit".to_string(), "退出");
	let show_window = CustomMenuItem::new("show_window".to_string(), "显示页面");
	let setting = CustomMenuItem::new("setting".to_string(), "设置");
	let tray_menu = SystemTrayMenu::new().add_item(show_window).add_item(setting).add_item(quit);
	let system_tray = SystemTray::new().with_menu(tray_menu);

	tauri::Builder::default()
		.setup(move |app| {
			let app_handle = app.handle();
			// windows
			let main_window = app.get_window("main").unwrap();
			#[cfg(target_os = "macos")]
			apply_vibrancy(&main_window, NSVisualEffectMaterial::HudWindow, None, Some(12.0))
				.expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

			#[cfg(target_os = "windows")]
			apply_blur(&main_window, Some((18, 18, 18, 125))).expect("Unsupported platform! 'apply_blur' is only supported on Windows");

			let main_handle = main_window.clone();
			main_handle.set_decorations(false).unwrap();
			let settings_window = app.get_window("settings").unwrap();
			settings_window.hide()?;

			// 注册全局快捷键
			let main_handle: tauri::Window = main_window.clone();
			let main_global_shortcut = MainGlobalShortcut::new(main_handle);
			main_global_shortcut.register(&app_handle, "CommandOrControl+Shift+0")?;
			app.manage(main_global_shortcut);
			// let mut global_shortcut = app.global_shortcut_manager();
			// // let window_handle = main_window.clone();
			// global_shortcut
			// 	.register("CommandOrControl+Shift+L", move || {
			// 		let w_main = w_main_handle.clone();
			// 		tauri::async_runtime::spawn(async move {
			// 			if w_main.is_visible().unwrap() {
			// 				w_main.hide().unwrap();
			// 			} else {
			// 				w_main.show().unwrap();
			// 				w_main.set_focus().unwrap();
			// 			}
			// 		});
			// 	})
			// 	.unwrap();

			// 添加失去焦点事件处理
			let window_handle = main_window.clone();
			main_window.on_window_event(move |event| {
				if let tauri::WindowEvent::Focused(false) = event {
					window_handle.hide().unwrap();
				}
			});

			let settings_handle = settings_window.clone();
			settings_window.on_window_event(move |event| {
				if let tauri::WindowEvent::CloseRequested { api, .. } = event {
					// 阻止窗口关闭
					api.prevent_close();
					// 仅隐藏窗口
					settings_handle.hide().unwrap();
				}
			});

			// 添加程序退出时的清理操作
			// let clipboard_helper = clipboard_helper_clone.clone();
			Ok(())
		})
		.system_tray(system_tray)
		.on_system_tray_event(|app, event| match event {
			SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
				"quit" => {
					std::process::exit(0);
				}
				"show_window" => {
					if let Some(window) = app.get_window("main") {
						window.show().unwrap();
						window.set_focus().unwrap();
					}
				}
				"setting" => {
					if let Some(settings) = app.get_window("settings") {
						settings.show().unwrap();
						settings.set_focus().unwrap();
					}
				}
				_ => {}
			},
			_ => {}
		})
		.manage(clipboard_helper)
		.invoke_handler(tauri::generate_handler![
			rs_invoke_get_clipboards,
			rs_invoke_search_clipboards,
			rs_invoke_set_clipboards,
			rs_invoke_get_user_config,
			rs_invoke_set_user_config,
			rs_invoke_open_settings,
			rs_invoke_register_global_shortcut
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
