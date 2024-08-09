use crate::{api::common::ClientError, utils::config::CONFIG};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

lazy_static! {
	static ref DEVICE_CACHE_PATH: PathBuf = {
		let cache_dir = CONFIG.read().unwrap().cache_dir.clone();
		cache_dir.join("device.toml")
	};
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceResp {
	pub id: i32,
	pub name: String,
	pub uuid: String,
	pub user_id: i32,
	pub icon: String,
	pub created_at: i64,
	pub updated_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateDevice {
	pub name: String,
	pub uuid: String,
	pub user_id: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateDevice {
	pub name: Option<String>,
	pub icon: Option<String>,
	pub user_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncItem {
	pub client_id: i32,
	pub timestamp: i32,
	pub hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncDevice {
	pub start_at: i32,
	pub end_at: i32,
	pub items: Vec<SyncItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncDeviceResult {
	pub update_client_ids: Vec<i32>,
	pub download_server_ids: Vec<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Device {
	pub id: i32,
	pub name: String,
	pub uuid: String,
	pub user_id: i32,
	pub icon: String,
}

impl DeviceResp {
	pub async fn save_local_path(&self, icon_bytes: Vec<u8>) -> Result<String, ClientError> {
		let icons_path = CONFIG.read().unwrap().icon_path.clone();
		let icon_name = format!("{}.png", self.uuid);
		let icon_path = icons_path.join(icon_name);
		fs::write(&icon_path, icon_bytes)
			.await
			.map_err(|e| ClientError::UnexpectedError(e.to_string()))?;
		Ok(icon_path.display().to_string())
	}
}

impl Device {
	pub async fn save(&self) -> Result<(), ClientError> {
		let content = toml::to_string(self).map_err(|e| ClientError::UnexpectedError(e.to_string()))?;
		fs::write(&*DEVICE_CACHE_PATH, content)
			.await
			.map_err(|e| ClientError::UnexpectedError(e.to_string()))?;
		Ok(())
	}

	pub async fn load() -> Result<Option<Device>, ClientError> {
		// 如果不存在就返回None
		if !DEVICE_CACHE_PATH.exists() {
			return Ok(None);
		}
		let content = fs::read_to_string(&*DEVICE_CACHE_PATH)
			.await
			.map_err(|e| ClientError::UnexpectedError(e.to_string()))?;
		let device: Device = toml::from_str(&content).map_err(|e| ClientError::UnexpectedError(e.to_string()))?;
		Ok(Some(device))
	}
}
