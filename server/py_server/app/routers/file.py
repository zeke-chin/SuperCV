from fastapi import APIRouter, File, UploadFile
from sqlalchemy.orm import Session
from fastapi_pagination import Params, paginate
from app.db.sql import get_db
from utils.nlp_web import web_try
from app import schemas, crud, models

router_file = APIRouter(
    prefix="/file",
    tags=["file-文件管理"],
)


@router_file.post('/{user_id}', summary="minio上传文件")
@web_try()
def upload_minio_file(user_id: int, file: UploadFile = File(...)):
    return crud.upload_minio_file(user_id, file)


@router_file.get("/{uri:path}", summary="获取文件")
def get_file(uri):
    return crud.get_minio_file(uri)
