from sqlalchemy.orm import Session
import time

from app import schemas, models
from app.db.sql import update_to_db


def create_content(db: Session, item: schemas.CreateContent):
    device_db = db.query(models.Device).filter_by(id=item.device_id).first()
    if not device_db:
        raise Exception(404, f"device_id={item.device_id} not exists")

    db_item = models.ClipboardEntries(
        device_id=item.device_id,
        user_id=device_db.user_id,
        content_type=item.content_type,
        content=item.content,
        created_at=int(time.time())
    )
    db.add(db_item)
    db.commit()
    return db_item.to_dict()


def get_content(db: Session, user_id: int, content_type: int, num_day: int):
    query = db.query(models.ClipboardEntries).filter(models.ClipboardEntries.user_id == user_id)

    if content_type:
        query = query.filter(models.ClipboardEntries.content_type == content_type)
    if num_day:
        query = query.filter(models.ClipboardEntries.created_at > int(time.time()) - num_day * 86400)
    return query.all()
