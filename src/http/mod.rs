mod response;
mod v1;

use actix_web::{App, HttpRequest, HttpServer};
use actix_web::web;
use actix_web::web::resource;
use crate::config::Config;
use crate::http::response::{ApiStatus, RackerResponse, RackerStatus};

pub(crate) async fn start(config: Config) {
    let server = HttpServer::new(|| {
        let api_v1 = web::scope("/v1")
            .service(resource("/").route(web::get().to(v1::index)));

        App::new()
            .service(api_v1)
            .route("/", web::get().to(index))
    });

    let host = config.network().host();
    let port = config.network().port();

    let server = match server.bind((host, port)) {
        Ok(server) => server,
        Err(err) => {
            log::error!("Failed to bind to {}:{}", host, port);
            crate::log_error_and_panic(err);
        }
    };

    match server.run().await {
        Ok(_) => (),
        Err(err) => {
            log::error!("Failed to start HTTP server");
            crate::log_error_and_panic(err);
        }
    };
}

async fn index(_request: HttpRequest) -> web::Json<RackerResponse<RackerStatus>> {
    web::Json(RackerResponse {
        status: "OK".to_string(),
        data: RackerStatus {
            api: ApiStatus {
                latest: "v1".to_string(),
                supported: vec!["v1".to_string()],
            },
        },
    })
}
