use crate::http::response;
use actix_web::HttpResponse;

pub(crate) mod api;

#[actix_web::get("/")]
pub(crate) async fn get() -> HttpResponse {
    HttpResponse::Ok().json(response::ok_with_message(
        "Hello from racker! :)".to_string(),
    ))
}
