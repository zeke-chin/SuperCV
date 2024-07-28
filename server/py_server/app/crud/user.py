from sqlalchemy.orm import Session
import time

from app import schemas, models
from app.db.sql import update_to_db


def create_user(db: Session, item: schemas.User):
    if db.query(models.User).filter_by(name=item.name).first():
        raise Exception(409, f"user_name={item.name} already exists")

    user = models.User(
        name=item.name,
        created_at=int(time.time()),
    )

    db.add(user)
    db.commit()
    return user.to_dict()


def get_user(db: Session, user_id: int = None):
    if not user_id:
        return db.query(models.User).all()
    if res := db.query(models.User).filter_by(id=user_id).first():
        return res
    raise Exception(404, f"{user_id=} not found")


def update_user(db: Session, item: schemas.User, user_id: int):
    return update_to_db(db=db, item_id=user_id, update_item=item, model_cls=models.User)


def delete_user(db: Session, user_id: int):
    db_item = db.query(models.User).filter_by(id=user_id).first()
    if not db_item:
        raise Exception(404, f"{user_id=} not found")
    db.delete(db_item)
    db.commit()
    return "ok"
