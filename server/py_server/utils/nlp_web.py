import asyncio
import traceback
from contextvars import ContextVar
from functools import wraps

from decorator import decorator
from configs import logger


def json_compatible(data):
    if isinstance(data, dict):
        return {k: json_compatible(v) for k, v in data.items()}
    if isinstance(data, bytes):
        return str(data)
    return data


current_function_name = ContextVar("current_function_name")


# 定义一个上下文管理的装饰器
def set_function_name_in_context(func):
    @wraps(func)
    async def wrapper(*args, **kwargs):
        # 在上下文中设置当前函数名
        token = current_function_name.set(func.__name__)
        try:
            # 调用实际函数
            if asyncio.iscoroutinefunction(func):
                return await func(*args, **kwargs)
            else:
                return func(*args, **kwargs)
        finally:
            # 删除上下文中的当前函数名
            current_function_name.reset(token)

    return wrapper


def web_try(exception_ret=None, append_err=None):
    @decorator
    @set_function_name_in_context
    async def f(func, *args, **kwargs):
        error_code = 200
        ret = None
        error_msg = ''
        func_name = current_function_name.get()

        try:
            # start_time = time.perf_counter()
            if asyncio.iscoroutinefunction(func):
                ret = await func(*args, **kwargs)
            else:
                ret = func(*args, **kwargs)
            # end_time = time.perf_counter()
            # logger.info(f"{func_name} cost time: {format_time(start_time, end_time)}")
        except Exception as e:
            msg = traceback.format_exc()
            append_err_msg = append_err() if callable(append_err) else ""
            if len(e.args) > 0 and isinstance(e.args[0], int):
                error_code = e.args[0]
            else:
                error_code = 400

            logger.error('--------------------------------')
            logger.error(f'Get Exception in web try :( \n{msg}\n{append_err_msg}')
            logger.error('--------------------------------')

            error_msg = msg.split('\n')[-2] if msg != '' else msg
            error_msg = error_msg + " \n " + append_err_msg if append_err_msg else error_msg

            if callable(exception_ret):
                ret = exception_ret()
            else:
                ret = exception_ret
        finally:
            # if ret is not None and isinstance(ret, JSONResponse):
            #     return ret
            return json_compatible(
                {"code": error_code, "data": ret, "error_msg": error_msg})

    return f
