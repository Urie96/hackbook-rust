use {
    crate::schema::{
        article, article_comment, course, section, user_role, user_study_info, ws_connect_info,
    },
    diesel::{
        prelude::{Associations, Identifiable, Insertable, Queryable, QueryableByName},
        sql_types::BigInt,
    },
};

#[derive(Identifiable, Debug, Queryable, Associations)]
#[diesel(belongs_to(Section, foreign_key = sectionId))]
#[diesel(table_name = article)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Article {
    pub id: String,
    pub done: bool,
    pub publish_date: String,
    #[diesel(column_name = sectionId)]
    pub section_id: String,
    pub title: String,
}

#[derive(Identifiable, Debug, Queryable, Associations)]
#[diesel(belongs_to(Course, foreign_key = courseId))]
#[diesel(table_name = section)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Section {
    pub id: String,
    #[diesel(column_name = courseId)]
    pub course_id: String,
    pub title: String,
}

#[derive(Identifiable, Debug, Queryable, QueryableByName)]
#[diesel(table_name = course)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
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
    pub done: bool,
    pub price: i32,
    pub title: String,
}

#[derive(Identifiable, Debug, Queryable, Associations)]
#[diesel(belongs_to(ArticleComment, foreign_key = parentCommentId))]
#[diesel(table_name = article_comment)]
#[diesel(primary_key(id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
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
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserRole {
    pub user_id: String,
    pub role: i32,
    pub created_at: i64,
    pub valid_before: i64,
    pub id: i32,
}

#[derive(Identifiable, Debug, Queryable, Insertable)]
#[diesel(table_name = user_study_info)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserStudyInfo {
    pub id: i32,
    pub course_id: String,
    pub article_id: String,
    pub last_study_at: i64,
    pub study_percent: f32,
    pub user_id: String,
}

#[derive(Identifiable, Debug, Queryable, Insertable)]
#[diesel(table_name = ws_connect_info)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct WsConnectInfo {
    pub id: i32,
    pub user_id: String,
    pub start_at: i64,
    pub end_at: i64,
}

#[derive(QueryableByName)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ConnectionSecs {
    #[diesel(sql_type = BigInt)]
    pub secs: i64,
}
