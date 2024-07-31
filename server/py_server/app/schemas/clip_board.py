from pydantic import BaseModel


class CreateContent(BaseModel):
    device_id: int
    type: int
    content: str  # EncryptedNote base64
    path: str
    hash: str
    timestamp: int

