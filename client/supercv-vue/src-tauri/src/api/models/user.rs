use crate::api::common::ClientError;
use crate::utils::config::CONFIG;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

lazy_static! {
	static ref USER_CACHE_PATH: PathBuf = {
		let cache_dir = CONFIG.read().unwrap().cache_dir.clone();
		cache_dir.join("user.toml")
	};
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct UserResp {
	pub id: i32,
	pub username: String,
	pub email: String,
	pub password_hash: String,
	pub encrypted_dek: String,
	pub created_at: i64,
	pub updated_at: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRegister {
	pub username: String,
	pub email: String,
	pub password_hash: String,
	pub encrypted_dek: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserLogin {
	pub username: String,
	pub password_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserResetPassword {
	pub username: String,
	pub email: String,
	pub password_hash: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
	pub id: i32,
	pub username: String,
	pub email: String,
	pub password_hash: String,
	pub encrypted_dek: String,
}

impl From<UserResp> for User {
	fn from(resp: UserResp) -> Self {
		User {
			id: resp.id,
			username: resp.username,
			email: resp.email,
			password_hash: resp.password_hash,
			encrypted_dek: resp.encrypted_dek,
		}
	}
}

impl User {
	pub async fn save(&self) -> Result<(), ClientError> {
		let content = toml::to_string(self).map_err(|e| ClientError::UnexpectedError(e.to_string()))?;
		fs::write(&*USER_CACHE_PATH, content)
			.await
			.map_err(|e| ClientError::UnexpectedError(e.to_string()))?;
		Ok(())
	}

	pub async fn load() -> Result<Option<User>, ClientError> {
		if !USER_CACHE_PATH.exists() {
			return Ok(None);
		}
		let content = fs::read_to_string(&*USER_CACHE_PATH)
			.await
			.map_err(|e| ClientError::UnexpectedError(e.to_string()))?;
		let user: User = toml::from_str(&content).map_err(|e| ClientError::UnexpectedError(e.to_string()))?;
		Ok(Some(user))
	}
}
