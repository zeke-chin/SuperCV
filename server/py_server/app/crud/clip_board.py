from sqlalchemy.orm import Session
import time

from app import schemas, models
from app.db.sql import update_to_db


def create_content(db: Session, item: schemas.CreateContent):
    if db.query(models.Device).filter_by(id=item.device_id).first() is None:
        raise Exception(404, f"device_id={item.device_id} not exists")

    db_item = models.ClipboardEntry(
        device_id=item.device_id,
        type=item.type,
        content=item.content,
        path=item.path,
        hash=item.hash,
        timestamp=item.timestamp
    )
    db.add(db_item)
    db.commit()
    return db_item.to_dict()

#
# def get_content(db: Session, user_id: int, content_type: int, num_day: int):
#     query = db.query(models.ClipboardEntries).filter(models.ClipboardEntries.user_id == user_id)
#
#     if content_type:
#         query = query.filter(models.ClipboardEntries.content_type == content_type)
#     if num_day:
#         query = query.filter(models.ClipboardEntries.created_at > int(time.time()) - num_day * 86400)
#     return query.all()
