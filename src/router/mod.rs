use actix_web::{get, post, App, web, HttpResponse, HttpServer, Responder};

use crate::structure;

#[get("/")]
pub async fn home() -> impl Responder {
    HttpResponse::Ok().body("hello world")
}

pub async fn login(user_login_info: web::Json<structure::UserLoginInfo>) -> impl Responder {
    HttpResponse::Ok().body(format!("Welcome {}: {}", user_login_info.username, user_login_info.password))
}