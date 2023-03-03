use actix_web::{App, HttpRequest, HttpServer};
use crate::config::Config;

pub(crate) async fn start(config: Config) {
    HttpServer::new(move || {
        App::new()
            .service(index)
    })
    .bind((config.network().host(), config.network().port()))
    .unwrap()
    .run()
    .await
    .unwrap();
}

#[actix_web::get("/")]
async fn index(request: HttpRequest) -> &'static str {
    r#"{ "message": "Hello, world!" }"#
}
