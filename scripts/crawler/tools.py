"""
工具函数模块
"""

import asyncio
import aiohttp
from typing import Callable, Any, Coroutine, TypeVar
from functools import wraps
import logging
import os

T = TypeVar("T")

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)


class AsyncPool:
    """异步并发控制池"""

    def __init__(self, max_concurrent: int = 50):
        self.max_concurrent = max_concurrent
        self.semaphore = asyncio.Semaphore(max_concurrent)
        self.tasks: list[Coroutine] = []

    async def submit(self, coro: Coroutine[Any, Any, T]) -> T:
        """提交异步任务"""
        async with self.semaphore:
            return await coro

    async def submit_sync(self, func: Callable[[], T]) -> T:
        """提交同步函数包装为异步任务"""
        return await self.submit(asyncio.to_thread(func))


class Requester:
    """HTTP 请求器"""

    def __init__(self, headers: dict, max_concurrent: int = 50, timeout: int = 30):
        self.headers = headers
        self.pool = AsyncPool(max_concurrent)
        self.timeout = aiohttp.ClientTimeout(total=timeout)
        self.user_agent_counter = 0

    async def get(self, url: str, **kwargs) -> dict:
        """GET 请求"""
        async with aiohttp.ClientSession(
            headers=self.headers, timeout=self.timeout
        ) as session:
            async with self.pool.semaphore:
                try:
                    async with session.get(url, **kwargs) as response:
                        data = await response.json()
                        if not data:
                            logger.warning(f"no response: {url}")
                        return data
                except Exception as e:
                    logger.error(f"GET {url} error: {e}")
                    return {}

    async def post(self, url: str, json: dict = None, **kwargs) -> dict:
        """POST 请求"""
        headers = self.headers.copy()
        headers["User-Agent"] = str(self.user_agent_counter % 10000)
        headers["Content-Type"] = "application/json"
        self.user_agent_counter += 1

        async with aiohttp.ClientSession(
            headers=headers, timeout=self.timeout, proxy=os.environ.get("http_proxy")
        ) as session:
            async with self.pool.semaphore:
                try:
                    async with session.post(url, json=json, **kwargs) as response:
                        result = await response.json()
                        if not result:
                            logger.warning(f"no response: {url}")
                        return result
                except Exception as e:
                    logger.error(f"POST {url} error: {e}")
                    return {}


def partial(from_obj: dict, to_obj: object, *fields: str) -> object:
    """部分字段复制"""
    for field in fields:
        if hasattr(to_obj, field) and field in from_obj:
            setattr(to_obj, field, from_obj[field])
    return to_obj


def to_geek_id(id: Any) -> str:
    """转换为极客时间 ID"""
    return f"G{id}" if id else ""


def to_lagou_id(id: Any) -> str:
    """转换为拉勾 ID"""
    return f"L{id}" if id else ""
