from fastapi import APIRouter, Depends, BackgroundTasks
from sqlalchemy.orm import Session
from fastapi_pagination import Params, paginate
from app.db.sql import get_db
from utils.nlp_web import web_try
from app import schemas, crud, models

router_user = APIRouter(
    prefix="/user",
    tags=["user-用户管理"],
)


@router_user.post("/register")
@web_try()
def register_user(item: schemas.UserRegister, db: Session = Depends(get_db)):
    return crud.user.register_user(db, item)


@router_user.post("/login")
@web_try()
def login_user(item: schemas.UserLogin, db: Session = Depends(get_db)):
    return crud.user.login_user(db, item)

@router_user.post("/reset")
@web_try()
def reset_user(item: schemas.UserResetPassword, db: Session = Depends(get_db)):
    return crud.user.reset_password_user(db, item)

