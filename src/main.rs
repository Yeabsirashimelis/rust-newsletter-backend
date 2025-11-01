use std::net::TcpListener;

use newsletter_backend::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind random port");

    run(listener)?.await
}
