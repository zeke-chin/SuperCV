from fastapi import APIRouter, Depends, Query
from sqlalchemy.orm import Session
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


@router_device.post("/{device_id}")
@web_try()
def update_device(device_id: int, item: schemas.UpdateDevice, db: Session = Depends(get_db)):
    return crud.device.update_device(db, device_id, item)


@router_device.delete("/{device_id}")
@web_try()
def delete_device(device_id: int, db: Session = Depends(get_db)):
    return crud.device.delete_device(db, device_id) == True


@router_device.get("/user/{user_id}")
@web_try()
def get_devices_by_user(user_id: int, db: Session = Depends(get_db)):
    return [item.to_dict() for item in crud.device.get_device_by_user_id(db, user_id)]


@router_device.post("/{device_id}/sync")
@web_try()
def sync_device(device_id: int, item: schemas.SyncDevice, db: Session = Depends(get_db)):
    return crud.device.sync_by_device(db, device_id, item)
