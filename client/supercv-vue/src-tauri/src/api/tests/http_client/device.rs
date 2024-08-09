#[cfg(test)]
mod tests {
    use crate::api::common::ClientTrait;
    use crate::api::http_client::HttpClient;
    use crate::api::models::device::{CreateDevice, UpdateDevice};
    use rand::Rng;

    #[tokio::test]
    async fn test_create_device() {
        let server_url = "http://127.0.0.1:8000";
        let client = HttpClient::new(server_url.parse().unwrap());
        let mut rng = rand::thread_rng();

        let random_uuid = rng.gen_range(10..100).to_string();

        let create_device = CreateDevice {
            name: "macos".to_string(),
            uuid: random_uuid.clone(),
            user_id: 9,
        };

        let result = client.create_device(create_device.clone()).await;
        assert!(result.is_ok());
        let device_resp_c = result.unwrap();
        assert_eq!(device_resp_c.name, "macos");
        assert_eq!(device_resp_c.uuid, random_uuid);
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
