pub(crate) mod request;
pub(crate) mod response;
mod routes;

use crate::http::response::{ApiInfo, Apis};
use crate::RackerState;
use actix_web::{web, App, HttpResponse, HttpServer};

pub(crate) async fn start(state: RackerState) {
    let config = state.config.clone();

    let data = web::Data::new(state);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(routes::get)
            .service(
                web::scope("/api").service(routes::api::get).service(
                    web::scope("/v1")
                        .service(routes::api::v1::get)
                        .service(
                            web::scope("/heads")
                                .service(routes::api::v1::heads::get)
                                .service(routes::api::v1::heads::post)
                                .service(routes::api::v1::heads::delete)
                                .service(routes::api::v1::heads::get_name),
                        )
                        .service(
                            web::scope("/auth")
                                .service(routes::api::v1::auth::get)
                                .service(routes::api::v1::auth::get_login),
                        ),
                ),
            )
    });

    let host = config.network().as_socket_addr();
    let server = match server.bind(host) {
        Ok(server) => {
            log::info!("Listening on {:?}", host);
            server
        }
        Err(err) => {
            log::error!("Failed to bind to {:?}", host);
            crate::log_error_and_panic(Box::new(err));
        }
    };

    match server.run().await {
        Ok(_) => (),
        Err(err) => {
            log::error!("Failed to start HTTP server");
            crate::log_error_and_panic(Box::new(err));
        }
    };
}
