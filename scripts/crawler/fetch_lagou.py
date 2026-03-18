"""
拉勾教育专栏爬虫
"""
import asyncio
import sys
import os
import re

# 添加父目录到路径
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from tools import Requester, to_lagou_id, partial
from dao import dao
import logging

logger = logging.getLogger(__name__)


# 拉勾教育 Header（需要自行替换）
LAGOU_HEADER = {
    'x-l-req-header': '{deviceType:1}',
    'Cookie': '',  # 请填入你的拉勾 Cookie
    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36'
}


async def save_course_list(requester: Requester):
    """保存课程列表"""
    url = 'https://gate.lagou.com/v1/neirong/edu/homepage/more/getCourseByClassifyV2'
    res = await requester.get(url)

    if not res or 'content' not in res:
        logger.error('Failed to fetch course list')
        return []

    course_list = res['content'].get('courseList', [])
    logger.info(f'拉勾教育课程总数: {len(course_list)}')

    tasks = []
    for c in course_list:
        course_id = str(c['id'])
        lagou_id = to_lagou_id(course_id)

        # 提取购买人数
        purchased_count = ''
        purchased_str = c.get('pruchasedCount', '')
        if purchased_str:
            match = re.search(r'[\d.w]+', purchased_str)
            if match:
                purchased_count = match.group(0)

        # 提取价格
        price = 0
        price_str = c.get('price', '')
        if price_str:
            match = re.search(r'\d+', price_str)
            if match:
                price = int(match.group(0))

        course = {
            'id': lagou_id,
            'title': c.get('title', ''),
            'teacherName': c.get('teacherName', ''),
            'teacherTitle': c.get('teacherTitle', ''),
            'brief': c.get('brief', ''),
            'image': c.get('image', ''),
            'purchasedCount': purchased_count,
            'price': price,
        }

        dao.save_course(course)

        # 异步保存课程详情
        tasks.append(save_course_details(requester, course_id))

        # 限制并发数
        if len(tasks) >= 10:
            await asyncio.gather(*tasks)
            tasks = []

    if tasks:
        await asyncio.gather(*tasks)

    return course_list


async def save_course_description(requester: Requester, course_id: str):
    """保存课程描述"""
    url = f'https://gate.lagou.com/v1/neirong/kaiwu/getCourseDescription?courseId={course_id}'
    data = await requester.get(url)

    if not data or 'content' not in data:
        return

    content = ''
    for item in data['content'].get('courseDescription', []):
        title = item.get('title', '')
        item_type = item.get('type', '')
        item_content = item.get('content', [])

        content += f'<h3>{title}</h3>'

        if item_type == 'image' and item_content:
            content += f'<img src="{item_content[0]}">'
        elif item_type == 'text' and item_content:
            for idx, text in enumerate(item_content, 1):
                content += f'<p>{idx}. {text}</p>'
            if item_content:
                content += f'<img src="{item_content[0]}">'
        elif item_type == 'teacher' and item_content:
            try:
                import json
                teacher_info = json.loads(item_content[0])
                content += teacher_info.get('description', '')
            except:
                content += str(item_content[0])
        else:
            if item_content:
                content += str(item_content[0])

    dao.save_course_description(to_lagou_id(course_id), content)


async def save_course_lessons(requester: Requester, course_id: str):
    """保存课程章节和文章"""
    url = f'https://gate.lagou.com/v1/neirong/kaiwu/getCourseLessons?courseId={course_id}'
    res = await requester.get(url)

    if not res or 'content' not in res:
        logger.warning(f'Failed to fetch lessons for course {course_id}')
        return

    sections = res['content'].get('courseSectionList', [])
    if not sections:
        return

    tasks = []
    for section_data in sections:
        section_id = to_lagou_id(section_data['id'])
        section = {
            'id': section_id,
            'courseId': to_lagou_id(course_id),
            'title': section_data.get('sectionName', ''),
        }

        dao.save_section(section)

        # 处理课程中的文章
        for lesson in section_data.get('courseLessons', []):
            lesson_id = to_lagou_id(lesson['id'])
            article = {
                'id': lesson_id,
                'sectionId': section_id,
                'title': lesson.get('theme', ''),
                'publishDate': '',
            }

            dao.save_article(article)

            # 异步保存文章内容
            tasks.append(save_lesson_content(requester, article, lesson['id']))

    # 限制并发数
    for i in range(0, len(tasks), 20):
        batch = tasks[i:i+20]
        await asyncio.gather(*batch)
        logger.info(f'Completed batch {i//20 + 1}/{(len(tasks) + 19)//20}')


async def save_lesson_content(requester: Requester, article: dict, lesson_id: str):
    """保存课程内容"""
    article_id = article['id']

    if dao.has_article_content(article_id):
        return

    url = f'https://gate.lagou.com/v1/neirong/kaiwu/getCourseLessonDetail?lessonId={lesson_id}'
    res = await requester.get(url)

    if not res or 'content' not in res:
        logger.warning(f'{article_id}: failed to fetch content')
        return

    content_data = res['content']
    publish_date = content_data.get('publishDate', '')
    text_content = content_data.get('textContent', '')

    if publish_date:
        article['publishDate'] = publish_date
        dao.save_article(article)

    if not text_content:
        logger.info(f'{article_id}: empty text content')
        return

    dao.save_article_content(article_id, text_content)
    logger.info(f'{article_id}: 爬取成功')

    # 保存评论
    await save_lesson_comments(requester, article, lesson_id)


async def save_lesson_comments(requester: Requester, article: dict, course_id: str, lesson_id: str):
    """保存课程评论"""
    url = f'https://gate.lagou.com/v1/neirong/course/comment/getCourseCommentList?courseId={course_id}&lessonId={lesson_id}&pageNum=1'
    data = await requester.get(url)

    if not data or 'content' not in data:
        return

    comment_list = data['content'].get('courseCommentList', [])
    for comment in comment_list:
        ac = {
            'id': to_lagou_id(comment['commentId']),
            'content': comment.get('comment', ''),
            'likeCount': int(comment.get('likeCount', 0)),
            'nickName': comment.get('nickName', ''),
            'articleId': article['id'],
        }

        dao.save_article_comment(ac)

        # 保存回复
        reply = comment.get('replayComment')
        if reply:
            r = {
                'id': to_lagou_id(reply['commentId']),
                'content': reply.get('comment', ''),
                'likeCount': int(reply.get('likeCount', 0)),
                'nickName': reply.get('nickName', ''),
                'articleId': article['id'],
                'parentCommentId': ac['id'],
            }
            dao.save_article_comment(r)


async def save_course_details(requester: Requester, course_id: str):
    """保存课程详情（描述和内容）"""
    await save_course_description(requester, course_id)
    await save_course_lessons(requester, course_id)


async def main():
    """主函数"""
    # 初始化 DAO
    dao.init_cache()

    requester = Requester(LAGOU_HEADER, max_concurrent=50)

    # 保存课程列表和详情
    await save_course_list(requester)

    logger.info('All courses completed')


if __name__ == '__main__':
    asyncio.run(main())
