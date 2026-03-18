"""
极客时间专栏爬虫
"""

import asyncio
import sys
import os

# 添加父目录到路径
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from tools import Requester, to_geek_id
from dao import dao
import logging

logger = logging.getLogger(__name__)

phone = [
    17713541424,
    17828228827,
    15196473896,
    15732933533,
    18228580670,
    13875080072,
    17336575801,
    15623836152,
    13114516778,
    15927916425,
    18975667456,
    13260001778,  # +1996
]

# 极客时间 Cookie（需要自行替换）
GEEKTIME_HEADER = {
    "Origin": "https://time.geekbang.org",
    "Cookie": "",  # 请填入你的极客时间 Cookie
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
}


async def get_all_product_ids(requester: Requester) -> list[str]:
    """获取所有专栏 ID"""
    ids = []
    page_index = 0
    has_more = True

    while has_more:
        url = "https://time.geekbang.org/serv/v4/pvip/product_list"
        data = await requester.post(
            url,
            {
                "prev": page_index,
                "product_form": 1,  # 图文/音频
                "product_type": 1,  # 体系课
                "pvip": 0,
                "size": 20,
                "sort": 1,
            },
        )

        if not data or "data" not in data:
            logger.error(f"Failed to fetch product list at page {page_index}")
            break

        has_more = data["data"].get("page", {}).get("more", False)
        for item in data["data"].get("products", []):
            ids.append(str(item["id"]))
        page_index += 1
        logger.info(f"Fetched page {page_index - 1}, total columns: {len(ids)}")

    logger.info(f"极客时间专栏总数: {len(ids)}")
    return ids


async def save_course_info(requester: Requester, course_id: str):
    """保存课程信息"""
    url = "https://time.geekbang.org/serv/v1/column/intro"
    data = await requester.post(url, {"cid": course_id})

    if not data or "data" not in data:
        logger.warning(f"Failed to fetch course info for {course_id}")
        return

    v = data["data"]
    course = {
        "id": to_geek_id(course_id),
        "title": v.get("column_title", ""),
        "teacherName": v.get("author_name", ""),
        "teacherTitle": v.get("author_intro", ""),
        "price": v.get("column_price", 0) / 100,
        "brief": v.get("column_subtitle", ""),
        "image": v.get("lecture_url", ""),
        "articleCount": v.get("article_count", 0),
        "purchasedCount": v.get("sub_count", 0),
    }

    await dao.save_course(course)

    # 保存课程描述
    description = v.get("column_intro", "")
    await dao.save_course_description(course["id"], description)

    logger.info(f"Course saved: {course['title']}")


async def save_sections(requester: Requester, course_id: str):
    """保存章节"""
    url = "https://time.geekbang.org/serv/v1/chapters"
    res = await requester.post(url, {"cid": course_id})

    if not res or "data" not in res:
        logger.warning(f"Failed to fetch sections for course {course_id}")
        return

    for chapter in res["data"]:
        section = {
            "id": to_geek_id(chapter["id"]),
            "title": chapter.get("title", ""),
            "courseId": to_geek_id(course_id),
        }

        # 检查是否已存在
        existing = await dao.get_section(section["id"])
        if not existing:
            await dao.save_section(section)


async def save_articles(requester: Requester, course_id: str):
    """保存文章"""
    url = "https://time.geekbang.org/serv/v1/column/articles"
    res = await requester.post(
        url,
        {
            "cid": course_id,
            "order": "earliest",
            "prev": 0,
            "sample": False,
            "size": 500,
        },
    )

    if not res or "data" not in res:
        logger.warning(f"Failed to fetch articles for course {course_id}")
        return

    article_list = res["data"].get("list", [])
    if not article_list:
        logger.info(f"No articles in course {course_id}")
        return

    for v in article_list:
        article_id = to_geek_id(v["id"])
        if dao.has_article_content(article_id):
            continue

        # 处理章节关联
        chapter_id = v.get("chapter_id", "0")
        if chapter_id == "0":
            # 没有章节，创建孤儿章节
            section_id = to_geek_id(course_id) + "ORPHAN"
            existing = await dao.get_section(section_id)
            if not existing:
                section = {
                    "id": section_id,
                    "courseId": to_geek_id(course_id),
                    "title": "",
                }
                await dao.save_section(section)
        else:
            section_id = to_geek_id(chapter_id)

        # 转换时间戳
        article_ctime = v.get("article_ctime", 0)
        from datetime import datetime

        publish_date = (
            datetime.fromtimestamp(article_ctime).strftime("%Y-%m-%d")
            if article_ctime
            else ""
        )

        article = {
            "id": article_id,
            "sectionId": section_id,
            "title": v.get("article_title", ""),
            "publishDate": publish_date,
        }

        await dao.save_article(article)

        # 保存文章内容
        await save_article_content(requester, article, v["id"])


