use std::fs;

use log::error;

pub fn format_size(size: usize) -> String {
    const KB: usize = 1024;
    const MB: usize = 1024 * KB;
    const GB: usize = 1024 * MB;

    if size < KB {
        format!("{} B", size)
    } else if size < MB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else if size < GB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else {
        format!("{:.2} GB", size as f64 / GB as f64)
    }
}

pub fn get_file_size(file_path: &str) -> String {
    let size = match fs::metadata(file_path) {
        Ok(metadata) => metadata.len(),
        Err(e) => {
            error!("Failed to get file size: {}, path: {}", e, file_path);
            0
        }
    };
    let size = format_size(size as usize);
    if size.is_empty() {
        "".to_string()
    } else {
        format!(" ({})", size)
    }
}
