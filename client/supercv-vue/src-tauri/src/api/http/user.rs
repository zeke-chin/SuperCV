use crate::utils::base64::encode_base64;
use crate::utils::cipher::CryptoHelper;
use crate::utils::hash::hash_str;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
	username: String,
	email: String,
	password_hash: String,
	encrypted_dek: String,
	#[serde(skip)]
	password: String,
	#[serde(skip)]
	super_key: String,
	#[serde(skip)]
	crypto_helper: CryptoHelper,
}

impl User {
	pub fn new(username: String, email: String, password: String, super_key: String) -> User {
		let password_hash = hash_str(&password);
		let crypto_helper = CryptoHelper::new(&super_key);
		let dek = CryptoHelper::gen_dek();
		let encrypted_dek_bytes = crypto_helper.encode_dek(&dek);
		let encrypted_dek = encode_base64(&encrypted_dek_bytes);
		User {
			username,
			email,
			password_hash,
			encrypted_dek,
			password,
			super_key,
			crypto_helper
		}
	}

}
