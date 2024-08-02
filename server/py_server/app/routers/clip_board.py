from fastapi import APIRouter, Depends, Query
from sqlalchemy.orm import Session
from fastapi_pagination import Params, paginate
from app.db.sql import get_db
from utils.nlp_web import web_try
from app import schemas, crud, models

router_content = APIRouter(
    prefix="/content",
    tags=["content-剪切板内容管理"],
)


@router_content.post("")
@web_try()
def create_content(item: schemas.CreateContent, db: Session = Depends(get_db)):
    return crud.create_content(db, item)


@router_content.get("/{content_id}")
@web_try()
def get_content(content_id: int, db: Session = Depends(get_db)):
    return crud.get_content(db, content_id)
