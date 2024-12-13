use crate::api::common::{ClientDeviceTrait, ClientError, ClientTrait};
use crate::api::models::clipboard::{ClipboardResp, CreateClipboard};
use crate::api::models::device::{CreateDevice, Device, DeviceResp, SyncDevice, SyncDeviceResult, UpdateDevice};
use crate::api::models::file;
use crate::api::models::file::FileResp;
use crate::api::models::user::{User, UserLogin, UserRegister, UserResetPassword, UserResp};
use reqwest::Client;
use serde::Deserialize;
use tokio::io::AsyncReadExt;

#[derive(Deserialize)]
struct ApiResponse<T> {
	code: i32,
	data: Option<T>,
	error_msg: Option<String>,
}

pub struct HttpClient {
	client: Client,
	base_url: String,
}

impl HttpClient {
	pub fn new(base_url: String) -> Self {
		HttpClient {
			client: Client::new(),
			base_url,
		}
	}

	async fn handle_response<T: for<'de> Deserialize<'de>>(&self, response: reqwest::Response) -> Result<T, ClientError> {
		let status = response.status();
		let body = response.text().await.map_err(|e| ClientError::NetworkError(e.to_string()))?;

		if status.is_success() {
			let api_response: ApiResponse<T> = serde_json::from_str(&body).map_err(|e| ClientError::SerializationError(e.to_string()))?;

			if api_response.code == 200 {
				api_response.data.ok_or_else(|| ClientError::ApiError {
					code: api_response.code,
					message: "No data in response".to_string(),
				})
			} else {
				Err(ClientError::ApiError {
					code: api_response.code,
					message: api_response.error_msg.unwrap_or_else(|| "Unknown error".to_string()),
				})
			}
		} else {
			Err(ClientError::ApiError {
				code: status.as_u16() as i32,
				message: body,
			})
		}
	}
}

#[async_trait::async_trait]
impl ClientTrait for HttpClient {
	async fn register_user(&self, create_user: UserRegister) -> Result<UserResp, ClientError> {
		let url = format!("{}/user/register", self.base_url);
		let response = self
			.client
			.post(&url)
			.json(&create_user)
			.send()
			.await
			.map_err(|e| ClientError::NetworkError(e.to_string()))?;
		self.handle_response(response).await
	}

	async fn login_user(&self, entity: UserLogin) -> Result<UserResp, ClientError> {
		let url = format!("{}/user/login", self.base_url);
		let response = self
			.client
			.post(&url)
			.json(&entity)
			.send()
			.await
			.map_err(|e| ClientError::NetworkError(e.to_string()))?;
		self.handle_response(response).await
	}

	async fn reset_user(&self, entity: UserResetPassword) -> Result<UserResp, ClientError> {
		let url = format!("{}/user/reset", self.base_url);
		let response = self
			.client
			.post(&url)
			.json(&entity)
			.send()
			.await
			.map_err(|e| ClientError::NetworkError(e.to_string()))?;
		self.handle_response(response).await
	}

	async fn upload_file(&self, user_id: i32, file_path: &str) -> Result<FileResp, ClientError> {
		let url = format!("{}/file/{}", self.base_url, user_id);

		let mut file = tokio::fs::File::open(file_path)
			.await
			.map_err(|e| ClientError::UnexpectedError(format!("Failed to open file: {}", e)))?;

		let mut buffer = Vec::new();
		file.read_to_end(&mut buffer)
			.await
			.map_err(|e| ClientError::UnexpectedError(format!("Failed to read file: {}", e)))?;

		// 从 file_path 中提取文件名
		let file_name = std::path::Path::new(file_path)
			.file_name()
			.and_then(|name| name.to_str())
			.ok_or_else(|| ClientError::UnexpectedError("Invalid file path".to_string()))?;

		let part = reqwest::multipart::Part::bytes(buffer).file_name(file_name.to_string());

		let form = reqwest::multipart::Form::new().part("file", part);

		let response = self
			.client
			.post(&url)
			.multipart(form)
			.send()
			.await
			.map_err(|e| ClientError::NetworkError(e.to_string()))?;

		self.handle_response(response).await
	}

