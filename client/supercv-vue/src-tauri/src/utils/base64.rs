use base64::engine::general_purpose;
use base64::Engine;
use std::error::Error;

pub fn encode_base64(data: &[u8]) -> String {
	general_purpose::STANDARD.encode(data)
}

pub fn decode_base64(encoded: &str) -> Result<Vec<u8>, Box<dyn Error>> {
	let decoded_bytes = general_purpose::STANDARD.decode(encoded)?;
	Ok(decoded_bytes)
}
