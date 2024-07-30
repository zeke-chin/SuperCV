use std::io;
use std::sync::Arc;
use tauri::Manager;
use clipboard_rs::{Clipboard, ClipboardContent, ClipboardContext, ClipboardWatcher, ClipboardWatcherContext, WatcherShutdown};
use log::{debug, error};
use sea_orm::DatabaseConnection;
use tokio::sync::Mutex;
use serde_json::Value;

use crate::core::clipboard::ClipboardHandle;
use crate::db::connection::init_db_connection;
use crate::db::crud;
use crate::db::entities::host_clipboard::Model;
use crate::time_it;
use crate::utils::{config, logger};
use crate::utils::config::{CONFIG, UserConfig};

pub struct ClipboardHelper {
    db: Arc<Mutex<DatabaseConnection>>,
    ctx: ClipboardContext,
    #[allow(dead_code)]
    watcher_shutdown: WatcherShutdown,
}

impl ClipboardHelper {
    pub async fn new(log_level: Option<i32>, sql_level: Option<i32>) -> Self {
        logger::init_logger(log_level, sql_level);
        // 初始化数据库连接
        let db_connection = init_db_connection(None)
            .await
            .expect("Failed to connect to database");
        let db = Arc::new(Mutex::new(db_connection));

        // 创建 ClipboardHandle
        let clipboard_manager = ClipboardHandle::new(db.clone());

        let mut watcher = ClipboardWatcherContext::new().unwrap();
        let watcher_shutdown = watcher
            .add_handler(clipboard_manager)
            .get_shutdown_channel();
        // 在新的任务中启动 watcher
        let _ = tokio::spawn(async move {
            watcher.start_watch();
        });

        Self {
            db,
            ctx: ClipboardContext::new().unwrap(),
            watcher_shutdown,
            // watcher_handle,
        }
    }


    async fn get_clipboards(
        &self,
        num: u64,
        type_list: Option<Vec<i32>>,
    ) -> Result<Vec<Model>, Box<dyn std::error::Error>> {
        let db_guard = self.db.lock().await;
        let all_entries = time_it!(async {
            crud::host_clipboard::get_clipboards_by_type_list(&db_guard, None, Some(num), type_list)
        })
            .await?;
        Ok(all_entries)
    }

    async fn search_clipboards(
        &self,
        query: &str,
        num: u64,
        type_list: Option<Vec<i32>>,
    ) -> Result<Vec<Model>, Box<dyn std::error::Error>> {
        let db_guard = self.db.lock().await;
        let all_entries = time_it!(async {
            crud::host_clipboard::get_clipboards_by_type_list(
                &db_guard,
                Some(query),
                Some(num),
                type_list,
            )
        })
            .await?;
        Ok(all_entries)
    }


    pub async fn set(&self, items: Vec<Model>) -> Result<(), String> {
        let first_type = items.first().map(|item| item.r#type);

        // 确保所有项目具有相同的类型
        if !items.iter().all(|item| Some(item.r#type) == first_type) {
            return Err("All items must have the same type".into());
        }

        // Determine clipboard content based on the type
        let clipboard_content: Vec<ClipboardContent> = match first_type {
            Some(0) => items.into_iter().map(|item| ClipboardContent::Text(item.content)).collect(),
            Some(1) | Some(2) => {
                let paths: Vec<String> = items.into_iter()
                    .flat_map(|item| {
                        if item.path.starts_with('[') && item.path.ends_with(']') {
                            // 尝试解析 JSON 数组
                            match serde_json::from_str::<Value>(&item.path) {
                                Ok(Value::Array(arr)) => arr.into_iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect::<Vec<String>>(),
                                _ => vec![item.path], // 如果解析失败，将原始字符串作为单个元素
                            }
                        } else {
                            // 如果不是 JSON 数组格式，就直接使用
                            vec![item.path]
                        }
                    })
                    .collect();
                return self.ctx.set_files(paths)
                    .map_err(|e| {
                        error!("Error setting files: {}", e);
                        e.to_string()
                    });
            }
            _ => return Err("Invalid type".into()),
        };

        // Set clipboard content
        self.ctx.set(clipboard_content)
            .map_err(|e| {
                error!("Error setting clipboard: {}", e);
                e.to_string()
            })
    }
    async fn set_clipboard(
        &self,
        clipboard: Model,
    ) -> Result<(), String> {
        self.set(vec![clipboard]).await
    }

    pub async fn get_user_config() -> UserConfig {
        CONFIG.read().unwrap().user_config.clone()
    }

    pub async fn set_user_config(user_config: UserConfig) -> io::Result<()> {
        config::update(user_config).await
    }
}
#[tauri::command]
pub async fn rs_invoke_get_clipboards(
    state: tauri::State<'_, Arc<ClipboardHelper>>,
    num: u64,
    type_list: Option<Vec<i32>>,
) -> Result<Vec<Model>, String> {
    match state.get_clipboards(num, type_list).await {
        Ok(clipboards) => Ok(clipboards),
        Err(e) => {
            error!("rs_invoke_get_clipboards err: {:?}", e);
            Err(format!("Failed to get clipboards: {}", e))
        }
    }
}

#[tauri::command]
pub async fn rs_invoke_search_clipboards(
    state: tauri::State<'_, Arc<ClipboardHelper>>,
    query: &str,
    num: u64,
    type_list: Option<Vec<i32>>,
) -> Result<Vec<Model>, String> {
    match state.search_clipboards(query, num, type_list).await {
        Ok(clipboards) => Ok(clipboards),
        Err(e) => {
            error!("rs_invoke_search_clipboards err: {:?}", e);
            Err(format!("Failed to search clipboards: {}", e))
        }
    }
}

#[tauri::command]
pub async fn rs_invoke_set_clipboards(
    state: tauri::State<'_, Arc<ClipboardHelper>>,
    item: Model,
) -> Result<bool, String> {
    match state.set_clipboard(item).await {
        Ok(_) => Ok(true),
        Err(e) => {
            error!("rs_invoke_set_clipboards err: {:?}", e);
            Err(format!("Failed to set clipboard: {}", e))
        }
    }
}

#[tauri::command]
pub async fn rs_invoke_get_user_config(
    _: tauri::State<'_, Arc<ClipboardHelper>>,
) -> Result<UserConfig, String> {
    match ClipboardHelper::get_user_config().await {
        config => Ok(config),
    }
}

#[tauri::command]
pub async fn rs_invoke_set_user_config(
    _: tauri::State<'_, Arc<ClipboardHelper>>,
    user_config: UserConfig,
) -> Result<bool, String> {
    match ClipboardHelper::set_user_config(user_config).await {
        Ok(()) => Ok(true),
        Err(e) => {
            error!("rs_invoke_set_config err: {:?}", e);
            Err(format!("Failed to set config: {}", e))
        }
    }
}

#[tauri::command]
pub fn rs_invoke_open_settings(window: tauri::Window) -> Result<(), String> {
    if let Some(settings_window) = window.get_window("settings") {
        settings_window.show().map_err(|e| e.to_string())?;
        settings_window.set_focus().map_err(|e| e.to_string())?;
    } else {
        return Err("Settings window not found".into());
    }
    Ok(())
}
