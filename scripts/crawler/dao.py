"""
数据访问层
"""

import aiosqlite
from pathlib import Path
from typing import Optional, List
import logging
import asyncio

logger = logging.getLogger(__name__)


class DAO:
    """数据访问对象"""

    def __init__(self, db_path: str = "storage/hackbook.db"):
        # 转换为绝对路径（相对于项目根目录）
        base_dir = Path(__file__).parent.parent.parent
        self.db_dir = base_dir / "storage"
        self.db_path = str(self.db_dir / "hackbook.db")
        self.storage_contents = base_dir / "storage" / "contents"
        self.storage_courses = base_dir / "storage" / "courses"

        # 确保目录存在
        self.db_dir.mkdir(parents=True, exist_ok=True)
        self.storage_contents.mkdir(parents=True, exist_ok=True)
        self.storage_courses.mkdir(parents=True, exist_ok=True)

        # 全局连接（aiosqlite 是线程安全的，支持并发）
        self._conn: Optional[aiosqlite.Connection] = None
        self._conn_lock = asyncio.Lock()

        # 缓存已存在的 ID
        self._cached_article_content_ids: set[str] = set()

    async def _get_connection(self) -> aiosqlite.Connection:
        """获取数据库连接（单例模式）"""
        async with self._conn_lock:
            if self._conn is None:
                self._conn = await aiosqlite.connect(self.db_path)
                # 启用 WAL 模式以支持更好的并发
                await self._conn.execute("PRAGMA journal_mode=WAL")
                await self._conn.execute("PRAGMA busy_timeout=30000")  # 30秒超时
                await self._conn.commit()
            return self._conn

    async def close(self):
        """关闭数据库连接"""
        async with self._conn_lock:
            if self._conn is not None:
                await self._conn.close()
                self._conn = None

    async def init_cache(self):
        """初始化缓存"""
        conn = await self._get_connection()
        cursor = await conn.cursor()
        await cursor.execute("SELECT id FROM article")
        self._cached_article_content_ids = {row[0] for row in await cursor.fetchall()}
        await cursor.close()
        logger.info(f"Cached {len(self._cached_article_content_ids)} article content IDs")

    def has_article_content(self, article_id: str) -> bool:
        """检查文章内容是否已存在"""
        # 检查文件是否存在
        content_file = self.storage_contents / f"{article_id}.html"
        return content_file.exists()

    async def save_course(self, course: dict) -> bool:
        """保存课程（只新增不更新，避免覆盖 done 标记）"""
        conn = await self._get_connection()
        try:
            cursor = await conn.cursor()
            # 检查是否已存在
            await cursor.execute("SELECT id FROM course WHERE id = ?", (course["id"],))
            if await cursor.fetchone():
                logger.debug(f"Course {course['id']} already exists, skipping")
                await cursor.close()
                return False

            await cursor.execute(
                """
                INSERT INTO course
                (id, title, teacherName, teacherTitle, brief, image, articleCount, purchasedCount, price, done)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
                (
                    course["id"],
                    course.get("title", ""),
                    course.get("teacherName", ""),
                    course.get("teacherTitle", ""),
                    course.get("brief", ""),
                    course.get("image", ""),
                    course.get("articleCount", 0),
                    course.get("purchasedCount", ""),
                    course.get("price", 0),
                    course.get("done", 0),
                ),
            )
            await conn.commit()
            await cursor.close()
            logger.debug(f"Course {course['id']} inserted")
            return True
        except Exception as e:
            logger.error(f"Save course error: {e}")
            return False

    async def save_section(self, section: dict) -> bool:
        """保存章节"""
        conn = await self._get_connection()
        try:
            cursor = await conn.cursor()
            # 检查是否存在
            await cursor.execute("SELECT id FROM section WHERE id = ?", (section["id"],))
            if await cursor.fetchone():
                await cursor.close()
                return False

            await cursor.execute(
                """
                INSERT INTO section (id, courseId, title)
                VALUES (?, ?, ?)
            """,
                (section["id"], section["courseId"], section["title"]),
            )
            await conn.commit()
            await cursor.close()
            return True
        except Exception as e:
            logger.error(f"Save section error: {e}")
            return False

    async def save_article(self, article: dict) -> bool:
        """保存文章（只新增不更新，避免覆盖 done 标记）"""
        conn = await self._get_connection()
        try:
            cursor = await conn.cursor()
            # 检查是否已存在
            await cursor.execute("SELECT id FROM article WHERE id = ?", (article["id"],))
            if await cursor.fetchone():
                logger.debug(f"Article {article['id']} already exists, skipping")
                await cursor.close()
                return False

            await cursor.execute(
                """
                INSERT INTO article
                (id, title, publishDate, sectionId, done)
                VALUES (?, ?, ?, ?, ?)
            """,
                (
                    article["id"],
                    article.get("title", ""),
                    article.get("publishDate", ""),
                    article.get("sectionId", ""),
                    article.get("done", 0),
                ),
            )
            await conn.commit()
            await cursor.close()
            logger.debug(f"Article {article['id']} inserted")

            # 更新缓存
            self._cached_article_content_ids.add(article["id"])
            return True
        except Exception as e:
            logger.error(f"Save article error: {e}")
            return False

    async def save_article_comment(self, comment: dict) -> bool:
        """保存文章评论（只新增不更新）"""
        conn = await self._get_connection()
        try:
            cursor = await conn.cursor()
            # 检查是否已存在
            await cursor.execute("SELECT id FROM article_comment WHERE id = ?", (comment["id"],))
            if await cursor.fetchone():
                logger.debug(f"Comment {comment['id']} already exists, skipping")
                await cursor.close()
                return False

            await cursor.execute(
                """
                INSERT INTO article_comment
                (id, content, likeCount, nickName, articleId, parentCommentId)
                VALUES (?, ?, ?, ?, ?, ?)
            """,
                (
                    comment["id"],
                    comment.get("content", ""),
                    comment.get("likeCount", 0),
                    comment.get("nickName", ""),
                    comment.get("articleId"),
                    comment.get("parentCommentId"),
                ),
            )
            await conn.commit()
            await cursor.close()
            logger.debug(f"Comment {comment['id']} inserted")
            return True
        except Exception as e:
            logger.error(f"Save comment error: {e}")
            return False

    async def save_course_description(self, course_id: str, content: str):
        """保存课程描述到文件"""
        def _save():
            desc_file = self.storage_courses / f"{course_id}.html"
            with open(desc_file, "w", encoding="utf-8") as f:
                f.write(content)

        loop = asyncio.get_event_loop()
        await loop.run_in_executor(None, _save)

    async def save_article_content(self, article_id: str, content: str):
        """保存文章内容到文件"""
        def _save():
            content_file = self.storage_contents / f"{article_id}.html"
            with open(content_file, "w", encoding="utf-8") as f:
                f.write(content)

        loop = asyncio.get_event_loop()
        await loop.run_in_executor(None, _save)

    async def get_course(self, course_id: str) -> Optional[dict]:
        """获取课程"""
        conn = await self._get_connection()
        cursor = await conn.cursor()
        await cursor.execute(
            "SELECT id, title, teacherName, done FROM course WHERE id = ?", (course_id,)
        )
        row = await cursor.fetchone()
        await cursor.close()
        if row:
            return {"id": row[0], "title": row[1], "teacherName": row[2], "done": row[3]}
        return None

    async def get_section(self, section_id: str) -> Optional[dict]:
        """获取章节"""
        conn = await self._get_connection()
        cursor = await conn.cursor()
        await cursor.execute(
            "SELECT id, courseId, title FROM section WHERE id = ?", (section_id,)
        )
        row = await cursor.fetchone()
        await cursor.close()
        if row:
            return {"id": row[0], "courseId": row[1], "title": row[2]}
        return None

    async def get_all_courses(self) -> List[dict]:
        """获取所有课程"""
        conn = await self._get_connection()
        cursor = await conn.cursor()
        await cursor.execute("SELECT id, title, teacherName FROM course")
        rows = await cursor.fetchall()
        await cursor.close()
        return [
            {"id": row[0], "title": row[1], "teacherName": row[2]}
            for row in rows
        ]


# 全局 DAO 实例
dao = DAO()
