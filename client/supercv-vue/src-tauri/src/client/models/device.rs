use serde::{Deserialize, Serialize};

// For responses (e.g., fetching device details)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DeviceResp {
    pub id: i32,
    pub name: String,
    pub uuid: String,
    pub user_id: i32,
    pub icon: String,
    pub created_at: i64,
    pub updated_at: i64,
}

// For creating a device
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateDevice {
    pub name: String,
    pub uuid: String,
    pub user_id: i32,
}

// For updating a device
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateDevice {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub user_id: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncItem {
    pub client_id: i32,
    pub timestamp: i32,
    pub hash: String,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncDevice {
    pub start_at: i32,
    pub end_at: i32,
    pub items: Vec<SyncItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncDeviceResult {
    pub update_client_ids: Vec<i32>,
    pub download_server_ids: Vec<i32>,
}
