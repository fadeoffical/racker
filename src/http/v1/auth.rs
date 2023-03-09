use actix_web::{HttpResponse, web};
use crate::http::response;
use serde::{Deserialize, Serialize};
use crate::http::request::Request;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Auth {
    username: String,
    password: String,
}

#[actix_web::get("/auth")]
pub(crate) async fn get() -> HttpResponse {
    HttpResponse::Ok().json(response::ok())
}

#[actix_web::get("/auth/login")]
pub(crate) async fn get_login(
    body: web::Json<Request<Auth>>,
) -> HttpResponse {
    let body = body.into_inner();

    if body.auth.is_some() {
        return HttpResponse::Ok().json(response::error_with_message_and_data("already logged in".to_string(), body.auth));
    }

    let auth = match body.data {
        Some(auth) => auth,
        None => return HttpResponse::Ok().json(response::error_with_data("missing auth data")),
    };

    let username = auth.username;
    let password = auth.password;

    if username != "admin" {
        return HttpResponse::Ok().json(response::error_with_data("invalid username"));
    }

    if password != "admin" {
        return HttpResponse::Ok().json(response::error_with_data("invalid password"));
    }

    HttpResponse::Ok().json(response::ok_with_data("logged in"))
}
