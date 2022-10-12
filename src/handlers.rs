use {
    crate::{errors, pb, repo::Repo, ws_server, ws_session},
    actix::Addr,
    actix_identity::Identity,
    actix_protobuf::ProtoBufResponseBuilder as _,
    actix_web::{dev::Payload, get, web, FromRequest, HttpRequest, HttpResponse},
    actix_web_actors::ws,
    log::*,
    ory_kratos_client::apis::{configuration::Configuration, v0alpha2_api::to_session},
    serde::{Deserialize, Serialize},
    std::{
        env,
        future::{ready, Ready},
        time::Instant,
    },
};

#[get("/course/{course_id}")]
async fn get_course_detail(
    repo: web::Data<Repo>,
    course_id: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let repo = repo.into_inner();
    // use web::block to offload blocking Diesel code without blocking server thread
    let (ref course, ref sections, desc) =
        web::block(move || repo.get_course_detail_by_course_id(course_id.as_str()))
            .await?
            .map_err(actix_web::error::ErrorInternalServerError)?;

    let mut c: pb::Course = course.into();
    c.sections = sections.iter().map(|s| s.into()).collect();
    if let Some(d) = desc {
        c.description = d;
    }
    // debug!("{:?}", query);

    HttpResponse::Ok().protobuf(c)
}

#[derive(Debug, Deserialize)]
pub struct ListCourseQuery {
    limit: Option<i64>,
    offset: Option<i64>,
    keyword: Option<String>,
}

#[get("/courses")]
async fn list_course(
    repo: web::Data<Repo>,
    query: web::Query<ListCourseQuery>,
) -> actix_web::Result<HttpResponse> {
    let repo = repo.into_inner();
    // use web::block to offload blocking Diesel code without blocking server thread
    let ref courses = web::block(move || {
        repo.list_course(
            query.keyword.as_ref().unwrap_or(&String::new()),
            query.offset.unwrap_or(0),
            query.limit.unwrap_or(10),
        )
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let c: pb::CourseList = courses.into();
    // debug!("{:?}", query);

    HttpResponse::Ok().protobuf(c)
}

#[get("/article/{article_id}/comments")]
async fn get_article_comments(
    repo: web::Data<Repo>,
    article_id: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    let repo = repo.into_inner();
    // use web::block to offload blocking Diesel code without blocking server thread
    let ref comments = web::block(move || repo.find_comments_by_article_id(article_id.as_str()))
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

    HttpResponse::Ok().protobuf(pb::CommentList {
        comments: comments
            .iter()
            .map(|(replies, comment)| {
                let mut pbc: pb::Comment = comment.into();
                pbc.replies = replies.iter().map(|r| r.into()).collect();
                pbc
            })
            .collect::<Vec<_>>(),
    })
}

#[get("/ws")]
async fn ws_start(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ws_server::WsServer>>,
    logged_user: LoggedUser,
) -> actix_web::Result<HttpResponse> {
    if let UserRole::Reader = logged_user.role {
        debug!("WS connection established");
        ws::start(
            ws_session::WsSession {
                id: 1,
                hb: Instant::now(),
                addr: srv.get_ref().clone(),
                user_id: logged_user.id,
            },
            &req,
            stream,
        )
    } else {
        Err(actix_web::error::ErrorForbidden("Forbidden"))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoggedUser {
    pub id: String,
    pub role: UserRole,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserRole {
    Visitor = 0,
    Reader = 1,
}

impl From<u32> for UserRole {
    fn from(v: u32) -> Self {
        match v {
            x if x == UserRole::Reader as u32 => UserRole::Reader,
            _ => UserRole::Visitor,
        }
    }
}

impl FromRequest for LoggedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<LoggedUser, actix_web::Error>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Ok(identity) = Identity::from_request(req, pl).into_inner() {
            if let Some(user_json) = identity.identity() {
                if let Ok(mut user) = serde_json::from_str::<LoggedUser>(&user_json) {
                    if let Some(repo) = req.app_data::<web::Data<Repo>>() {
                        user.role = get_user_role(repo.get_ref(), user.id.as_str());
                    }
                    return ready(Ok(user));
                }
            }
        }

        ready(Err(errors::ServiceError::Unauthorized.into()))
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginQuery {
    return_to: String,
}

#[get("/login")]
pub async fn login(
    req: HttpRequest,
    id: Identity,
    repo: web::Data<Repo>,
    query: web::Query<LoginQuery>,
) -> actix_web::Result<HttpResponse> {
    let success = HttpResponse::TemporaryRedirect()
        .insert_header(("location", query.return_to.as_str()))
        .finish();
    let failure = HttpResponse::TemporaryRedirect()
        .insert_header((
            "location",
            format!(
                "{}/self-service/login/browser?aal=&refresh=&return_to={}",
                env::var("SSO").unwrap_or("https://sso.lubui.com".to_string()),
                query.return_to
            ),
        ))
        .finish();

    let mut config = Configuration::new();
    config.base_path = "https://sso.lubui.com".to_owned();
    match to_session(
        &config,
        None,
        req.headers().get("cookie").map(|a| a.to_str().unwrap()),
    )
    .await
    {
        Ok(session) => {
            let user_id = session.identity.id;
            let user = LoggedUser {
                id: user_id.to_owned(),
                role: get_user_role(repo.as_ref(), user_id.as_ref()),
            };
            id.remember(serde_json::to_string(&user).unwrap());
            Ok(success)
        }
        Err(e) => {
            error!("11, {:?}", e);
            Ok(failure)
        }
    }
}

#[get("/me")]
pub async fn get_me(logged_user: LoggedUser, id: Identity) -> actix_web::Result<HttpResponse> {
    id.remember(serde_json::to_string(&logged_user).unwrap());
    let u: pb::UserInfo = logged_user.into();
    HttpResponse::Ok().protobuf(u)
}

pub fn get_user_role(repo: &Repo, user_id: &str) -> UserRole {
    if let Ok(Some(user_role)) = repo.find_user_role(user_id) {
        return match user_role.role {
            1 => UserRole::Reader,
            _ => UserRole::Visitor,
        };
    }
    UserRole::Visitor
}
