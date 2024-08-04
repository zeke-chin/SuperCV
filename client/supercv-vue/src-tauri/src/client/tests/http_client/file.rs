#[cfg(test)]
mod tests {
	use crate::client::common::ClientUserTrait;
	use crate::client::http_client::HttpClient;
	use crate::utils::hash::{hash_str, hash_vec};
	use std::path::PathBuf;
	use tokio::fs::{remove_file, File};
	use tokio::io::AsyncWriteExt;

	async fn create_temp_file(content: &str) -> PathBuf {
		let path = PathBuf::from("test_temp_file.txt");
		let mut file = File::create(&path).await.unwrap();
		file.write_all(content.as_bytes()).await.unwrap();
		path
	}

	async fn cleanup_temp_file(path: &PathBuf) {
		remove_file(path).await.unwrap();
	}

	#[tokio::test]
	async fn test_upload_and_get_file() {
		let file_content = "Hello, World!";
		let file_path = create_temp_file(file_content).await;
		let file_name = "test.txt";
		let user_id = 1;

		let server_url = "http://127.0.0.1:8000";
		let client = HttpClient::new(server_url.parse().unwrap());

		// Test file upload
		let upload_result = client.upload_file(user_id, file_path.to_str().unwrap(), file_name).await;
		assert!(upload_result.is_ok());
		let file_resp = upload_result.unwrap();
		assert!(file_resp.uri.contains(file_name));

		// Test file download
		let download_result = client.get_file(&file_resp.uri).await;
		assert!(download_result.is_ok());
		let downloaded_content = download_result.unwrap();

		// Verify the content using hash
		assert_eq!(hash_vec(&downloaded_content), hash_str(file_content));

		// Cleanup: remove the temporary file
		cleanup_temp_file(&file_path).await;
	}
}
