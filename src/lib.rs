pub mod configuration;
pub mod routes;
pub mod startup;


use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    email: String,
    name: String,
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
    //     .bind(("127.0.0.1", 8080))?
    //     .run();

    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server) //returns server. which is asynchronous
}
