use {
    crate::models,
    anyhow::Result,
    diesel::{
        mysql::MysqlConnection,
        prelude::*,
        r2d2::{self, ConnectionManager},
    },
    std::env,
};

#[derive(Clone)]
pub struct Repo {
    pool: r2d2::Pool<ConnectionManager<MysqlConnection>>,
}

impl Repo {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<MysqlConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Repo { pool }
    }

    pub fn find_section_by_id(&self, id: &str) -> Result<models::Section> {
        use crate::schema::section::dsl;

        let conn = &mut self.pool.get()?;

        Ok(dsl::section
            .filter(dsl::id.eq(id))
            .first::<models::Section>(conn)?)
    }

    pub fn find_course_by_id(&self, id: &str) -> Result<models::Course> {
        use crate::schema::course::dsl;

        let conn = &mut self.pool.get()?;

        Ok(dsl::course
            .filter(dsl::id.eq(id))
            .first::<models::Course>(conn)?)
    }

    pub fn get_course_detail_by_course_id(
        &self,
        course_id: &str,
    ) -> Result<(
        models::Course,
        Vec<(models::Section, Vec<models::Article>)>,
        Option<String>,
    )> {
        let conn = &mut self.pool.get()?;
        use crate::schema::course;

        let one_course = course::dsl::course
            .filter(course::dsl::id.eq(course_id))
            .first::<models::Course>(conn)?;

        let sections = models::Section::belonging_to(&one_course).load::<models::Section>(conn)?;

        let desc = models::CourseDescription::belonging_to(&one_course)
            .first::<models::CourseDescription>(conn)
            .ok()
            .map(|c| c.content);

        let articles = models::Article::belonging_to(&sections)
            .load::<models::Article>(conn)?
            .grouped_by(&sections);
        let data = sections.into_iter().zip(articles).collect::<Vec<_>>();

        Ok((one_course, data, desc))
    }

    pub fn list_course(
        &self,
        keyword: &str,
        offset: i64,
        limit: i64,
    ) -> Result<Vec<models::Course>> {
        use crate::schema::course::dsl;

        let conn = &mut self.pool.get()?;
        // let a=dsl::course;
        // a.filter(predicate)

        let res = dsl::course
            .filter(dsl::title.like(format!("%{}%", keyword)))
            .offset(offset)
            .limit(limit)
            .load::<models::Course>(conn)?;

        Ok(res)
    }

    pub fn get_article_detail(
        &self,
        id: &str,
    ) -> Result<(models::Article, models::ArticleContent)> {
        use crate::schema::article::dsl;

        let conn = &mut self.pool.get().unwrap();

        let art = dsl::article
            .filter(dsl::id.eq(id))
            .first::<models::Article>(conn)?;
        let content =
            models::ArticleContent::belonging_to(&art).first::<models::ArticleContent>(conn)?;

        Ok((art, content))
    }

    pub fn find_comments_by_article_id(
        &self,
        article_id: &str,
    ) -> Result<Vec<(Vec<models::ArticleComment>, models::ArticleComment)>> {
        use crate::schema::article_comment::dsl;

        let conn = &mut self.pool.get()?;

        let comments = dsl::article_comment
            .filter(dsl::articleId.eq(article_id))
            .load::<models::ArticleComment>(conn)?;

        let replies = models::ArticleComment::belonging_to(&comments)
            .load::<models::ArticleComment>(conn)?
            .grouped_by(&comments);

        let data = replies.into_iter().zip(comments).collect::<Vec<_>>();

        Ok(data)
    }

    pub fn find_user_role(&self, user_id: &str) -> Result<Option<models::UserRole>> {
        use crate::schema::user_role::dsl;

        let conn = &mut self.pool.get()?;

        let now = chrono::Utc::now().naive_utc().timestamp().unsigned_abs();

        let one_role = dsl::user_role
            .filter(dsl::user_id.eq(user_id))
            .filter(dsl::valid_before.gt(now))
            .first::<models::UserRole>(conn)
            .optional()?;
        Ok(one_role)
    }
}
