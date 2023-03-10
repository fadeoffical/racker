use crate::http::request::Request;
use crate::http::response;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Auth {
    username: String,
    password: String,
}

#[actix_web::get("")]
pub(crate) async fn get() -> HttpResponse {
    HttpResponse::Ok().json(response::ok())
}

#[actix_web::get("/login")]
pub(crate) async fn get_login(
    body: web::Json<Request<Auth>>,
    data: web::Data<crate::RackerState>,
) -> HttpResponse {
    let body = body.into_inner();

    if body.auth.is_some() {
        return HttpResponse::Ok().json(response::error_with_message_and_data(
            "already logged in".to_string(),
            body.auth,
        ));
    }

    let auth = match body.data {
        Some(auth) => auth,
        None => return HttpResponse::Ok().json(response::error_with_data("missing auth data")),
    };

    let username = auth.username;
    let password = auth.password;

    let users = data.users.lock().unwrap();
    return users
        .iter()
        .find(|user| user.username == username && user.password == password)
        .map(|user| HttpResponse::Ok().json(response::ok_with_data(user)))
        .unwrap_or_else(|| {
            HttpResponse::Unauthorized().json(response::error_with_message(
                "invalid username or password".to_string(),
            ))
        });
}
