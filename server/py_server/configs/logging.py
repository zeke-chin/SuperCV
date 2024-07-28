import logging
import sys
from datetime import datetime
from loguru import logger
from configs import root_path, config

# 设置日志级别
LEVEL = config["LOG"].get("level", "INFO")

# 设置日志路径
date_str = datetime.now().strftime('%Y-%m-%d')
logger_path = root_path / "logs" / f"log_{date_str}.log"
error_logger_path = root_path / "logs" / f"error_{date_str}.log"

# 添加日志处理器
logger.add(
    logger_path,
    level=LEVEL,
    encoding="utf-8",
    rotation="5 MB",
    retention="10 days",
    enqueue=True,
    backtrace=True,
    diagnose=True,
)

logger.add(
    error_logger_path,
    level="ERROR",
    encoding="utf-8",
    rotation="5 MB",
    retention="10 days",
    enqueue=True,
    backtrace=True,
    diagnose=True,
)

# 从标准输出移除默认处理器并添加新的处理器
logger.remove(0)
logger.add(sys.stderr, level=LEVEL)


class InterceptHandler(logging.Handler):
    def emit(self, record):
        # 获取Loguru对应的日志等级
        level = logger.level(record.levelname).name if logger.level(record.levelname) else "INFO"
        logger.opt(depth=6, exception=record.exc_info).log(level, record.getMessage())


# 配置标准日志记录器，使其输出经过Loguru
handler = InterceptHandler()

# 要拦截的日志记录器列表
loggers = ["uvicorn", "sqlalchemy"]
for name in loggers:
    logging_logger = logging.getLogger(name)
    logging_logger.handlers = [handler]
    logging_logger.propagate = False
