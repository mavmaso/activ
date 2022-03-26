use actix_web::{get, web, App, HttpServer, Responder, HttpResponse, HttpRequest};
use actix_web::dev::Server;

#[get("/")]
async fn greet() -> impl Responder {
    format!("Hello World!")
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub async fn run() -> Result<Server, std::io::Error> {
    println!("Running ...");

    let server = HttpServer::new(|| {
        App::new()
        .service(greet)
        .route("/health_check", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    Ok(server)
}
