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
    std::sync::Arc,
    time::Duration,
};

mod errors;
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

    info!("starting HTTP server at http://localhost:8080");
    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(repo_instance.clone()))
            .app_data(Data::new(server.clone()))
            .app_data(JsonConfig::default().limit(4096))
            .wrap(middleware::Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new("1996".repeat(8).as_bytes())
                    .name("auth")
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
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
