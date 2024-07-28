from sqlalchemy import Column, Integer, String, Boolean

from app.db.sql import BaseModel


class Device(BaseModel):
    __tablename__ = "device"
    id = Column(Integer, primary_key=True, index=True, comment="id")
    uuid = Column(String, comment="设备 UUID")
    name = Column(String, comment="设备名称")
    icon = Column(String, comment="设备图标")
    user_id = Column(Integer, comment="User ID")
    status = Column(Integer, comment="设备状态, 0: 离线, 1: 在线")
    created_at = Column(Integer, comment="User creation date")
    updated_at = Column(Integer, comment="User last update date")
