pub(crate) mod heads;

use crate::http::response;
use actix_web::HttpResponse;

#[actix_web::get("/")]
pub(crate) async fn get() -> HttpResponse {
    HttpResponse::Ok().json(response::ok())
}
