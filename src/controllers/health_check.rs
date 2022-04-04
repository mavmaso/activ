use actix_web::{get, Responder, HttpResponse};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/")]
pub async fn greet() -> impl Responder {
    format!("Hello Mundo!")
}