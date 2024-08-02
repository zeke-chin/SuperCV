use aes_gcm::{
	aead::{Aead, KeyInit, OsRng},
	AeadCore, Aes256Gcm, Nonce,
};
use pbkdf2::pbkdf2_hmac_array;
use rand::Rng;
use sha2::Sha256; // For generating random salt

const ITERATIONS: u32 = 100_000;
const SALT_SIZE: usize = 16;
pub struct CryptoHelper {
	cipher: Aes256Gcm,
}

impl CryptoHelper {
	pub fn new(super_key: &String) -> CryptoHelper {
		let mut rng = rand::thread_rng();
		let salt: [u8; SALT_SIZE] = rng.gen(); // Generate random bytes for the salt
		let kek = pbkdf2_hmac_array::<Sha256, 32>(super_key.as_bytes(), &salt, ITERATIONS);
		let cipher = Aes256Gcm::new_from_slice(&kek).unwrap();
		CryptoHelper { cipher }
	}

	pub fn encode_dek(&self, dek: &[u8]) -> Vec<u8> {
		let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 随机生成 nonce
		let ciphertext = self.cipher.encrypt(&nonce, dek).unwrap();
		let mut result = nonce.to_vec();
		result.extend_from_slice(&ciphertext);
		result
	}

	pub fn decode_dek(&self, e_dek: &[u8]) -> Vec<u8> {
		let nonce = Nonce::from_slice(&e_dek[0..12]);
		self.cipher.decrypt(nonce, &e_dek[12..]).unwrap()
	}

	pub fn gen_dek() -> [u8; 32] {
		let mut rng = rand::thread_rng();
		let dek: [u8; 32] = rng.gen();
		return dek
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use rand::Rng;

	#[test]
	fn test_encode_and_decode_dek() {
		let super_key = "test_super_key".to_string();
		let crypto_helper = CryptoHelper::new(&super_key);

		// 生成一个随机的 DEK 进行测试
		let mut rng = rand::thread_rng();
		let dek: [u8; 32] = rng.gen();

		// 编码 DEK
		let encoded_dek = crypto_helper.encode_dek(&dek);

		// 确保编码后的 DEK 长度正确（12字节nonce + 密文）
		assert!(encoded_dek.len() > 12);

		// 解码 DEK
		let decoded_dek = crypto_helper.decode_dek(&encoded_dek);

		// 验证解码后的 DEK 与原始 DEK 相同
		assert_eq!(dek.to_vec(), decoded_dek);
	}

	#[test]
	fn test_different_encodes_produce_different_results() {
		let super_key = "test_super_key".to_string();
		let crypto_helper = CryptoHelper::new(&super_key);

		let dek: [u8; 32] = rand::thread_rng().gen();

		let encoded_dek1 = crypto_helper.encode_dek(&dek);
		let encoded_dek2 = crypto_helper.encode_dek(&dek);

		// 确保两次编码产生不同的结果（因为nonce不同）
		assert_ne!(encoded_dek1, encoded_dek2);
	}

	#[test]
	fn test_different_super_keys_produce_different_results() {
		let super_key1 = "test_super_key_1".to_string();
		let super_key2 = "test_super_key_2".to_string();

		let crypto_helper1 = CryptoHelper::new(&super_key1);
		let crypto_helper2 = CryptoHelper::new(&super_key2);

		let dek: [u8; 32] = rand::thread_rng().gen();

		let encoded_dek1 = crypto_helper1.encode_dek(&dek);
		let encoded_dek2 = crypto_helper2.encode_dek(&dek);

		// 确保不同的super_key产生不同的编码结果
		assert_ne!(encoded_dek1, encoded_dek2);
	}
}
