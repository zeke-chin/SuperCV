from sqlalchemy import Column, Integer, String, Boolean
from sqlalchemy.dialects.postgresql import JSONB

from app.db.sql import BaseModel


class User(BaseModel):
    __tablename__ = "user"
    id = Column(Integer, primary_key=True, index=True, comment="User ID")
    name = Column(String, comment="User name, 不可重复")
    created_at = Column(Integer, comment="User creation date")
    updated_at = Column(Integer, comment="User last update date")
