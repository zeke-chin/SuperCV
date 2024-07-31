from sqlalchemy import Column, Integer, String, Boolean

from app.db.sql import BaseModel


class Device(BaseModel):
    __tablename__ = "device"
    id = Column(Integer, primary_key=True, index=True, comment="id")
    name = Column(String, comment="设备名称")
    uuid = Column(String, comment="设备 UUID")
    user_id = Column(Integer, comment="User ID")
    icon = Column(String, comment="设备图标")
    status = Column(Integer, comment="设备状态, 0: 离线, 1: 在线")
    created_at = Column(Integer, comment="User creation date")
    updated_at = Column(Integer, comment="User last update date")
