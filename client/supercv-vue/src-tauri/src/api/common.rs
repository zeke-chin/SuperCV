use crate::api::models::{clipboard, device, file, user};

use crate::api::models::device::{Device, DeviceResp};
use std::fmt;

#[derive(Debug, Clone)]
pub enum ClientError {
	NetworkError(String),
	ApiError { code: i32, message: String },
	SerializationError(String),
	UnexpectedError(String),
}



impl fmt::Display for ClientError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ClientError::NetworkError(e) => write!(f, "Network error: {}", e),
			ClientError::ApiError { code, message } => write!(f, "API error ({}): {}", code, message),
			ClientError::SerializationError(e) => write!(f, "Serialization error: {}", e),
			ClientError::UnexpectedError(e) => write!(f, "Unexpected error: {}", e),
		}
	}
}
#[async_trait::async_trait]
pub trait ClientTrait {
	// User
	async fn register_user(&self, create_user: user::UserRegister) -> Result<user::UserResp, ClientError>;
	async fn login_user(&self, entity: user::UserLogin) -> Result<user::UserResp, ClientError>;
	async fn reset_user(&self, entity: user::UserResetPassword) -> Result<user::UserResp, ClientError>;

	// File
	async fn upload_file(&self, user_id: i32, file_path: &str) -> Result<file::FileResp, ClientError>;
	async fn get_file(&self, uri: &str) -> Result<Vec<u8>, ClientError>;

	// Clipboard
	async fn create_clipboard(&self, create_clipboard: clipboard::CreateClipboard) -> Result<clipboard::ClipboardResp, ClientError>;
	async fn get_clipboards_by_id(&self, content_id: i32) -> Result<clipboard::ClipboardResp, ClientError>;

	// Device
	async fn create_device(&self, create_device: device::CreateDevice) -> Result<device::DeviceResp, ClientError>;
	async fn update_device(&self, update_device: device::UpdateDevice, device_id: i32) -> Result<device::DeviceResp, ClientError>;
	async fn delete_device(&self, device_id: i32) -> Result<bool, ClientError>;
	async fn get_devices_by_user_id(&self, user_id: i32) -> Result<Vec<DeviceResp>, ClientError>;
	async fn sync_device(&self, sync_device: device::SyncDevice, device_id: i32) -> Result<device::SyncDeviceResult, ClientError>;
}

#[async_trait::async_trait]
pub trait ClientDeviceTrait {
	async fn device_resp2device(&self, device_resp: DeviceResp) -> Result<Device, ClientError>;
}
