use std::sync::Arc;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;

use clipboard_rs::{Clipboard, ClipboardContext, ClipboardHandler};
use log::{debug, error};
use sea_orm::DatabaseConnection;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

use crate::core::pasteboard::PasteboardContent;
use crate::db::crud::host_clipboard::add_clipboard_entry;
use crate::time_it;

pub struct ClipboardHandle {
    #[allow(dead_code)]
    db: Arc<Mutex<DatabaseConnection>>,
    ctx: ClipboardContext,
    pub(crate) last_hash: String,
    sender: Sender<PasteboardContent>,
    #[allow(dead_code)]
    receiver_handle: JoinHandle<()>,
    #[allow(dead_code)]
    runtime: Arc<Runtime>,
}

impl ClipboardHandle {
    pub fn new(db: Arc<Mutex<DatabaseConnection>>) -> Self {
        let ctx = ClipboardContext::new().unwrap();
        let (sender, receiver) = mpsc::channel();
        let runtime = Arc::new(Runtime::new().unwrap());

        let db_clone = db.clone();
        let runtime_clone = runtime.clone();
        let receiver_handle = std::thread::spawn(move || {
            Self::process_receiver(receiver, db_clone, runtime_clone);
        });

        ClipboardHandle {
            ctx,
            db,
            sender,
            last_hash: "".to_string(),
            receiver_handle,
            runtime,
        }
    }
    fn process_receiver(
        receiver: Receiver<PasteboardContent>,
        db: Arc<Mutex<DatabaseConnection>>,
        runtime: Arc<Runtime>,
    ) {
        while let Ok(content) = receiver.recv() {
            // debug!("Received clipboard content: {:?}", content);
            runtime.block_on(async {
                Self::add_clipboard_entry(&db, content).await;
            });
        }
    }

    async fn add_clipboard_entry(db: &Arc<Mutex<DatabaseConnection>>, content: PasteboardContent) {
        let db_guard = db.lock().await;
        time_it!(async add_clipboard_entry(&db_guard, content))
            .await
            .unwrap();
    }
}

impl ClipboardHandler for ClipboardHandle {
    fn on_clipboard_change(&mut self) {
        let mut content = None;

        let mut have_files = false;
        match self.ctx.get_files() {
            Ok(file_urls) if !file_urls.is_empty() => {
                have_files = true;
                content = self.new_file_content(file_urls);
            }
            Ok(_) => {}
            Err(e) => {
                #[cfg(target_os = "windows")]
                {}

                #[cfg(any(target_os = "macos", target_os = "linux"))]
                {
                    error!("Error getting files from clipboard: {}", e);
                }
            }
        };
        if !have_files && content.is_none() {
            if let Ok(img) = self.ctx.get_image() {
                content = self.new_img_content(&img);
            } else if let Ok(text) = self.ctx.get_text() {
                content = self.new_text_content(text);
            }
        }
        // 将content push
        if let Some(content) = content {
            let _ = self.sender.send(content);
        }
    }
}

pub(crate) fn string_is_large(input: &String) -> bool {
    const LARGE_SIZE: usize = 250000;
    let input_len = input.len();
    debug!("get_sting_length: {}", input_len);
    input_len > LARGE_SIZE
}

//
// #[cfg(test)]
// mod tests {
//     use crate::db::connection::init_db_connection;
//     use super::*;
//     use crate::db::entities::host_clipboard::Model;
//     use std::sync::Arc;
//     use tokio::sync::Mutex;
//
//     #[tokio::test]
//     async fn test_set_text() {
//         let db_connection = init_db_connection(None)
//             .await
//             .expect("Failed to connect to database");
//         let db = Arc::new(Mutex::new(db_connection));
//         let mut clipboard_manager = ClipboardHandle::new(db.clone());
//
//         let text = "abc输出输出test".to_string();
//         let item = Model {
//             id: 0,
//             r#type: 0,
//             path: "".to_string(),
//             content: text.clone(),
//             timestamp: 0,
//             hash: "abc".to_string(),
//         };
//
//         let result = clipboard_manager.set(vec![item]).await;
//         assert!(result.is_ok());
//
//         match clipboard_manager.ctx.get_text() {
//             Ok(c_text) if !c_text.is_empty() => {
//                 assert_eq!(c_text, text);
//             }
//             _ => { panic!("not text"); }
//
//         }
//     }
//
//     #[tokio::test]
//     async fn test_set_img() {
//         let db_connection = init_db_connection(None)
//             .await
//             .expect("Failed to connect to database");
//         let db = Arc::new(Mutex::new(db_connection));
//         let clipboard_manager = ClipboardHandle::new(db.clone());
//
//         let img_path = "/Users/zeke/Pictures/704_1020913_847718.jpg".to_string();
//         let item = Model {
//             id: 0,
//             r#type: 1,
//             path: img_path.clone(),
//             content: "图片".to_string(),
//             timestamp: 0,
//             hash: "abc".to_string(),
//         };
//
//         let result = clipboard_manager.set(vec![item]).await;
//         assert!(result.is_ok());
//         match clipboard_manager.ctx.get_files() {
//             Ok(file_urls) if !file_urls.is_empty() => {
//                 assert!(file_urls[0].contains(&img_path));
//             }
//             _ => { panic!("not file"); }
//         }
//     }
//
//     #[tokio::test]
//     async fn test_set_file() {
//         let db_connection = init_db_connection(None)
//             .await
//             .expect("Failed to connect to database");
//         let db = Arc::new(Mutex::new(db_connection));
//         let clipboard_manager = ClipboardHandle::new(db.clone());
//
//         let file_path = "/Users/zeke/Downloads/RustRover-2024.1.5-aarch64.dmg".to_string();
//         let item = Model {
//             id: 0,
//             r#type: 2,
//             path: file_path.clone(),
//             content: "File: /Users/zeke/Downloads/two_windows.zip".to_string(),
//             timestamp: 0,
//             hash: "abc".to_string(),
//         };
//
//         let result = clipboard_manager.set(vec![item]).await;
//         assert!(result.is_ok());
//         match clipboard_manager.ctx.get_files() {
//             Ok(file_urls) if !file_urls.is_empty() => {
//                 assert!(file_urls[0].contains(&file_path));
//             }
//             _ => { panic!("not file"); }
//         }
//     }
// }
