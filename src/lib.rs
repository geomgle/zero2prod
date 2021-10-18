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
pub use anyhow::{Context, Result};

pub mod configuration;
pub mod routes;
pub mod startup;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}
