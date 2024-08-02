#[cfg(test)]
mod tests {
	use crate::client::common::ClientUserTrait;
	use crate::client::http_client::HttpClient;
	use crate::client::models::clipboard::{ClipboardResp, CreateClipboard};
	use crate::client::models::device::{CreateDevice, UpdateDevice};
	use crate::utils::hash::{hash_str, hash_vec};
	use chrono::Utc;
	use mockito::mock;

	#[tokio::test]
	async fn test_create_device() {
		let server_url = "http://127.0.0.1:8000";
		let client = HttpClient::new(server_url.parse().unwrap());

		let create_device = CreateDevice {
			name: "macos".to_string(),
			uuid: "9090".to_string(),
			user_id: 9,
		};

		let result = client.create_device(create_device.clone()).await;
		assert!(result.is_ok());
		let device_resp_c = result.unwrap();
		assert_eq!(device_resp_c.name, "macos");
		assert_eq!(device_resp_c.uuid, "9090");
		assert_eq!(device_resp_c.user_id, 9);

		let update_device = UpdateDevice {
			name: Some("linux".to_string()),
			icon: None,
			user_id: None,
		};

		let result = client.update_device(update_device, device_resp_c.id).await;
		assert!(result.is_ok());
		let device_resp = result.unwrap();
		assert_eq!(device_resp.id, device_resp_c.id);
		assert_eq!(device_resp.name, "linux");
		assert_eq!(device_resp.uuid, create_device.uuid);
		assert_eq!(device_resp.user_id, create_device.user_id);

		let del_result = client.delete_device(device_resp.id).await;
		assert!(del_result.is_ok());

	}
}
