use {
    crate::ws_server,
    actix::prelude::*,
    actix_web_actors::ws,
    prost::Message,
    std::time::{Duration, Instant},
};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);

/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct WsSession {
    /// unique session id
    pub id: usize,

    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    pub hb: Instant,

    /// Chat server
    pub addr: Addr<ws_server::WsServer>,

    pub user_id: String,

    pub start_at: chrono::DateTime<chrono::Utc>,
}

impl WsSession {
    /// helper method that sends ping to client every 5 seconds (HEARTBEAT_INTERVAL).
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(ws_server::Disconnect {
                    id: act.id,
                    start_at: act.start_at,
                });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with WsServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(ws_server::Connect {
                //ws_session_id: self.id,
                user_id: self.user_id.to_owned(),
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(ws_server::Disconnect {
            id: self.id,
            start_at: self.start_at,
        });
        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<ws_server::ServerMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: ws_server::ServerMessage, ctx: &mut Self::Context) {
        match msg {
            // ws_server::ServerMessage::Text(text) => ctx.text(text),
            ws_server::ServerMessage::Close(reason) => {
                println!("Websocket Client disconnected: {:?}", reason);
                if let Some(reason) = &reason {
                    // js 无法获取close的reason
                    if let Some(desc) = &reason.description {
                        if desc != "" {
                            ctx.text(desc.as_ref());
                        }
                    }
                }
                ctx.close(reason);
            } // ws_server::ServerMessage::Binary(bin) => ctx.binary(bin),
        }
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        log::debug!("WEBSOCKET MESSAGE: {msg:?}");
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();
                // we check for /sss type of messages
                if m.starts_with('/') {
                    let v: Vec<&str> = m.splitn(2, ' ').collect();
                    match v[0] {
                        "/article_detail" => {
                            if v.len() == 2 {
                                // send message to chat server
                                self.addr
                                    .send(ws_server::GetArticleDetail {
                                        article_id: v[1].to_owned(),
                                        session_id: self.id,
                                        user_id: self.user_id.to_owned(),
                                    })
                                    .into_actor(self)
                                    .then(|res, _act, ctx| {
                                        if let Ok(Ok(res)) = res {
                                            ctx.binary(res.encode_to_vec());
                                        } else {
                                            ctx.stop();
                                        }
                                        fut::ready(())
                                    })
                                    .wait(ctx);
                            } else {
                                ctx.text("!!! article id is required");
                            }
                        }
                        _ => ctx.text(format!("!!! unknown command: {m:?}")),
                    }
                } else {
                    ctx.text(text);
                }
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}
