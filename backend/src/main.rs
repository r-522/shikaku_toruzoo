mod config;
mod db;
mod errors;
mod handlers;
mod middleware;
mod models;
mod services;
mod utils;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{http::header, web, App, HttpRequest, HttpResponse, HttpServer, middleware::Logger};
use config::Config;
use db::SupabaseClient;

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

async fn spa_fallback(config: web::Data<Config>, _req: HttpRequest) -> actix_web::Result<actix_files::NamedFile> {
    let index_path = format!("{}/index.html", config.static_dir);
    Ok(actix_files::NamedFile::open(index_path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let config = Config::from_env();
    let port = config.port();
    let static_dir = config.static_dir.clone();
    let cors_origin = config.cors_origin.clone();

    let supabase = SupabaseClient::new(&config.supabase_url, &config.supabase_key);

    log::info!("Starting CertManager on port {}", port);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cors_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::HeaderName::from_static("x-requested-with"),
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(supabase.clone()))
            // API routes
            .route("/api/health", web::get().to(health))
            .service(
                web::scope("/api/auth")
                    .route("/signup", web::post().to(handlers::auth::signup))
                    .route("/signin", web::post().to(handlers::auth::signin))
                    .route("/signout", web::post().to(handlers::auth::signout))
                    .route("/me", web::get().to(handlers::auth::me)),
            )
            .service(
                web::scope("/api/certifications")
                    .route("", web::get().to(handlers::certification::list))
                    .route("", web::post().to(handlers::certification::create))
                    .route("/{id}", web::put().to(handlers::certification::update))
                    .route("/{id}", web::delete().to(handlers::certification::delete)),
            )
            .service(
                web::scope("/api/goals")
                    .route("", web::get().to(handlers::goal::list))
                    .route("", web::post().to(handlers::goal::create))
                    .route("/{id}", web::put().to(handlers::goal::update))
                    .route("/{id}", web::delete().to(handlers::goal::delete)),
            )
            .service(
                web::scope("/api/master")
                    .route("/certifications", web::get().to(handlers::master::search)),
            )
            .service(
                web::scope("/api/community")
                    .route("/users", web::get().to(handlers::community::list_users))
                    .route("/users/{id}", web::get().to(handlers::community::get_user)),
            )
            .service(
                web::scope("/api/favorites")
                    .route("", web::get().to(handlers::favorite::list))
                    .route("/{userId}", web::post().to(handlers::favorite::add))
                    .route("/{userId}", web::delete().to(handlers::favorite::remove)),
            )
            // Static files and SPA fallback
            .service(Files::new("/", &static_dir).index_file("index.html"))
            .default_service(web::get().to(spa_fallback))
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
