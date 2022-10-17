use {
    crate::{models, pb, repo::Repo},
    actix::prelude::*,
    actix_web_actors::ws::CloseReason,
    anyhow::Result,
    rand::{self, rngs::ThreadRng, Rng},
    std::{collections::HashMap, sync::Arc},
};

#[derive(Message)]
#[rtype(result = "()")]
pub enum ServerMessage {
    // Text(String),
    Close(Option<CloseReason>),
}

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub ws_session_id: usize,
    pub user_id: String,
    pub addr: Recipient<ServerMessage>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
    pub start_at: chrono::DateTime<chrono::Utc>,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "Result<pb::Article>")]
pub struct GetArticleDetail {
    pub article_id: String,
    pub session_id: usize,
    pub user_id: String,
}

#[derive(Debug)]
struct SessionInfo {
    user_id: String,
    addr: Recipient<ServerMessage>,
}

pub struct WsServer {
    sessions: HashMap<usize, SessionInfo>,
    rng: ThreadRng,
    repo: Arc<Repo>,
}

impl WsServer {
    pub fn new(repo: Arc<Repo>) -> WsServer {
        WsServer {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
            repo: repo,
        }
    }
}

/// Make actor from `WsServer`
impl Actor for WsServer {
    /// We are going to use simple Context, we just need ability to communicate
    /// with other actors.
    type Context = Context<Self>;
}

/// Handler for Connect message.
impl Handler<Connect> for WsServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
        println!("id {:?} joined", msg.user_id);
        self.sessions.retain(|_, v| {
            let should_disconnect = v.user_id == msg.user_id;
            println!("{},{}", v.user_id, msg.user_id);
            if should_disconnect {
                v.addr.do_send(ServerMessage::Close(Some(CloseReason {
                    code: actix_web_actors::ws::CloseCode::Policy,
                    description: Some("You have logged in elsewhere".to_string()),
                })));
            }
            !should_disconnect
        });

        let session_id = self.rng.gen::<usize>();

        self.sessions.insert(
            session_id,
            SessionInfo {
                user_id: msg.user_id,
                addr: msg.addr,
            },
        );

        println!("login count: {}", self.sessions.len());

        // send id back
        session_id
    }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for WsServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        println!("{:?} disconnected", msg.id);

        // remove address
        if let Some(session) = self.sessions.remove(&msg.id) {
            println!("Removed session");
            println!("login count: {}", self.sessions.len());

            self.repo.save_connect_info(&models::WsConnectInfo {
                id: 0,
                user_id: session.user_id,
                start_at: msg.start_at.timestamp() as u64,
                end_at: chrono::Utc::now().timestamp() as u64,
            });
        }
    }
}

/// Handler for Disconnect message.
impl Handler<GetArticleDetail> for WsServer {
    type Result = Result<pb::Article>;

    fn handle(&mut self, msg: GetArticleDetail, _: &mut Context<Self>) -> Self::Result {
        if self
            .sessions
            .iter()
            .find(|(&session_id, _)| session_id == msg.session_id)
            .is_some()
        {
            let (article, content) = self.repo.get_article_detail(&msg.article_id)?;
            let section = self.repo.find_section_by_id(&article.section_id)?;
            let course = self.repo.find_course_by_id(&section.course_id)?;
            let mut res: pb::Article = article.into();
            res.content = content.content;
            res.section = Some(section.into());
            res.course = Some(course.into());
            if let Ok(info) = self.repo.find_user_study_info(&msg.user_id, "", &res.id) {
                res.study_info = info.first().and_then(|info| {
                    Some(pb::StudyInfo {
                        percent: info.study_percent,
                        last_study_at: info.last_study_at,
                    })
                });
            }

            Ok(res)
        } else {
            Err(anyhow::anyhow!("Invalid session id"))
        }
    }
}
