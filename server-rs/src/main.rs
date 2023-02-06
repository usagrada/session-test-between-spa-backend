use actix_cors::Cors;
use actix_session::{config::PersistentSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie::{self, Key},
    http,
    middleware::Logger,
    web, App, HttpServer,
};
use db::create_db_pool;
use dotenv::dotenv;

mod db;
mod models;
use models::*;
mod routes;
use routes::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    dotenv().ok();
    let pool = create_db_pool().await;
    println!("Connected to Sqlite");

    HttpServer::new(move || {
        println!("Server Start");
        App::new()
            // enable logger
            .wrap(Logger::default())
            // cookie session middleware
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    // customize session and cookie expiration
                    .session_lifecycle(
                        PersistentSession::default().session_ttl(cookie::time::Duration::hours(2)),
                    )
                    .build(),
            )
            .wrap(
                Cors::default()
                    .allowed_origin("https://www.rust-lang.org")
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().ends_with(b".rust-lang.org")
                    })
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .supports_credentials()
                    .allowed_header(http::header::COOKIE)
                    .allowed_header(http::header::SET_COOKIE)
                    .allowed_header(http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS)
                    .allowed_header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN)
                    .max_age(3600),
            )
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(echo)
            .service(add_user)
            .service(get_all_users)
            .service(get_user)
            .service(login)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
