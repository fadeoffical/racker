use actix_web::{HttpRequest, web};
use crate::http::response::RackerResponse;

pub(crate) async fn index(_request: HttpRequest) -> web::Json<RackerResponse<()>> {
    web::Json(RackerResponse {
        status: "".to_string(),
        data: (),
    })
}
