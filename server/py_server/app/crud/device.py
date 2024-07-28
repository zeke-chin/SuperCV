from sqlalchemy.orm import Session
import time

from app import schemas, models
from app.db.sql import update_to_db


def create_device(db: Session, item: schemas.CreateDevice):
    # 判断uuid是否已经存在
    if db.query(models.Device).filter(models.Device.uuid == item.uuid).first():
        raise Exception(409, f"device uuid={item.uuid} already exists")
    if db.query(models.Device).filter(models.Device.name == item.name).first():
        raise Exception(409, f"device name={item.name} already exists")
    if db.query(models.User).filter_by(id=item.user_id).first() is None:
        raise Exception(404, f"user_id={item.user_id} not exists")

    db_item = models.Device(**item.dict(), **{
        "status": 1,
        "created_at": time.time()
    })

    db.add(db_item)
    db.commit()
    return db_item.to_dict()


def get_device(db: Session, device_id: int = None):
    if not device_id:
        return db.query(models.Device).all()
    if res := db.query(models.Device).filter_by(id=device_id).first():
        return res
    raise Exception(404, f"{device_id=} not exists")


def update_device(db: Session, device_id: int, item: schemas.UpdateDevice):
    return update_to_db(db=db, item_id=device_id, update_item=item, model_cls=models.Device).to_dict()


def delete_device(db: Session, device_id: int):
    db_item = db.query(models.Device).filter_by(id=device_id).first()
    if db_item is None:
        raise Exception(404, f"{device_id=} not exists")
    db.delete(db_item)
    db.commit()
    return db_item


def get_device_by_user_id(db: Session, user_id):
    db_user = db.query(models.User).filter_by(id=user_id).first()
    if db_user is None:
        raise Exception(404, f"{user_id=} not exists")
    return db.query(models.Device).filter_by(user_id=user_id).all()
