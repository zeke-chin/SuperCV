use crate::client::models::user::{UserRegister, UserResp};

#[cfg(test)]
mod tests {
    use crate::client::common::ClientUserTrait;
    use crate::client::http_client::HttpClient;
    use crate::client::models::clipboard::{ClipboardResp, CreateClipboard};
    use crate::utils::hash::{hash_str, hash_vec};
    use mockito::mock;
    use chrono::Utc;

    fn create_mock_server(method: &str, path: &str, body: &str) -> mockito::Mock {
        mock(method, path)
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(body)
            .create()
    }

    #[tokio::test]
    async fn test_create_clipboard() {
        let mock_server = create_mock_server(
            "POST",
            "/content",
            r#"
            {
                "code": 200,
                "data": {
                    "id": 1,
                    "device_id": 123,
                    "type": 1,
                    "content": "VGVzdCBjb250ZW50",
                    "path": "/test/path",
                    "hash": "testhash",
                    "timestamp": 1625567890
                },
                "error_msg": null
            }"#,
        );

        let client = HttpClient::new(mockito::server_url());
        let create_clipboard = CreateClipboard {
            device_id: 123,
            r#type: 1,
            content: "VGVzdCBjb250ZW50".to_string(),
            path: "/test/path".to_string(),
            hash: "testhash".to_string(),
            timestamp: 1625567890,
        };

        let result = client.create_clipboard(create_clipboard.clone()).await;
        assert!(result.is_ok());
        let clipboard_resp = result.unwrap();
        assert_content_equal(&clipboard_resp, &create_clipboard);

        mock_server.assert();
    }

    #[tokio::test]
    async fn test_get_clipboards_by_id() {
        let mock_server = create_mock_server(
            "GET",
            "/content/1",
            r#"
            {
                "code": 200,
                "data":
                    {
                        "id": 1,
                        "device_id": 123,
                        "type": 1,
                        "content": "VGVzdCBjb250ZW50 1",
                        "path": "/test/path1",
                        "hash": "testhash1",
                        "timestamp": 1625567890
                    },
                "error_msg": null
            }"#,
        );

        let client = HttpClient::new(mockito::server_url());
        let result = client.get_clipboards_by_id(1).await;
        assert!(result.is_ok());
        let clipboard = result.unwrap();
        assert_eq!(clipboard.id, 1);
        mock_server.assert();
    }

    #[tokio::test]
    async fn test_clipboard_integration() {
        let server_url = "http://127.0.0.1:8000";  // Assuming your server is running on this address
        let client = HttpClient::new(server_url.to_string());

        // Create a clipboard entry
        let create_clipboard = CreateClipboard {
            device_id: 123,
            r#type: 1,
            content: "VGVzdCBjb250ZW50".to_string(),
            path: "/test/path".to_string(),
            hash: hash_str("Test content"),
            timestamp: Utc::now().timestamp(),
        };

        let create_result = client.create_clipboard(create_clipboard.clone()).await;
        assert!(create_result.is_ok());
        let created_clipboard = create_result.unwrap();

        assert_content_equal(&created_clipboard, &create_clipboard);

        // Get clipboards for the user
        let get_result = client.get_clipboards_by_id(created_clipboard.id).await;  // Assuming device_id is the same as user_id for this test
        assert!(get_result.is_ok());
        let clipboards = get_result.unwrap();
        assert_eq!(clipboards, created_clipboard)
    }


    fn assert_content_equal(resp: &ClipboardResp, req: &CreateClipboard) {
        assert_eq!(resp.device_id, req.device_id);
        assert_eq!(resp.r#type, req.r#type);
        assert_eq!(resp.content, req.content);
        assert_eq!(resp.path, req.path);
        assert_eq!(resp.hash, req.hash);
        assert_eq!(resp.timestamp, req.timestamp);
    }
}