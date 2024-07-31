from sqlalchemy import Column, Integer, String
from sqlalchemy.dialects.postgresql import JSONB

from app.db.sql import BaseModel


class User(BaseModel):
    __tablename__ = "user"
    id = Column(Integer, primary_key=True, index=True, autoincrement=True, comment="User ID")
    username = Column(String, unique=True, nullable=False, comment="User name, 不可重复")
    email = Column(String, unique=True, nullable=False, comment="User email, 不可重复")
    password_hash = Column(String(16), nullable=False, comment="User password hash")
    encrypted_dek = Column(String, nullable=False, comment="eDEK 48 字节")
    created_at = Column(Integer, comment="User creation date")
    updated_at = Column(Integer, comment="User last update date")
