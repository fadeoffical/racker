use std::sync::{MutexGuard, PoisonError};
use std::task::ready;
use crate::http::response::{Response, Status};
use crate::RackerState;

use serde::{Deserialize, Serialize};
use actix_web::{HttpResponse, web};
use crate::head::{Head, Heads};
use crate::http::response;

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
        Self {
            name: head.name,
        }
    }
}

#[actix_web::get("/")]
pub(crate) async fn get_index() -> web::Json<Response<(), ()>> {
    web::Json(response::ok())
}

#[actix_web::get("/heads")]
pub(crate) async fn get_heads(data: web::Data<RackerState>) -> web::Json<Response<V1Heads, ()>> {
    let mut api_heads = Vec::new();

    let heads = data.heads.lock().unwrap();

    heads.get_all().iter().for_each(|head| {
        let api_head = head.clone().into();
        api_heads.push(api_head);
    });


    web::Json(response::ok_with_data(V1Heads {
        heads: api_heads,
    }))
}

#[actix_web::post("/heads")]
pub(crate) async fn post_heads(
    data: web::Data<RackerState>,
    body: web::Json<V1Head>,
) -> HttpResponse {
    let mut heads = match data.heads.lock() {
        Ok(heads) => heads,
        Err(err) => return error_lock_heads(err),
    };


    let head: Head = body.into_inner().into();

    log::info!("Added head: {:?}", &head.name);

    match heads.register_head(head) {
        Ok(_) => HttpResponse::Ok().json(response::ok()),
        Err(_) => HttpResponse::Conflict().json(response::error_with_data("Head already exists")),
    }


}

#[actix_web::get("/heads/{name}")]
pub(crate) async fn get_head_by_name(
    data: web::Data<RackerState>,
    path: web::Path<(String,)>,
) -> HttpResponse {
    let head_name = path.into_inner().0;

    let heads = match data.heads.lock() {
        Ok(heads) => heads,
        Err(err) => return error_lock_heads(err),
    };

    let head = heads.get_all()
        .iter()
        .find(|head| head.name == head_name)
        .map_or_else(|| None, |head| {
            Some(V1Head {
                name: head.name.clone(),
            })
        }
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
