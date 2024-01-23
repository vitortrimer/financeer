use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};

pub mod application;
pub mod domain;

#[get("/api/healthcheck")]
async fn health_check_handler() -> impl Responder {
    const MESSAGE: &str = "Financeer web services is running";

    HttpResponse::Ok().json(serde_json::json!({"status": "success", "message": MESSAGE}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    env_logger::init();
    println!("🚀 Server started on port 8000");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(health_check_handler)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
