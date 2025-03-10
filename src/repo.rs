use {
    crate::models,
    anyhow::{Context, Result},
    diesel::{
        connection::SimpleConnection,
        prelude::*,
        r2d2::{ConnectionManager, Pool},
        sql_query,
        sql_types::{BigInt, Integer, VarChar},
        SqliteConnection,
    },
    std::{env, fs, path::PathBuf, time::Duration},
};

pub struct Repo {
    pool: Pool<ConnectionManager<SqliteConnection>>,
    storage_path: PathBuf,
}

#[derive(Debug)]
pub struct ConnectionOptions {
    pub enable_wal: bool,
    pub enable_foreign_keys: bool,
    pub busy_timeout: Option<Duration>,
}

impl diesel::r2d2::CustomizeConnection<SqliteConnection, diesel::r2d2::Error>
    for ConnectionOptions
{
    fn on_acquire(&self, conn: &mut SqliteConnection) -> Result<(), diesel::r2d2::Error> {
        (|| {
            if self.enable_wal {
                conn.batch_execute("PRAGMA journal_mode = WAL; PRAGMA synchronous = NORMAL;")?;
            }
            if self.enable_foreign_keys {
                conn.batch_execute("PRAGMA foreign_keys = ON;")?;
            }
            if let Some(d) = self.busy_timeout {
                conn.batch_execute(&format!("PRAGMA busy_timeout = {};", d.as_millis()))?;
            }
            Ok(())
        })()
        .map_err(diesel::r2d2::Error::QueryError)
    }
}

impl Repo {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let storage = env::var("STORAGE_PATH").expect("STORAGE_PATH must be set");

        let pool = Pool::builder()
            .max_size(2)
            .connection_customizer(Box::new(ConnectionOptions {
                enable_wal: true,
                enable_foreign_keys: true,
                busy_timeout: Some(Duration::from_secs(30)),
            }))
            .build(ConnectionManager::<SqliteConnection>::new(database_url))
            .unwrap();

        let storage_path = PathBuf::from(&storage);
        if !storage_path.is_dir() {
            panic!("{storage} is not a dir");
        }

        Repo { pool, storage_path }
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
        use crate::schema::course;

        let conn = &mut self.pool.get()?;

        let one_course = course::dsl::course
            .filter(course::dsl::id.eq(course_id))
            .first::<models::Course>(conn)?;

        let sections = models::Section::belonging_to(&one_course).load::<models::Section>(conn)?;

        let desc = fs::read_to_string(
            self.storage_path
                .join(format!("courses/{}.html", one_course.id)),
        )
        .ok();

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
        user_id: &str,
    ) -> Result<(Vec<models::Course>, bool)> {
        let conn = &mut self.pool.get()?;
        // let a=dsl::course;
        // a.filter(predicate)

        // let res = dsl::course
        //     .filter(dsl::title.like(format!("%{}%", keyword)))
        //     .offset(offset)
        //     .limit(limit)
        //     .load::<models::Course>(conn)?;
        let mut res = sql_query(
            "
SELECT
    course.*
FROM
    course
    LEFT JOIN (
        SELECT
            course_id,
            MAX(last_study_at) AS last_study_at
        FROM
            user_study_info
        WHERE
            user_id = ?
        GROUP BY
            course_id
        ORDER BY
            2 DESC
    ) AS t ON course.id = t.course_id
WHERE
    title LIKE ?
ORDER BY
    t.last_study_at DESC,
    id
LIMIT
    ?
OFFSET
    ?;
            ",
        )
        .bind::<VarChar, _>(user_id)
        .bind::<VarChar, _>(format!("%{}%", keyword))
        .bind::<Integer, _>((limit + 1) as i32)
        .bind::<Integer, _>(offset as i32)
        .get_results(conn)?;

        if res.len() > limit as usize {
            res.pop();
            return Ok((res, true));
        }

        Ok((res, false))
    }

    pub fn get_article_detail(&self, id: &str) -> Result<(models::Article, String)> {
        use crate::schema::article::dsl;
        let conn = &mut self.pool.get()?;
        let art = dsl::article
            .filter(dsl::id.eq(id))
            .first::<models::Article>(conn)?;
        let file_path = self.storage_path.join(format!("contents/{id}.html"));
        let content = fs::read_to_string(&file_path)
            .context(format!("{} not exist", file_path.to_string_lossy()))?;

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

    pub fn find_user_role(&self, user_id: &str) -> Result<models::UserRole> {
        use crate::schema::user_role::dsl;
        let conn = &mut self.pool.get()?;
        let now = chrono::Utc::now().timestamp();

        let one_role = dsl::user_role
            .filter(dsl::user_id.eq(user_id))
            .filter(dsl::valid_before.gt(now))
            .first::<models::UserRole>(conn)?;
        Ok(one_role)
    }

    pub fn save_study_info(&self, new_study_info: &models::UserStudyInfo) -> Result<()> {
        use crate::schema::user_study_info;
        let conn = &mut self.pool.get()?;
        diesel::replace_into(user_study_info::table)
            .values(new_study_info)
            .execute(conn)?;

        Ok(())
    }

    pub fn test(&self) -> Result<()> {
        use crate::schema::user_study_info::{self, dsl};
        let conn = &mut self.pool.get()?;
        let mut sql = user_study_info::table
            .filter(dsl::user_id.eq("0698edd5-1ea8-4493-9092-003c4230516a"))
            .into_boxed();

        let article_id = "G183";
        let course_id = "G100002201";

        if !article_id.is_empty() {
            sql = sql.filter(dsl::article_id.eq(article_id));
        } else {
            sql = sql.filter(dsl::course_id.eq(course_id));
        }
        let res = sql.load::<models::UserStudyInfo>(conn)?;
        println!("{:?}", res);

        Ok(())
    }

    pub fn find_user_study_info(
        &self,
        user_id: &str,
        course_id: &str,
        article_id: &str,
    ) -> Result<Vec<models::UserStudyInfo>> {
        use crate::schema::user_study_info::{self, dsl};

        let conn = &mut self.pool.get()?;
        let mut sql = user_study_info::table
            .filter(dsl::user_id.eq(user_id))
            .into_boxed();

        if !article_id.is_empty() {
            sql = sql.filter(dsl::article_id.eq(article_id));
        } else {
            sql = sql.filter(dsl::course_id.eq(course_id));
        }
        let res = sql.load::<models::UserStudyInfo>(conn)?;

        Ok(res)
    }

    pub fn save_connect_info(&self, new_connect_info: &models::WsConnectInfo) -> Result<()> {
        use crate::schema::ws_connect_info;

        let conn = &mut self.pool.get()?;
        diesel::insert_into(ws_connect_info::table)
            .values(new_connect_info)
            .execute(conn)?;

        Ok(())
    }

    pub fn get_connect_seconds(
        &self,
        user_id: &str,
        start_at_gt: i64,
        start_at_lt: i64,
    ) -> Result<i64> {
        let conn = &mut self.pool.get()?;
        let res = sql_query(
            "
SELECT
    IFNULL(SUM(s), 0) AS secs
FROM
    (
        SELECT
            end_at - start_at AS s
        FROM
            ws_connect_info
        WHERE
            user_id = ?
            AND start_at BETWEEN ?
            AND ?
    ) t;
            ",
        )
        .bind::<VarChar, _>(user_id)
        .bind::<BigInt, _>(start_at_gt)
        .bind::<BigInt, _>(start_at_lt)
        .get_result::<models::ConnectionSecs>(conn)?;

        Ok(res.secs)
    }
}
