from sqlalchemy import Column, Integer, String, Boolean, LargeBinary

from app.db.sql import BaseModel


class ClipboardEntry(BaseModel):
    __tablename__ = "clipboard_entry"
    id = Column(Integer, primary_key=True, index=True, comment="id")
    device_id = Column(Integer, comment="设备ID")
    type = Column(Integer, comment="类型, 0: 文本, 1: 图片, 2: 文件")
    content = Column(String, comment="EncryptedNote")
    path = Column(String, comment="文件路径")
    hash = Column(String(16), comment="item hash")
    timestamp = Column(Integer, comment="时间戳")
