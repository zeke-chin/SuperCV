use serde::{Deserialize, Serialize};

// Struct for clipboard entries
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ClipboardResp {
	pub id: i32,
	pub device_id: i32,
	pub r#type: i32, // `type` is a reserved keyword, so we use `type_`
	pub content: String,
	pub path: String,
	pub hash: String,
	pub timestamp: i64,
}

// Struct for creating content
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateClipboard {
	pub device_id: i32,
	pub r#type: i32,      // `type` is a reserved keyword, so we use `type_`
	pub content: String, // Base64 encoded content
	pub path: String,
	pub hash: String,
	pub timestamp: i64,
}
