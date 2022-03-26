use actix_web::{get, web, App, HttpServer, Responder, HttpResponse, HttpRequest};

#[get("/")]
async fn greet() -> impl Responder {
    format!("Hello World!")
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

pub async fn run() -> std::io::Result<()> {
    println!("Running ...");

    HttpServer::new(|| {
        App::new()
        .service(greet)
        .route("/health_check", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
