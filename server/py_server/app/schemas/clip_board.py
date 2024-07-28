from pydantic import BaseModel


class CreateContent(BaseModel):
    device_id: int
    content_type: int
    content: str
