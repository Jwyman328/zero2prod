use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    name: String,
    email: String,
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

async fn sign_up_for_newsletter(info: web::Form<Info>) -> impl Responder {
    println!("we have good data name {}", info.name);
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route(
                "/sign_up_for_newsletter",
                web::post().to(sign_up_for_newsletter),
            )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
