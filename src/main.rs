#[macro_use]
extern crate diesel;

mod account_api;
mod account_database;
mod account_dto;
mod account_entity;
mod account_interaction;
mod error_data;
mod notes_api;
mod notes_data;
mod postgres_database_connection;
mod schema;
mod security;
mod security_data;
mod utils;

use actix_web::{App, HttpServer};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};

const LOCALHOST_WITH_PORT: &str = "localhost:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // tbd create auth middleware

    // let ssl_acceptor_builder = get_ssl_acceptor_builder();

    let account_database_connection_pool = postgres_database_connection::get_database_connection_pool();

    HttpServer::new(move || {
        App::new()
            .data(account_database_connection_pool.clone())
            .service(account_api::account_v1_scope())
            .service(notes_api::notes_v1_scope())
    })
    .bind(LOCALHOST_WITH_PORT)?
    // .bind_openssl(LOCALHOST_WITH_PORT, ssl_acceptor_builder)?
    .run()
    .await
}

fn get_ssl_acceptor_builder() -> SslAcceptorBuilder {
    let mut ssl_acceptor_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    ssl_acceptor_builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    ssl_acceptor_builder
        .set_certificate_chain_file("cert.pem")
        .unwrap();

    ssl_acceptor_builder
}
