use log::error;

use crate::api::api_client::ApiClient;
use crate::api::common::{ClientDeviceTrait, ClientError, ClientTrait};
use crate::api::models::clipboard;
use crate::api::models::device;
use crate::api::models::file;
use crate::api::models::user;

struct Api<T: ClientTrait + ClientDeviceTrait> {
	client: ApiClient<T>,
	user: Option<user::User>,
	device: Option<device::Device>,
}

impl<T: ClientTrait + ClientDeviceTrait> Api<T> {
	pub async fn new(client: ApiClient<T>) -> Self {
		let device = device::Device::load().await.unwrap_or_else(|e| {
			error!("load cache device {:?}", e);
			None
		});
		let user = user::User::load().await.unwrap_or_else(|e| {
			error!("load cache user {:?}", e);
			None
		});
		Self { client, user, device }
	}

	// User
	pub async fn register_user(&mut self, create_user: user::UserRegister) -> Result<(), ClientError> {
		let user_resp = self.client.register_user(create_user).await?;
		let user = user::User::from(user_resp);
		user.save().await?;
		self.user = Some(user);
		Ok(())
	}

	pub async fn login_user(&mut self, login_user: user::UserLogin) -> Result<(), ClientError> {
		let user_resp = self.client.login_user(login_user).await?;
		let user = user::User::from(user_resp);
		user.save().await?;
		self.user = Some(user);
		Ok(())
	}

	pub async fn reset_user_password(&mut self, reset_user: user::UserResetPassword) -> Result<(), ClientError> {
		let user_resp = self.client.reset_user(reset_user).await?;
		let user = user::User::from(user_resp);
		user.save().await?;
		self.user = Some(user);
		Ok(())
	}

	// Device
	pub async fn create_device(&mut self, create_device: device::CreateDevice) -> Result<(), ClientError> {
		let device_resp = self.client.create_device(create_device).await?;
		let device = self.client.device_resp2device(device_resp).await?;
		device.save().await?;
		self.device = Some(device);
		Ok(())
	}

	pub async fn update_device(&mut self, update_device: device::UpdateDevice, device_id: i32) -> Result<(), ClientError> {
		let device_resp = self.client.update_device(update_device, device_id).await?;
		let device = self.client.device_resp2device(device_resp).await?;
		device.save().await?;
		self.device = Some(device);
		Ok(())
	}

	pub async fn delete_device(&mut self, device_id: i32) -> Result<bool, ClientError> {
		let result = self.client.delete_device(device_id).await?;
		if result {
			self.device = None;
			// 可以考虑在这里删除本地缓存的设备信息
		}
		Ok(result)
	}

	pub async fn get_devices_by_user_id(&self, user_id: i32) -> Result<Vec<device::Device>, ClientError> {
		let device_resp_list = self.client.get_devices_by_user_id(user_id).await?;
		let mut devices = Vec::new();
		for device_resp in device_resp_list {
			let device = self.client.device_resp2device(device_resp).await?;
			devices.push(device);
		}
		Ok(devices)
	}

	pub async fn sync_device(&mut self, sync_device: device::SyncDevice, device_id: i32) -> Result<device::SyncDeviceResult, ClientError> {
		self.client.sync_device(sync_device, device_id).await
	}

	// File
	pub async fn upload_file(&self, file_path: &str) -> Result<file::FileResp, ClientError> {
		let user_id = self
			.user
			.as_ref()
			.ok_or(ClientError::UnexpectedError("User not logged in".to_string()))?
			.id;
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
}
