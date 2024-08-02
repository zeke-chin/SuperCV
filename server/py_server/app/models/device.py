from sqlalchemy import Column, Integer, String, Boolean

from app.db.sql import BaseModel


class Device(BaseModel):
    __tablename__ = "device"
    id = Column(Integer, primary_key=True, index=True, comment="id")
    name = Column(String, nullable=False, comment="设备名称")
    uuid = Column(String, nullable=False, comment="设备 UUID")
    user_id = Column(Integer, nullable=False, comment="User ID")
    icon = Column(String, nullable=False, comment="设备图标")
    created_at = Column(Integer, unique=True, comment="User creation date")
    updated_at = Column(Integer, unique=True, comment="User last update date")
