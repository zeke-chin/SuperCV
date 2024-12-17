extern crate chrono;
use std::cmp::PartialEq;
use std::io;
use std::path::{Path, PathBuf};

use chrono::offset::FixedOffset;
use chrono::{DateTime, Datelike};
use clipboard_rs::common::RustImage;
use clipboard_rs::RustImageData;
use log::debug;
use serde_json::json;
#[cfg(target_os = "linux")]
use url::Url;

use crate::core::clipboard::ClipboardHandle;
use crate::utils;
use crate::utils::config::CONFIG;
use crate::utils::file::{format_size, get_file_size};
use crate::utils::hash::hash_vec;
use crate::utils::time::get_current_date_time;

#[derive(Debug, Clone)]
pub enum ContentType {
    Text,
    Image,
    File,
}

impl ContentType {
    pub fn to_i32(&self) -> i32 {
        match self {
            ContentType::Text => 0,
            ContentType::Image => 1,
            ContentType::File => 2,
        }
    }
}

impl PartialEq for ContentType {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

#[derive(Debug)]
pub struct PasteboardContent {
    pub text_content: String, // 索引内容
    pub r#type: ContentType,  // 类型
    pub hash: String,         // content or text_content hash
    pub path: String,         // 路径
    pub date_time: DateTime<FixedOffset>,
}

impl PasteboardContent {
    // 创建文本类型的 PasteboardContent
    pub fn new(text_content: String, content_type: ContentType, hash: String, path: Option<String>) -> Self {
        PasteboardContent {
            text_content,
            r#type: content_type,
            hash,
            path: path.unwrap_or_default(),
            date_time: get_current_date_time(),
        }
    }
}

impl ClipboardHandle {
    pub(crate) fn new_text_content(&mut self, text_content: String) -> Option<PasteboardContent> {
        if crate::core::clipboard::string_is_large(&text_content) || text_content.trim().is_empty() {
            return None;
        }

        let hash = utils::hash::hash_str(&text_content);
        if self.check_hash(&hash) {
            return None;
        }
        self.last_hash = hash.clone();
        return Some(PasteboardContent::new(text_content, ContentType::Text, hash, None));
    }

    pub(crate) fn new_file_content(&mut self, file_urls: Vec<String>) -> Option<PasteboardContent> {
        const IMG_EXTENSIONS: [&str; 6] = ["png", "jpg", "jpeg", "bmp", "gif", "HEIC"];

        if file_urls.is_empty() {
            return None;
        }

        let (text_content, content_type, path) = if file_urls.len() == 1 {
            #[cfg(target_os = "linux")]
            {
                let url = Url::parse(&file_urls[0]).expect("Invalid URL");
                let decoded_path = urlencoding::decode(url.path()).expect("UTF-8");
                let path = PathBuf::from(decoded_path.as_ref());
            }

            #[cfg(any(target_os = "macos", target_os = "windows"))]
            let path = PathBuf::from(&file_urls[0]);

            // let path = url.path();
            let file_name = path.file_name()?.to_str()?;
            let is_image: bool = match path.extension() {
                Some(extension) => {
                    let file_extension = extension.to_str()?;
                    IMG_EXTENSIONS.contains(&file_extension)
                },
                None => false,
            };

            if is_image {
                let content_type = ContentType::Image;
                let prefix = "Img";
                (
                    format!("{}: {}{}", prefix, file_name, get_file_size(path.to_str()?)),
                    content_type,
                    path.to_str()?.to_string(),
                )
            } else {
                let content_type = ContentType::File;
                let prefix = "File";
                (
                    format!("{}: {}{}", prefix, file_name, get_file_size(path.to_str()?)),
                    content_type,
                    json!(vec![path.to_str()?.to_string()]).to_string(),
                )
            }
        } else {
            let paths: Vec<PathBuf> = file_urls
                .iter()
                .map(|url| {
                    #[cfg(any(target_os = "macos", target_os = "windows"))]
                    return PathBuf::from(url);

                    #[cfg(target_os = "linux")]
                    {
                        let url = Url::parse(url).expect("Invalid URL");
                        let decoded_path = urlencoding::decode(url.path()).expect("UTF-8");
                        return PathBuf::from(decoded_path.as_ref());
                    }
                })
                .collect();
            let file_names: String = paths
                .iter()
                .filter_map(|path| path.file_name().and_then(|name| name.to_str()))
                .collect::<Vec<_>>()
                .join(", ");
            let path_strings: Vec<String> = paths.into_iter().filter_map(|path| path.to_str().map(String::from)).collect();
            (
                format!("{} Files: {}", file_urls.len(), file_names),
                ContentType::File,
                json!(path_strings).to_string(),
            )
        };

        let hash = utils::hash::hash_str(&path);
        if self.check_hash(&hash) {
            return None;
        }
        self.last_hash = hash.clone();
        Some(PasteboardContent::new(text_content, content_type, hash, Some(path)))
    }

    pub(crate) fn new_img_content(&mut self, img: &RustImageData) -> Option<PasteboardContent> {
        let (w, h) = img.get_size();
        let text_content = format!("Img: {}x{} ({})", w, h, format_size(img.get_bytes().len()));
        // let text_content = format!("Img: {}x{} ({})", w, h, 0);
        let hash = hash_vec(img.get_bytes());
        // let hash = "test".to_string();

        let path = get_local_path("png").unwrap();
        img.save_to_path(&path).unwrap();
        if self.check_hash(&hash) {
            return None;
        }
        self.last_hash = hash.clone();
        Some(PasteboardContent::new(text_content, ContentType::Image, hash, Some(path)))
    }

    fn check_hash(&self, hash: &str) -> bool {
        return if self.last_hash == *hash {
            debug!("check_hash true");
            true
        } else {
            false
        };
    }
}

fn get_local_path(suffix: &str) -> Result<String, io::Error> {
    let date_time = get_current_date_time();
    let root_file_path = CONFIG
        .read()
        .unwrap()
        .files_path
        .join(format!("{}{}{}", date_time.year(), date_time.month(), date_time.day()))
        .to_str()
        .unwrap()
        .to_string();
    // 判断root_file_path 是否存在 不存在则递归创建
    if !Path::new(&root_file_path).exists() {
        std::fs::create_dir_all(&root_file_path).unwrap_or_else(|e| {
            panic!("Failed to create directories: {}", e);
        });
    }
    Ok(format!("{}/{}.{}", root_file_path, date_time.timestamp(), suffix))
}
