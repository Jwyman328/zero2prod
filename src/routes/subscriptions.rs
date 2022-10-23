use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SubscriberInfo {
    name: String,
    email: String,
}

pub async fn subscribe(info: web::Form<SubscriberInfo>) -> impl Responder {
    println!("we have good data name {} {}", info.name, info.email);
    HttpResponse::Ok()
}