	async fn get_file(&self, uri: &str) -> Result<Vec<u8>, ClientError> {
		let url = format!("{}{}", self.base_url, uri);

		let response = self.client.get(&url).send().await.map_err(|e| ClientError::NetworkError(e.to_string()))?;

		if response.status().is_success() {
			response
				.bytes()
				.await
				.map(|b| b.to_vec())
				.map_err(|e| ClientError::NetworkError(e.to_string()))
		} else {
			Err(ClientError::ApiError {
				code: response.status().as_u16() as i32,
				message: format!("Failed to get file: {}", response.status()),
			})
		}
	}

	async fn create_clipboard(&self, entity: CreateClipboard) -> Result<ClipboardResp, ClientError> {
		let url = format!("{}/content", self.base_url);
		let response = self
			.client
			.post(&url)
			.json(&entity)
			.send()
			.await
			.map_err(|e| ClientError::NetworkError(e.to_string()))?;
		self.handle_response(response).await
	}

	async fn get_clipboards_by_id(&self, content_id: i32) -> Result<ClipboardResp, ClientError> {
		let url = format!("{}/content/{}", self.base_url, content_id);
		let response = self.client.get(&url).send().await.map_err(|e| ClientError::NetworkError(e.to_string()))?;
		self.handle_response(response).await
	}

	async fn create_device(&self, entity: CreateDevice) -> Result<DeviceResp, ClientError> {
		let url = format!("{}/device", self.base_url);
		let response = self
			.client
			.post(&url)
			.json(&entity)
			.send()
			.await
			.map_err(|e| ClientError::NetworkError(e.to_string()))?;
		self.handle_response(response).await
	}

	async fn update_device(&self, entity: UpdateDevice, device_id: i32) -> Result<DeviceResp, ClientError> {
		let url = format!("{}/device/{}", self.base_url, device_id);
		let response = self
			.client
			.post(&url)
			.json(&entity)
			.send()
			.await
			.map_err(|e| ClientError::NetworkError(e.to_string()))?;
		self.handle_response(response).await
	}

	async fn delete_device(&self, device_id: i32) -> Result<bool, ClientError> {
		let url = format!("{}/device/{}", self.base_url, device_id);
		let response = self
			.client
			.delete(&url)
			.send()
			.await
			.map_err(|e| ClientError::NetworkError(e.to_string()))?;
		self.handle_response(response).await
	}

	async fn get_devices_by_user_id(&self, user_id: i32) -> Result<Vec<DeviceResp>, ClientError> {
		let url = format!("{}/device/user/{}", self.base_url, user_id);
		let response = self.client.get(&url).send().await.map_err(|e| ClientError::NetworkError(e.to_string()))?;
		self.handle_response(response).await
	}

	async fn sync_device(&self, entity: SyncDevice, device_id: i32) -> Result<SyncDeviceResult, ClientError> {
		let url = format!("{}/device/{}/sync", self.base_url, device_id);
		let response = self
			.client
			.post(&url)
			.json(&entity)
			.send()
			.await
			.map_err(|e| ClientError::NetworkError(e.to_string()))?;
		self.handle_response(response).await
	}
}

#[async_trait::async_trait]
impl ClientDeviceTrait for HttpClient {
	async fn device_resp2device(&self, device_resp: DeviceResp) -> Result<Device, ClientError> {
		let icon_bytes = self.get_file(&*device_resp.icon).await?;
		let local_path = device_resp.save_local_path(icon_bytes).await?;
		Ok(Device {
			id: device_resp.id,
			name: device_resp.name,
			uuid: device_resp.uuid,
			user_id: device_resp.user_id,
			icon: local_path,
		})
	}
}
