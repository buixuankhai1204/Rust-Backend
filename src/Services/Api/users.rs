// Create a ticket
use actix_web::{get, post, put, delete, web, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError};
use serde::{Serialize, Deserialize};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::body::BoxBody;
use actix_web::cookie::time::Duration;
use futures::TryFutureExt;
use crate::Models::RequestUser;
use crate::Services::Sql::postgres;
use validator::{Validate, ValidationError};



#[post("/register")]
pub async fn register_user(mut req: web::Json<RequestUser>) -> HttpResponse {
    let mut json = req.validate();
    if json != Ok(())
    {
         return HttpResponse::BadRequest().json(json)
    }
    
    /*let data: RequestUser = req.into_inner();
    println!("{:?}", &data);*/
    tokio::time::sleep(core::time::Duration::new(5,0)).await; // <-- Ok. Worker thread will handle other requests here
    let data = postgres::get_user().await;
    println!("{:?}",&data);
    if(data.is_err())
    {
        println!("error")
    }
    
    let respone = serde_json::to_string(&data.unwrap()).unwrap();
    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(respone)
}

#[post("/register1")]
pub async fn register_user1(mut req: web::Json<RequestUser>) -> HttpResponse {
    let mut json = req.validate();
    if json != Ok(())
    {
        return HttpResponse::BadRequest().json(json)
    }

    /*let data: RequestUser = req.into_inner();
    println!("{:?}", &data);*/
/*    tokio::time::sleep(core::time::Duration::new/**/(5,0)).await; // <-- Ok. Worker thread will handle other requests here
*/    let data = postgres::get_user().await;
    println!("{:?}",&data);
    if(data.is_err())
    {
        println!("error")
    }

    let respone = serde_json::to_string(&data.unwrap()).unwrap();
    HttpResponse::Created()
        .content_type(ContentType::json())
        .body(respone)
}