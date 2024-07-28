from fastapi import APIRouter, Depends, BackgroundTasks
from sqlalchemy.orm import Session
from fastapi_pagination import Params, paginate
from app.db.sql import get_db
from utils.nlp_web import web_try
from app import schemas, crud, models

router_device = APIRouter(
    prefix="/device",
    tags=["device-设备管理"],
)


@router_device.post("")
@web_try()
def create_device(item: schemas.CreateDevice, db: Session = Depends(get_db)):
    return crud.device.create_device(db, item)


@router_device.get("")
@web_try()
def get_devices(params: Params = Depends(), db: Session = Depends(get_db)):
    return paginate([item.to_dict() for item in crud.device.get_device(db)], params)


@router_device.get("/{device_id}")
@web_try()
def get_device_by_uuid(device_id: int, db: Session = Depends(get_db)):
    return crud.device.get_device(db, device_id)


@router_device.post("/{device_id}")
@web_try()
def update_device(device_id: int, item: schemas.UpdateDevice, db: Session = Depends(get_db)):
    return crud.device.update_device(db, device_id, item)


@router_device.delete("/{device_id}")
@web_try()
def delete_device(device_id: int, db: Session = Depends(get_db)):
    return crud.device.delete_device(db, device_id)


@router_device.get("/user/{user_id}")
@web_try()
def get_devices_by_user(user_id: int, params: Params = Depends(), db: Session = Depends(get_db)):
    return paginate([item.to_dict() for item in crud.device.get_device_by_user_id(db, user_id)], params)
