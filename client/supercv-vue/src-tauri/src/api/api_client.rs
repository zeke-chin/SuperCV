use super::models::{clipboard, device, file, user};
use crate::api::common::{ClientDeviceTrait, ClientError, ClientTrait};
use crate::api::models::device::{Device, DeviceResp};

pub struct ApiClient<T: ClientTrait + ClientDeviceTrait> {
	pub client: T,
}

impl<T: ClientTrait + ClientDeviceTrait> ApiClient<T> {
	pub fn new(client: T) -> Self {
		Self { client }
	}

	// User
	pub async fn register_user(&self, create_user: user::UserRegister) -> Result<user::UserResp, ClientError> {
		self.client.register_user(create_user).await
	}
	pub async fn login_user(&self, entity: user::UserLogin) -> Result<user::UserResp, ClientError> {
		self.client.login_user(entity).await
	}
	pub async fn reset_user(&self, entity: user::UserResetPassword) -> Result<user::UserResp, ClientError> {
		self.client.reset_user(entity).await
	}

	// File
	pub async fn upload_file(&self, user_id: i32, file_path: &str) -> Result<file::FileResp, ClientError> {
		self.client.upload_file(user_id, file_path).await
	}
	pub async fn get_file(&self, uri: &str) -> Result<Vec<u8>, ClientError> {
		self.client.get_file(uri).await
	}

	// Clipboard
	pub async fn create_clipboard(&self, create_clipboard: clipboard::CreateClipboard) -> Result<clipboard::ClipboardResp, ClientError> {
		self.client.create_clipboard(create_clipboard).await
	}
	pub async fn get_clipboards_by_id(&self, content_id: i32) -> Result<clipboard::ClipboardResp, ClientError> {
		self.client.get_clipboards_by_id(content_id).await
	}

	// Device
	pub async fn create_device(&self, create_device: device::CreateDevice) -> Result<device::DeviceResp, ClientError> {
		self.client.create_device(create_device).await
	}
	pub async fn update_device(&self, update_device: device::UpdateDevice, device_id: i32) -> Result<device::DeviceResp, ClientError> {
		self.client.update_device(update_device, device_id).await
	}
	pub async fn delete_device(&self, device_id: i32) -> Result<bool, ClientError> {
		self.client.delete_device(device_id).await
	}
	pub async fn get_devices_by_user_id(&self, user_id: i32) -> Result<Vec<DeviceResp>, ClientError> {
		self.client.get_devices_by_user_id(user_id).await
	}
	pub async fn sync_device(&self, sync_device: device::SyncDevice, device_id: i32) -> Result<device::SyncDeviceResult, ClientError> {
		self.client.sync_device(sync_device, device_id).await
	}

	// ClientDeviceTrait
	pub async fn device_resp2device(&self, device_resp: DeviceResp) -> Result<Device, ClientError> {
		self.client.device_resp2device(device_resp).await
	}
}
