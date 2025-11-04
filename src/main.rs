use std::net::TcpListener;

use newsletter_backend::{configuration::get_configuration, run};

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    // panic if we can't read configuration
    let configuration = get_configuration().expect("Failed to read configuration");

    // we have removed the hard_coded "8080" - it is coming from our settings!
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    run(listener)?.await
}
