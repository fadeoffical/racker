use actix_web::{web, HttpResponse};
use std::sync::{MutexGuard, PoisonError};

use serde::{Deserialize, Serialize};

use crate::head::{Head, Heads};
use crate::http::response;
use crate::RackerState;

#[derive(Debug, Serialize)]
pub(crate) struct V1Heads {
    pub(crate) heads: Vec<V1Head>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct V1Head {
    pub(crate) name: String,
}

impl From<Head> for V1Head {
    fn from(head: Head) -> Self {
        Self { name: head.name }
    }
}

#[actix_web::get("/heads")]
pub(crate) async fn get(data: web::Data<RackerState>) -> HttpResponse {
    let mut api_heads = Vec::new();

    let heads = data.heads.lock().unwrap();

    heads.get_all().iter().for_each(|head| {
        let api_head = head.clone().into();
        api_heads.push(api_head);
    });

    HttpResponse::Ok().json(response::ok_with_data(V1Heads { heads: api_heads }))
}

#[actix_web::post("/heads")]
pub(crate) async fn post(data: web::Data<RackerState>, body: web::Json<V1Head>) -> HttpResponse {
    let mut heads = match data.heads.lock() {
        Ok(heads) => heads,
        Err(err) => return error_lock_heads(err),
    };

    let head_name = body.name.clone();
    let head: Head = body.into_inner().into();

    match heads.register_head(head) {
        Ok(_) => {
            log::info!("Registered head {}", head_name);
            HttpResponse::Ok().json(response::ok())
        }
        Err(_) => {
            log::warn!("Tried to register head {} which already exists", head_name);
            HttpResponse::Conflict().json(response::error_with_data("Head already exists"))
        }
    }
}

#[actix_web::delete("/heads/{name}")]
pub(crate) async fn delete(
    data: web::Data<RackerState>,
    path: web::Path<(String,)>,
) -> HttpResponse {
    let head_name = path.into_inner().0;

    let mut heads = match data.heads.lock() {
        Ok(heads) => heads,
        Err(err) => return error_lock_heads(err),
    };

    match heads.unregister_head(head_name.clone()) {
        Ok(_) => {
            log::info!("Unregistered head {}", head_name);
            HttpResponse::Ok().json(response::ok())
        }
        Err(_) => {
            log::warn!(
                "Tried to unregister head {} which does not exist",
                head_name
            );
            HttpResponse::NotFound().json(response::error_with_data("Head does not exist"))
        }
    }
}

#[actix_web::get("/heads/{name}")]
pub(crate) async fn get_name(
    data: web::Data<RackerState>,
    path: web::Path<(String,)>,
) -> HttpResponse {
    let head_name = path.into_inner().0;

    let heads = match data.heads.lock() {
        Ok(heads) => heads,
        Err(err) => return error_lock_heads(err),
    };

    let head = heads
        .get_all()
        .iter()
        .find(|head| head.name == head_name)
        .map_or_else(
            || None,
            |head| {
                Some(V1Head {
                    name: head.name.clone(),
                })
            },
        );

    match head {
        None => HttpResponse::NotFound().json(response::error()),
        Some(head) => HttpResponse::Ok().json(response::ok_with_data(head)),
    }
}

fn error_lock_heads(err: PoisonError<MutexGuard<'_, Heads>>) -> HttpResponse {
    log::error!("Failed to lock heads: {}", err);
    HttpResponse::InternalServerError().json(response::error_with_data("Internal server error"))
}
