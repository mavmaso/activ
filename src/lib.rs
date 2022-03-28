use actix_web::dev::Server;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, post};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/")]
async fn greet() -> impl Responder {
    format!("Hello Mundo!")
}
#[derive(serde::Deserialize)]
struct FormData {
    _email: String,
    _name: String
}

#[post("/subscriptions")]
async fn subscribe(_form: web::Form<FormData>) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .service(greet)
            .service(subscribe)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
