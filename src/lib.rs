use actix_web::{web, get, post, App, HttpResponse, HttpServer};
use actix_web::dev::Server;
use serde::Deserialize;
use std::net::TcpListener;

#[get("/health_check")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String
}
#[post("/subscriptions")]
async fn subscriptions(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| { 
        App::new()
            .service(health_check)
            .service(subscriptions)
    })
    .listen(listener)?
    .run();
    Ok(server)
}

