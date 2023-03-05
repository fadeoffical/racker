pub(crate) mod response;
pub(crate) mod v1;

use actix_web::{web, App, HttpServer};
use crate::http::response::{Apis, Response, ApiInfo};
use crate::RackerState;

pub(crate) async fn start(state: RackerState) {
    let config = state.config.clone();

    let data = web::Data::new(state);
    let server = HttpServer::new(move || {

        App::new()
            .app_data(data.clone())
            .service(
                web::scope("/v1")
                    .service(v1::get_index)
                    .service(v1::get_heads)
                    .service(v1::post_heads)
                    .service(v1::get_head_by_name)
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


async fn index() -> web::Json<Response<ApiInfo, ()>> {
    web::Json(Response::Ok {
        data: Some(ApiInfo {
            api: Apis {
                latest: "v1".to_string(),
                supported: vec!["v1".to_string()],
            },
        }),
    })
}
