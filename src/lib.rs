use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use actix_web::dev::Server;

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}


pub fn run(address: &str) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| { 
            App::new()
                .service(health_check)
        })
        .bind(address)?
        .run();
    Ok(server)
}

