use {
    crate::schema::{
        article, article_comment, article_content, course, course_description, section, user_role,
    },
    diesel::prelude::{Associations, Identifiable, Queryable},
};

#[derive(Identifiable, Debug, Queryable, Associations)]
#[diesel(belongs_to(Section, foreign_key = sectionId))]
#[diesel(table_name = article)]
pub struct Article {
    pub id: String,
    pub done: i8,
    pub publish_date: String,
    #[diesel(column_name = sectionId)]
    pub section_id: String,
    pub title: String,
}

#[derive(Identifiable, Debug, Queryable, Associations)]
#[diesel(belongs_to(Course, foreign_key = courseId))]
#[diesel(table_name = section)]
pub struct Section {
    pub id: String,
    #[diesel(column_name = courseId)]
    pub course_id: String,
    pub title: String,
}

#[derive(Identifiable, Debug, Queryable)]
#[diesel(table_name = course)]
pub struct Course {
    pub id: String,
    pub brief: String,
    pub teacher_name: String,
    pub teacher_title: String,
    pub image: String,
    pub article_count: i32,
    pub purchased_count: String,
    pub done: i8,
    pub price: i32,
    pub title: String,
}

#[derive(Identifiable, Debug, Queryable, Associations)]
#[diesel(belongs_to(Course, foreign_key = courseId))]
#[diesel(table_name = course_description)]
#[diesel(primary_key(courseId))]
pub struct CourseDescription {
    #[diesel(column_name = courseId)]
    pub course_id: String,
    pub content: String,
}

#[derive(Identifiable, Debug, Queryable, Associations)]
#[diesel(belongs_to(Article, foreign_key = articleId))]
#[diesel(table_name = article_content)]
#[diesel(primary_key(articleId))]
pub struct ArticleContent {
    #[diesel(column_name = articleId)]
    pub article_id: String,
    pub content: String,
}

#[derive(Identifiable, Debug, Queryable, Associations)]
#[diesel(belongs_to(ArticleComment, foreign_key = parentCommentId))]
#[diesel(table_name = article_comment)]
#[diesel(primary_key(id))]
pub struct ArticleComment {
    pub id: String,
    pub content: String,
    pub like_count: i32,
    pub nick_name: String,
    pub article_id: Option<String>,
    #[diesel(column_name = parentCommentId)]
    pub parent_comment_id: Option<String>,
}

#[derive(Identifiable, Debug, Queryable)]
#[diesel(table_name = user_role)]
pub struct UserRole {
    pub user_id: String,
    pub role: u32,
    pub created_at: u64,
    pub valid_before: u64,
    pub id: u64,
}
