extern crate actix_web;

use actix_web::{get, post, put, delete, web, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::body::BoxBody;
use serde::{Serialize, Deserialize};
use std::fmt::Display;
use std::sync::Mutex;
pub mod Models;
pub mod Routers;
pub mod Services;
use tokio_postgres::{Error, NoTls, Row};

#[tokio::main]
async fn main() -> Result<(), Error> {
    
    
    HttpServer::new(move || {
        App::new()
            .service(Services::Api::users::register_user)
            .service(Services::Api::users::register_user1)
    })
        .bind(("127.0.0.1", 8000)).unwrap()
        .run()
        .await.unwrap();
    
    Ok(())
}