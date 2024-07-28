from sqlalchemy import Column, Integer, String, Boolean
from sqlalchemy.dialects.postgresql import JSONB

from app.db.sql import BaseModel


class ClipboardEntries(BaseModel):
    __tablename__ = "clipboard_entries"
    id = Column(Integer, primary_key=True, index=True, comment="id")
    device_id = Column(Integer, comment="设备ID")
    user_id = Column(Integer, comment="用户ID")
    content_type = Column(Integer, comment="类型, 0: 文本, 1: 文件")
    content = Column(String, comment="文本就是内容, 文件就是文件名")
    created_at = Column(Integer, comment="创建时间")
