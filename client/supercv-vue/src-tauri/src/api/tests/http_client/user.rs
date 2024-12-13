#[cfg(test)]
mod tests {
	use crate::api::common::ClientTrait;
	use crate::api::http_client::HttpClient;
	use crate::api::models::user::{UserLogin, UserRegister, UserResetPassword, UserResp};
	use mockito::{mock, server_url};
	use rand::Rng;

	fn create_mock_server(method: &str, path: &str, mut body: Option<&str>) -> mockito::Mock {
		if body.is_none() {
			body = Some(
				r#"
{
    "code": 200,
    "data": {
        "id": 1,
        "username": "testuser",
        "email": "test@example.com",
        "password_hash": "somehashvalue",
        "encrypted_dek": "someencryptedkey",
        "created_at": 1625567890,
        "updated_at": 1625567891
    },
    "error_msg": null
}"#,
			);
		}
		mock(method, path)
			.with_status(200)
			.with_header("content-type", "application/json")
			.with_body(body.unwrap())
			.create()
	}

	#[tokio::test]
	async fn test_register_user() {
		let mock_server = create_mock_server("POST", "/user/register", None);

		let client = HttpClient::new(server_url());
		let user = UserRegister {
			username: "testuser".to_string(),
			email: "test@example.com".to_string(),
			password_hash: "hashedpassword".to_string(),
			encrypted_dek: "encrypteddek".to_string(),
		};

		let result = client.register_user(user).await;
		assert!(result.is_ok());
		let user_resp = result.unwrap();
		assert_eq!(user_resp.id, 1);
		assert_eq!(user_resp.username, "testuser");
		assert_eq!(user_resp.email, "test@example.com");

		mock_server.assert();
	}

	#[tokio::test]
	async fn test_login_user() {
		let mock_server = create_mock_server("POST", "/user/login", None);

		let client = HttpClient::new(server_url());
		let login = UserLogin {
			username: "testuser".to_string(),
			password_hash: "somehashvalue".to_string(),
		};

		let result = client.login_user(login).await;
		assert!(result.is_ok());
		let user_resp = result.unwrap();
		assert_eq!(user_resp.username, "testuser");
		assert_eq!(user_resp.password_hash, "somehashvalue");

		mock_server.assert();
	}

	#[tokio::test]
	async fn test_reset_user() {
		let mock_server = create_mock_server("POST", "/user/reset", None);

		let client = HttpClient::new(server_url());
		let reset = UserResetPassword {
			username: "testuser".to_string(),
			email: "test@example.com".to_string(),
			password_hash: "newhashedpassword".to_string(),
		};

		let result = client.reset_user(reset).await;
		assert!(result.is_ok());
		let user_resp = result.unwrap();
		assert_eq!(user_resp.id, 1);
		assert_eq!(user_resp.username, "testuser");
		assert_eq!(user_resp.email, "test@example.com");

		mock_server.assert();
	}

	#[tokio::test]
	async fn test_user() {
		let mut rng = rand::thread_rng();
		let server_url = "http://127.0.0.1:8000";
		let client = HttpClient::new(server_url.parse().unwrap());

		// 生成一个 2 位数的随机整数
		let random_two_number = rng.gen_range(10..100);
		let user_register = UserRegister {
			username: format!("test_{}", random_two_number),
			email: format!("{}@test", random_two_number),
			password_hash: format!("ph_{}", random_two_number),
			encrypted_dek: format!("ph_{}", random_two_number),
		};

		let user_login = UserLogin {
			username: format!("test_{}", random_two_number),
			password_hash: format!("ph_{}", random_two_number),
		};
		let user_reset_password = UserResetPassword {
			username: format!("test_{}", random_two_number),
			email: format!("{}@test", random_two_number),
			password_hash: "reset_ph".to_string(),
		};

		// 注册
		let res_regi = client.register_user(user_register.clone()).await;
		assert!(res_regi.is_ok());
		assert_user_fields_match(&res_regi.unwrap(), &user_register);

		// 登录
		let res_login = client.login_user(user_login).await;
		assert!(res_login.is_ok());
		assert_user_fields_match(&res_login.unwrap(), &user_register);

		// reset
		let res_reset = client.reset_user(user_reset_password.clone()).await;
		assert!(res_reset.is_ok());
		assert_eq!(res_reset.unwrap().email, user_reset_password.email);
	}

	fn assert_user_fields_match(resp: &UserResp, reg: &UserRegister) {
		assert_eq!(resp.username, reg.username, "Username mismatch");
		assert_eq!(resp.email, reg.email, "Email mismatch");
		assert_eq!(resp.password_hash, reg.password_hash, "Password hash mismatch");
		assert_eq!(resp.encrypted_dek, reg.encrypted_dek, "Encrypted DEK mismatch");
	}
}
