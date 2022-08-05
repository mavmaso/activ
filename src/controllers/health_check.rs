use actix_web::{get, HttpResponse, Responder};

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/")]
pub async fn greet() -> impl Responder {
    format!("Hello Mundo!")
}
