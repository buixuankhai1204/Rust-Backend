// Create a ticket
use actix_web::{get, post, put, delete, web, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};
use serde::{Serialize, Deserialize};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::body::BoxBody;
use crate::Models::RequestUser;
use validator::{Validate, ValidationError};



#[post("/register")]
pub async fn register_user(req: web::Json<RequestUser>) -> impl Responder {
    let json = req.validate();
    println!("{:?}", json);
    if json != Ok(()){
         return HttpResponse::BadRequest().json(json)
    }
    /*let new_register = RequestUser {
        id: req.id,
        username: req.username,
        fullname: "".to_string(),
        email: None,
        age: 0,
        address: None,
    };*/
    println!("{:?}", req);
    let response = serde_json::to_string(&req).unwrap();

    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(response)
}