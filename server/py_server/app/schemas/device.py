from pydantic import BaseModel


class CreateDevice(BaseModel):
    name: str
    uuid: str
    user_id: int


class UpdateDevice(BaseModel):
    name: str = ""
    icon: str = ""
    user_id: int = 0
