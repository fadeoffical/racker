pub(crate) mod response;
pub(crate) mod v1;

use actix_web::{web, App, HttpServer, HttpResponse};
use crate::http::response::{Apis, ApiInfo};
use crate::RackerState;

pub(crate) async fn start(state: RackerState) {
    let config = state.config.clone();

    let data = web::Data::new(state);
    let server = HttpServer::new(move || {
        App::new().app_data(data.clone())
            .service(
                web::scope("/v1")
                    .service(v1::get)
                    .service(v1::heads::get)
                    .service(v1::heads::post)
                    .service(v1::heads::delete)
                    .service(v1::heads::get_name)
            )
            .route("/", web::get().to(index))
    });

    let host = config.network().as_socket_addr();

    let server = match server.bind(host) {
        Ok(server) => server,
        Err(err) => {
            log::error!("Failed to bind to {:?}", host);
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


async fn index() -> HttpResponse {
    HttpResponse::Ok().json(response::ok_with_data(ApiInfo {
        api: Apis {
            latest: "v1".to_string(),
            supported: vec!["v1".to_string()],
        },
    }))
}
