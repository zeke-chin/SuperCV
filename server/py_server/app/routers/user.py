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


@router_user.post("")
@web_try()
def creat_user(item: schemas.User, db: Session = Depends(get_db)):
    return crud.user.create_user(db, item)


@router_user.get("")
@web_try()
def get_users(params: Params = Depends(), db: Session = Depends(get_db)):
    return paginate([item.to_dict() for item in crud.user.get_user(db)], params)


@router_user.get("/{user_id}")
@web_try()
def get_user(user_id: int, db: Session = Depends(get_db)):
    return crud.user.get_user(db, user_id)


@router_user.post("/{user_id}")
@web_try()
def update_user(user_id: int, item: schemas.User, db: Session = Depends(get_db)):
    return crud.user.update_user(db, item, user_id)


@router_user.delete("/{user_id}")
@web_try()
def delete_user(user_id: int, db: Session = Depends(get_db)):
    return crud.user.delete_user(db, user_id)
