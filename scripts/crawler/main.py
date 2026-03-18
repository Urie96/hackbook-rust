"""
爬虫主入口
使用方法:
    python main.py geektime      # 爬取极客时间专栏
    python main.py lagou         # 爬取拉勾教育专栏
    python main.py geektime 100  # 爬取极客时间指定专栏 ID
    python main.py lagou 123      # 爬取拉勾教育指定课程 ID
"""

import asyncio
import sys
import os
from typing import Optional

# 添加当前目录到路径
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

from dao import dao
from fetch_geektime import fetch_column, Requester, GEEKTIME_HEADER
from fetch_lagou import save_course_details, Requester as LagouRequester, LAGOU_HEADER


async def fetch_geektime(course_id: Optional[str] = None):
    """爬取极客时间"""
    dao.init_cache()
    requester = Requester(GEEKTIME_HEADER, max_concurrent=50)

    if course_id:
        # 爬取指定专栏
        print(f"爬取极客时间专栏: {course_id}")
        await fetch_column(requester, course_id, fetch_content=True)
    else:
        # 爬取所有专栏
        from fetch_geektime import get_all_product_ids

        print("开始爬取极客时间所有专栏...")
        course_ids = await get_all_product_ids(requester)

        for i, cid in enumerate(course_ids, 1):
            print(f"[{i}/{len(course_ids)}] 爬取专栏: {cid}")
            await fetch_column(requester, cid, fetch_content=True)


async def fetch_lagou(course_id: Optional[str] = None):
    """爬取拉勾教育"""
    dao.init_cache()
    requester = LagouRequester(LAGOU_HEADER, max_concurrent=50)

    if course_id:
        # 爬取指定课程
        print(f"爬取拉勾教育课程: {course_id}")
        await save_course_details(requester, course_id)
    else:
        # 爬取所有课程
        from fetch_lagou import save_course_list

        print("开始爬取拉勾教育所有课程...")
        await save_course_list(requester)


def main():
    """主函数"""
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    platform = sys.argv[1].lower()
    course_id = sys.argv[2] if len(sys.argv) > 2 else None

    if platform == "geektime":
        asyncio.run(fetch_geektime(course_id))
    elif platform == "lagou":
        asyncio.run(fetch_lagou(course_id))
    else:
        print(f"不支持的平台: {platform}")
        print(__doc__)
        sys.exit(1)


if __name__ == "__main__":
    main()
