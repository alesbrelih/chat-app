use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Room {
    pub id: Uuid,
    pub name: String,
}

#[get("/api/health")]
async fn health_checker_handler() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "OK"}))
}

#[get("/api/rooms")]
async fn rooms(req_body: String) -> impl Responder {
    HttpResponse::Ok().json(Room {
        id: Uuid::new_v4(),
        name: String::from("first room"),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    // chat
    println!("🚀 Server started successfully!");

    HttpServer::new(move || {
        App::new()
            .service(health_checker_handler)
            .service(rooms)
            .wrap(Logger::default())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
