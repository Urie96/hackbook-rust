use {
    actix::Actor,
    actix_identity::{CookieIdentityPolicy, IdentityService},
    actix_web::{
        middleware,
        web::{Data, JsonConfig},
        App, HttpServer,
    },
    dotenvy::dotenv,
    log::*,
    std::{env, sync::Arc},
    time::Duration,
};

mod handlers;
mod models;
mod pb;
mod repo;
mod schema;
mod ws_server;
mod ws_session;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let repo_instance = Arc::new(repo::Repo::new());

    // start chat server actor
    let server = ws_server::WsServer::new(repo_instance.clone()).start();

    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port: u16 = env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap();
    info!("starting HTTP server at http://{}:{}", host, port);
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(repo_instance.clone()))
            .app_data(Data::new(server.clone()))
            .app_data(JsonConfig::default().limit(4096))
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new("1996".repeat(8).as_bytes())
                    .name("bookauth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age(Duration::days(7))
                    .secure(false), // this can only be true if you have https
            ))
            .service(handlers::get_course_detail)
            .service(handlers::list_course)
            .service(handlers::get_article_comments)
            .service(handlers::ws_start)
            .service(handlers::login)
            .service(handlers::get_me)
            .service(handlers::save_study_info)
            .service(handlers::get_connect_seconds)
            .service(handlers::test)
    })
    .bind((host, port))?
    .run()
    .await
}
