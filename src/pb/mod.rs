use crate::{handlers, models};

mod abi; // 声明 abi.rs
pub use abi::*;

impl From<&models::Course> for Course {
    fn from(course: &models::Course) -> Self {
        Course {
            article_count: course.article_count as u32,
            brief: course.brief.to_owned(),
            done: course.done > 0,
            id: course.id.to_owned(),
            image: course.image.to_owned(),
            price: course.price as u32,
            purchased_count: course.purchased_count.to_owned(),
            teacher_name: course.teacher_name.to_owned(),
            teacher_title: course.teacher_title.to_owned(),
            title: course.title.to_owned(),
            sections: vec![],
            description: "".to_owned(),
        }
    }
}

impl From<&Vec<models::Course>> for CourseList {
    fn from(courses: &Vec<models::Course>) -> Self {
        CourseList {
            courses: courses.iter().map(|c| c.into()).collect(),
        }
    }
}

impl From<&(models::Section, Vec<models::Article>)> for Section {
    fn from((section, articles): &(models::Section, Vec<models::Article>)) -> Self {
        Section {
            id: section.id.to_owned(),
            title: section.title.to_owned(),
            articles: articles.into_iter().map(|a| a.into()).collect(),
        }
    }
}

impl From<&models::Section> for Section {
    fn from(section: &models::Section) -> Self {
        Section {
            id: section.id.to_owned(),
            title: section.title.to_owned(),
            articles: vec![],
        }
    }
}

impl From<&models::Article> for Article {
    fn from(article: &models::Article) -> Self {
        Article {
            id: article.id.to_owned(),
            done: article.done > 0,
            publish_date: article.publish_date.to_owned(),
            title: article.title.to_owned(),
            content: "".to_owned(),
            course: None,
            section: None,
        }
    }
}

impl From<&models::ArticleComment> for Comment {
    fn from(comment: &models::ArticleComment) -> Self {
        Comment {
            content: comment.content.to_owned(),
            like_count: comment.like_count as u32,
            nick_name: comment.nick_name.to_owned(),
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
