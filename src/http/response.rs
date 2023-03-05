use serde::{Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "status")]
pub(crate) enum Response<T, R> {
    Ok {
        data: Option<T>,
    },
    Error {
        error: Option<R>,
    },
}


pub(crate) fn ok() -> Response<(), ()> {
    Response::Ok {
        data: None,
    }
}

pub(crate) fn error() -> Response<(), ()> {
    Response::Error {
        error: None,
    }
}

pub(crate) fn ok_with_data<T>(data: T) -> Response<T, ()> {
    Response::Ok {
        data: Some(data),
    }
}

pub(crate) fn error_with_data<E>(error: E) -> Response<(), E> {
    Response::Error {
        error: Some(error),
    }
}


// #[derive(Debug, Serialize)]
// pub(crate) enum Status {
//     Ok,
//     Error,
// }


#[derive(Debug, Serialize)]
pub(crate) struct ApiInfo {
    pub(crate) api: Apis,
}

#[derive(Debug, Serialize)]
pub(crate) struct Apis {
    pub(crate) latest: String,
    pub(crate) supported: Vec<String>,
}
