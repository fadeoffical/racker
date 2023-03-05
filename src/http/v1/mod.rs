pub(crate) mod heads;

use actix_web::HttpResponse;
use crate::http::response;

#[actix_web::get("/")]
pub(crate) async fn get() -> HttpResponse {
    HttpResponse::Ok().json(response::ok())
}
