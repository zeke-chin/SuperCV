from pydantic import BaseModel


class UserRegister(BaseModel):
    username: str
    email: str
    password_hash: str
    encrypted_dek: str


class UserLogin(BaseModel):
    username: str
    password_hash: str


class UserResetPassword(BaseModel):
    username: str
    email: str
    password_hash: str
