from pydantic import BaseModel


class CreateDevice(BaseModel):
    name: str
    uuid: str
    user_id: int


class UpdateDevice(BaseModel):
    name: str = ""
    icon: str = ""
    user_id: int = 0


class SyncItem(BaseModel):
    client_id: int
    timestamp: int
    hash: str


class SyncDevice(BaseModel):
    start_at: int
    end_at: int
    items: list[SyncItem]


class SyncDeviceResult(BaseModel):
    update_client_ids: list[int]
    download_server_ids: list[int]
