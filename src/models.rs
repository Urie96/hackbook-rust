use {
    crate::schema::{
        article, article_comment, article_content, course, course_description, section, user_role,
        user_study_info, ws_connect_info,
    },
    diesel::{
        prelude::{Associations, Identifiable, Insertable, Queryable, QueryableByName},
        sql_types::{BigInt, Unsigned},
    },
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

#[derive(Identifiable, Debug, Queryable, QueryableByName)]
#[diesel(table_name = course)]
pub struct Course {
    pub id: String,
    pub brief: String,
    #[diesel(column_name = teacherName)]
    pub teacher_name: String,
    #[diesel(column_name = teacherTitle)]
    pub teacher_title: String,
    pub image: String,
    #[diesel(column_name = articleCount)]
    pub article_count: i32,
    #[diesel(column_name = purchasedCount)]
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

#[derive(Identifiable, Debug, Queryable, Insertable)]
#[diesel(table_name = user_study_info)]
pub struct UserStudyInfo {
    pub id: u32,
    pub course_id: String,
    pub article_id: String,
    pub last_study_at: u64,
    pub study_percent: f32,
    pub user_id: String,
}

#[derive(Identifiable, Debug, Queryable, Insertable)]
#[diesel(table_name = ws_connect_info)]
pub struct WsConnectInfo {
    pub id: i32,
    pub user_id: String,
    pub start_at: u64,
    pub end_at: u64,
}

#[derive(QueryableByName)]
pub struct ConnectionSecs {
    #[diesel(sql_type = Unsigned<BigInt>)]
    pub secs: u64,
}
