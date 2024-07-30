use mac_address::get_mac_address;
use sysinfo;
use twox_hash::xxh3::hash64;
use uuid::Uuid;

pub fn generate_device_uuid() -> String {
	let combined_info = get_combined_info();
	let hash_result = hash64(combined_info.as_bytes());

	// 将哈希结果转换为UUID
	let uuid = Uuid::from_u64_pair(
		(hash_result >> 32) as u64,
		(hash_result & 0xFFFFFFFF) as u64,
	);

	uuid.to_string()
}

fn get_combined_info() -> String {
	// 获取MAC地址
	let mac = get_mac_address().unwrap().unwrap().to_string();

	// 获取主机名
	let hostname = sysinfo::System::host_name().unwrap_or_else(|| "unknown".to_string());

	// 获取操作系统信息
	let os_name = sysinfo::System::name().unwrap_or_else(|| "unknown".to_string());
	let os_version = sysinfo::System::os_version().unwrap_or_else(|| "unknown".to_string());

	// 组合所有信息
	format!("{}:{}:{}:{}", mac, hostname, os_name, os_version)
}

#[cfg(test)]
mod tests {
	use crate::utils::uuid::*;

	#[test]
	fn test_ddevice_uuid() {
		println!("combined_info: {}", get_combined_info());
		let device_uuid = generate_device_uuid();
		println!("device_id: {}", device_uuid)
	}
}
