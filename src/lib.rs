#![allow(unused, dead_code, unused_imports, unused_doc_comments)]

use std::net::TcpListener;

pub use actix_web::{
    dev::Server,
    web,
    App,
    HttpRequest,
    HttpResponse,
    HttpServer,
    Responder,
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}
