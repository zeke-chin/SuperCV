use std::{env, fs};
use std::io;
use std::path::PathBuf;
use std::sync::RwLock;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use toml;

use crate::utils::time::get_current_timestamp;

lazy_static! {
    pub static ref CONFIG: RwLock<Config> = RwLock::new(Config::new());
}

pub async fn update(user_config: UserConfig) -> io::Result<()> {
    let (_, config_path) = get_paths(); // 获取配置路径
    let file_path = config_path.join("config.toml"); // 配置文件路径

    {
        let mut config = CONFIG.write().unwrap(); // 获取写锁
        config.user_config = user_config;
    } // 写锁在这里自动释放

    // 在调用异步操作之前复制所需的数据
    let user_config_to_save = {
        let config = CONFIG.read().unwrap(); // 短暂地获取读锁
        config.user_config.clone() // 复制需要保存的数据
    };

    // 现在你可以在没有锁的情况下进行异步操作
    user_config_to_save.save_async(&file_path).await?;
    Ok(())
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpiredConfig {
    pub text: i64,
    pub img: i64,
    pub file: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PreviewConfig {
    pub preview_number: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserConfig {
    pub expired_config: ExpiredConfig,
    pub preview_config: PreviewConfig,
}

pub struct Config {
    pub db_path: PathBuf,
    pub files_path: PathBuf,
    pub logs_path: PathBuf,
    pub user_config: UserConfig,
}

impl Config {
    pub fn new() -> Self {
        let (cache_dir, config_dir) = get_paths();
        let db_path = cache_dir.join("db");
        let files_path = cache_dir.join("files");
        let logs_path = cache_dir.join("logs");

        for p in [&db_path, &files_path, &logs_path, &config_dir].iter() {
            if !p.exists() {
                fs::create_dir_all(p).expect(&format!("创建目录 {} 失败", p.display()));
            }
        }

        let user_config = UserConfig::load(&config_dir).expect("加载用户配置失败");

        Self {
            db_path,
            files_path,
            logs_path,
            user_config,
        }
    }

    // text, img, file
    pub fn get_expired_ts(&self) -> (i64, i64, i64) {
        let now = get_current_timestamp();
        (
            now - self.user_config.expired_config.text * 24 * 60 * 60,
            now - self.user_config.expired_config.img * 24 * 60 * 60,
            now - self.user_config.expired_config.file * 24 * 60 * 60,
        )
    }
}

impl UserConfig {
    pub fn load(config_dir: &PathBuf) -> io::Result<Self> {
        let file_path = config_dir.join("config.toml");

        let config: UserConfig = if file_path.exists() {
            let content = fs::read_to_string(&file_path)?;
            toml::from_str(&content).unwrap_or_else(|e| {
                eprintln!("配置文件解析失败: {}. 使用默认配置。", e);
                Self::default()
            })
        } else {
            Self::default()
        };
        config.save(&file_path)?;

        Ok(config)
    }

    // 同步版本的 save
    pub fn save(&self, config_path: &PathBuf) -> io::Result<()> {
        let content =
            toml::to_string(self).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    // 异步版本的 save
    pub async fn save_async(&self, config_path: &PathBuf) -> io::Result<()> {
        use tokio::fs;

        let content =
            toml::to_string(self).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&config_path, content).await?;
        Ok(())
    }
}

fn get_paths() -> (PathBuf, PathBuf) {
    let home = dirs::home_dir().expect("读取用户家目录失败");

    // 根据操作系统返回不同的路径
    match env::consts::OS {
        "macos" | "linux" => (
            home.join(".cache").join("super-cv"),
            home.join(".config").join("super-cv"),
        ),
        "windows" => (
            home.join("Documents").join("super-cv"),
            home.join(".config").join("super-cv"),
        ),
        _ => panic!("Unsupported operating system"),
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            expired_config: ExpiredConfig {
                text: 7,
                img: 3,
                file: 3,
            },
            preview_config: PreviewConfig { preview_number: 20 },
        }
    }
}