async def save_article_content(requester: Requester, article: dict, raw_id: str):
    """保存文章内容"""
    article_id = article["id"]

    if dao.has_article_content(article_id):
        return

    url = "https://time.geekbang.org/serv/v1/article"
    res = await requester.post(url, {"id": raw_id, "is_freelyread": True})

    if not res:
        logger.info(f"{article_id}: empty response")
        return

    # 检查是否有错误（未购买）
    error = res.get("error")
    if not isinstance(error, list):
        logger.info(f"{article_id}: {error}")
        return

    if "data" not in res:
        logger.warning(f"{article_id}: no data in response")
        return

    content = res["data"].get("article_content", "")
    if content == "":
        logger.warning(f"{article_id}: empty content")
        return
    await dao.save_article_content(article_id, content)
    logger.info(f"{article_id}: 爬取成功")

    # 保存评论
    await save_comments(requester, article, raw_id)


async def save_comments(requester: Requester, article: dict, raw_id: str):
    """保存评论"""
    url = "https://time.geekbang.org/serv/v1/comments"
    res = await requester.post(url, {"aid": raw_id, "prev": 0})

    if not res or "data" not in res:
        return

    for v in res["data"].get("list", []):
        comment = {
            "id": to_geek_id(v["id"]),
            "nickName": v.get("user_name", ""),
            "likeCount": v.get("like_count", 0),
            "content": v.get("comment_content", ""),
            "articleId": article["id"],
        }

        await dao.save_article_comment(comment)

        # 保存回复
        for reply in v.get("replies", []):
            reply_comment = {
                "id": to_geek_id(reply["id"]),
                "nickName": reply.get("user_name", ""),
                "content": reply.get("content", ""),
                "likeCount": reply.get("like_count", 0),
                "articleId": article["id"],
                "parentCommentId": comment["id"],
            }
            await dao.save_article_comment(reply_comment)

    logger.info(f"{raw_id}: comments saved")


async def fetch_column(
    requester: Requester, course_id: str, fetch_content: bool = True
):
    """爬取单个专栏"""
    geek_id = to_geek_id(course_id)

    # 检查专栏是否已经完成
    course = await dao.get_course(geek_id)
    if course and course.get("done") == 1:
        logger.info(f"Column {course_id} is already done, skipping")
        return

    logger.info(f"Starting to fetch column: {course_id}")

    await save_course_info(requester, course_id)
    await save_sections(requester, course_id)

    if fetch_content:
        await save_articles(requester, course_id)

    logger.info(f"Column {course_id} completed")


async def main():
    """主函数"""
    try:
        # 初始化 DAO
        await dao.init_cache()

        requester = Requester(GEEKTIME_HEADER, max_concurrent=50)

        # 获取所有专栏 ID
        course_ids = await get_all_product_ids(requester)

        # 可以指定爬取特定专栏，如 course_ids[0:3]
        # course_ids = course_ids[0:3]

        # 爬取所有专栏
        tasks = []
        for course_id in course_ids:
            tasks.append(fetch_column(requester, course_id, fetch_content=True))

        # 限制并发数
        for i in range(0, len(tasks), 5):
            batch = tasks[i : i + 5]
            await asyncio.gather(*batch)
            logger.info(f"Completed batch {i // 5 + 1}/{(len(tasks) + 4) // 5}")
    finally:
        # 关闭数据库连接
        await dao.close()


if __name__ == "__main__":
    asyncio.run(main())
