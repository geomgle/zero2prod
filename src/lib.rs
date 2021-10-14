#![allow(unused, dead_code, unused_imports, unused_doc_comments)]

pub use actix_web::{
    dev::Server, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
pub use anyhow::{Context, Result};
use std::net::TcpListener;

pub mod configuration;
pub mod routes;
pub mod startup;

#[derive(serde::Deserialize)]
pub struct FormData {
    name: String,
    email: String,
}
