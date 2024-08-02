use serde::{Deserialize, Serialize};

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
