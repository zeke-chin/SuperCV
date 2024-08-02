from sqlalchemy.orm import Session
import time

from sqlalchemy import and_
from collections import defaultdict

from app import schemas, models
from app.db.sql import update_to_db


def create_device(db: Session, item: schemas.CreateDevice):
    # 判断uuid是否已经存在
    if db.query(models.Device).filter(models.Device.uuid == item.uuid).first():
        raise Exception(409, f"device uuid={item.uuid} already exists")
    if db.query(models.User).filter_by(id=item.user_id).first() is None:
        raise Exception(404, f"user_id={item.user_id} not exists")

    db_item = models.Device(**item.dict(), **{
        "status": 0,
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


def sync_by_device(db: Session, device_id, item: schemas.SyncDevice) -> schemas.SyncDeviceResult:
    device = db.query(models.Device).filter(models.Device.id == item.device_id).first()
    if not device:
        raise Exception(404, f"{device_id=} not found")

    # 使用一次查询获取所有需要的数据
    server_entries = db.query(models.ClipboardEntry).filter(
        and_(
            models.ClipboardEntry.device_id == device.id,
            models.ClipboardEntry.timestamp >= item.start_at,
            models.ClipboardEntry.timestamp <= item.end_at
        )
    ).all()

    # 创建哈希映射以快速查找服务器条目
    server_hash_map = {entry.hash: entry for entry in server_entries}

    # 创建客户端哈希集合以快速检查存在性
    client_hash_set = set()

    update_client_ids = []
    download_server_ids = []

    # 使用defaultdict来跟踪每个哈希的最新时间戳
    latest_timestamps = defaultdict(int)

    # 处理客户端条目
    for client_item in item.items:
        client_hash_set.add(client_item.hash)
        server_entry = server_hash_map.get(client_item.hash)

        if not server_entry or client_item.timestamp > server_entry.timestamp:
            update_client_ids.append(client_item.client_id)
            latest_timestamps[client_item.hash] = max(latest_timestamps[client_item.hash], client_item.timestamp)
        elif client_item.timestamp < server_entry.timestamp:
            download_server_ids.append(server_entry.id)
            latest_timestamps[client_item.hash] = max(latest_timestamps[client_item.hash], server_entry.timestamp)

    # 检查服务器上独有的条目
    for server_entry in server_entries:
        if server_entry.hash not in client_hash_set:
            download_server_ids.append(server_entry.id)
            latest_timestamps[server_entry.hash] = max(latest_timestamps[server_entry.hash], server_entry.timestamp)

    # 批量更新数据库
    updates = []
    for hash, timestamp in latest_timestamps.items():
        updates.append({
            'hash': hash,
            'timestamp': timestamp
        })

    if updates:
        db.bulk_update_mappings(models.ClipboardEntry, updates)

    # 更新设备的最后同步时间
    device.updated_at = int(time.time())
    db.commit()

    return schemas.SyncDeviceResult(
        update_client_ids=update_client_ids,
        download_server_ids=download_server_ids
    )
