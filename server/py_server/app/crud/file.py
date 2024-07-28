import io
import time
from pathlib import Path

import magic
from fastapi.responses import StreamingResponse

from app.db.minio import FileHandler

FMH = FileHandler("host-clipboard")


def upload_minio_file(user_id: int, file):
    file_byte = file.file.read()
    result = str(user_id) / Path(time.strftime("%Y%m%d", time.localtime()))
    real_path = result / file.filename
    path = FMH.put_file(real_path, file_byte)
    return {'uri': f'/file/{path}'}


def get_minio_file(path: str):
    file_byte = FMH.get_file(path)
    content_type = magic.from_buffer(file_byte, mime=True)
    if file_byte:
        return StreamingResponse(io.BytesIO(file_byte), media_type=content_type)
    else:
        raise Exception(404, f"文件 {path} 不存在")
