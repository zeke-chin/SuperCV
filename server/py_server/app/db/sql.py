import time  # database.py
from typing import Optional, Type
from sqlalchemy.orm import Session
from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker
from sqlalchemy.orm.attributes import flag_modified
from configs.settings import config

db_config = config['DATABASE']
HOST = db_config.get('HOST')
PORT = db_config.get('PORT')
USER = db_config.get('USER')
PWD = db_config.get('PWD')
DB_NAME = db_config.get('DB_NAME')

SQLALCHEMY_DATABASE_URI = f"postgresql://{USER}:{PWD}@{HOST}:{PORT}/{DB_NAME}"
engine = create_engine(SQLALCHEMY_DATABASE_URI, echo=False, pool_recycle=3600)

SessionLocal = sessionmaker(autocommit=False, autoflush=False, bind=engine)

Base = declarative_base()


class BaseModel(Base):
    __abstract__ = True

    def to_dict(self):
        return {c.name: getattr(self, c.name) for c in self.__table__.columns}

    def update(self, db: Session, enforce_update: Optional[dict] = None):
        for k in enforce_update or {}:
            flag_modified(self, k)
        if hasattr(self, "update_time"):
            self.update_time = int(time.time())
        # db.add(self)
        db.commit()
        db.flush()
        db.refresh(self)

    def set_field(self, data: Optional[dict] = None):
        for key in self.__class__.__dict__.keys():
            if key in data:
                if data[key] is None:
                    continue
            if not key.startswith('_') and key in data:
                setattr(self, key, data[key])
            if hasattr(self, 'update_time'):
                setattr(self, 'update_time', time.time())


def update_to_db(db: Session, item_id: int, update_item, model_cls: Type[BaseModel],
                 extra: tuple = (), force: int = 0, force_fields: tuple = tuple()):
    db_item = db.query(model_cls).filter(model_cls.id == item_id).first()
    if not db_item:
        raise Exception(404, f'未找到该任务 by model_cls: {model_cls}, item_id: {item_id}')
    update_dict = update_item.dict(exclude_unset=True)
    if len(extra) > 1:
        update_dict[extra[0]] = extra[1]
    for k, v in update_dict.items():
        if not force:  # 更新时想保留原值，值为空即可
            if v is None:
                continue
        else:
            if k not in force_fields and v is None:  # 更新时想要保留原值，1。值为空，2。字段没被指定强改
                continue
        setattr(db_item, k, v)
        flag_modified(db_item, k)

    if hasattr(db_item, "update_time"):
        db_item.update_time = int(time.time())
    db.commit()
    db.flush()
    db.refresh(db_item)
    return db_item


def get_db():
    try:
        db = SessionLocal()
        yield db
    finally:
        db.close()
