#![allow(dead_code)]

// <ssl>
use ntex::web;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[web::get("/")]
async fn index(_req: web::HttpRequest) -> impl web::Responder {
    "Welcome!"
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    // load TLS keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder =
        SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    web::HttpServer::new(|| web::App::new().service(index))
        .bind_openssl("127.0.0.1:8080", builder)?
        .run()
        .await
}
// </ssl>
