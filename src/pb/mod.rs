use crate::{handlers, models};

mod abi; // 声明 abi.rs
pub use abi::*;

impl From<models::Course> for Course {
    fn from(course: models::Course) -> Self {
        Course {
            article_count: course.article_count as u32,
            brief: course.brief,
            done: course.done,
            id: course.id,
            image: course.image,
            price: course.price as u32,
            purchased_count: course.purchased_count,
            teacher_name: course.teacher_name,
            teacher_title: course.teacher_title,
            title: course.title,
            sections: vec![],
            description: String::new(),
            study_info: None,
        }
    }
}

impl From<(models::Section, Vec<models::Article>)> for Section {
    fn from((section, articles): (models::Section, Vec<models::Article>)) -> Self {
        Section {
            id: section.id,
            title: section.title,
            articles: articles.into_iter().map(|a| a.into()).collect(),
        }
    }
}

impl From<models::Section> for Section {
    fn from(section: models::Section) -> Self {
        Section {
            id: section.id,
            title: section.title,
            articles: vec![],
        }
    }
}

impl From<models::Article> for Article {
    fn from(article: models::Article) -> Self {
        Article {
            id: article.id,
            done: article.done,
            publish_date: article.publish_date,
            title: article.title,
            content: String::new(),
            course: None,
            section: None,
            study_info: None,
        }
    }
}

impl From<models::ArticleComment> for Comment {
    fn from(comment: models::ArticleComment) -> Self {
        Comment {
            content: comment.content,
            like_count: comment.like_count as u32,
            nick_name: comment.nick_name,
            replies: vec![],
        }
    }
}

impl From<handlers::LoggedUser> for UserInfo {
    fn from(user: handlers::LoggedUser) -> Self {
        UserInfo {
            id: user.id,
            role: user.role as i32,
        }
    }
}
