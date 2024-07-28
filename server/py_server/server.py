from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.db.sql import engine, Base
from app import routers

Base.metadata.create_all(bind=engine)
app = FastAPI(title='HostClipboard python server')

# CORS 跨源资源共享
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)
app.include_router(routers.router_user)
app.include_router(routers.router_device)
app.include_router(routers.router_content)
app.include_router(routers.router_file)

print('server init finish:)!!!')


@app.get("/ping", description="健康检查")
async def ping():
    return "pong!!!"
