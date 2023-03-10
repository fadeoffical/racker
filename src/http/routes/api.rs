use crate::http::response;
use crate::http::response::{Api, ApiInfo, Apis};
use actix_web::HttpResponse;

pub(crate) mod v1;

#[actix_web::get("")]
pub(crate) async fn get() -> HttpResponse {
    HttpResponse::Ok().json(response::ok_with_data(ApiInfo {
        api: Apis {
            latest: Api {
                version: "v1".to_string(),
                route: "/api/v1".to_string(),
            },
            supported: vec![Api {
                version: "v1".to_string(),
                route: "/api/v1".to_string(),
            }],
        },
    }))
}
