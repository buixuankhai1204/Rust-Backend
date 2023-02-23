use actix_web::{HttpRequest, HttpResponse, web, App, HttpServer, Result,body, Responder};
use serde::{Deserialize, Serialize};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web_validator::Form;
use actix_web_validator::Query;
use validator::Validate;

#[derive(Deserialize,Serialize,Debug, Validate)]
pub struct RequestUser{
    pub id: u32,
    #[validate(length(min = 3, message = "Name must be greater than 3 chars"))]
    pub username: String,
    pub fullname: String,
    pub email: Option<String>,
    pub age: u32,
    pub address: Option<String>
}

impl Responder for RequestUser {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let res_body = serde_json::to_string(&self).unwrap();

        // Create HttpResponse and set Content Type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(res_body)
    }
}