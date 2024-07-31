from sqlalchemy.orm import Session
import time

from app import schemas, models
from app.db.sql import update_to_db


def register_user(db: Session, item: schemas.UserRegister):
    if db.query(models.User).filter_by(username=item.username).first():
        raise Exception(409, f"username={item.username} already exists")
    if db.query(models.User).filter_by(email=item.email).first():
        raise Exception(409, f"email={item.email} already exists")

    user = models.User(**item.dict(), **{
        "created_at": time.time(),
        "updated_at": time.time()
    }
                       )

    db.add(user)
    db.commit()
    return user.to_dict()


def login_user(db: Session, item: schemas.UserLogin):
    if db_item := db.query(models.User).filter_by(username=item.username).first():
        if db_item.password_hash == item.password_hash:
            return db_item.to_dict()
        else:
            raise Exception(401, f"password error")
    raise Exception(404, f"{item.username=} not found")


def reset_password_user(db: Session, item: schemas.UserResetPassword):
    if db_item := db.query(models.User).filter_by(username=item.username).first():
        if db_item.email == item.email:
            update_to_db(db=db, item_id=db_item.id, update_item=item, model_cls=models.User)
        else:
            raise Exception(401, f"email error")
    else:
        raise Exception(404, f"{item.username=} not found")
