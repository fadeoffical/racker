use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "status")]
pub(crate) enum Response<T, R> {
    Ok {
        message: Option<String>,
        data: Option<T>
    },
    Error {
        message: Option<String>,
        error: Option<R>,
    },
}

pub(crate) fn ok() -> Response<(), ()> {
    Response::Ok {
        message: None,
        data: None
    }
}

pub(crate) fn error() -> Response<(), ()> {
    Response::Error {
        message: None,
        error: None,
    }
}

pub(crate) fn ok_with_data<T>(data: T) -> Response<T, ()> {
    Response::Ok {
        message: None,
        data: Some(data)
    }
}

pub(crate) fn ok_with_message(message: String) -> Response<(), ()> {
    Response::Ok {
        message: Some(message),
        data: None
    }
}

pub(crate) fn ok_with_message_and_data<T>(message: String, data: T) -> Response<T, ()> {
    Response::Ok {
        message: Some(message),
        data: Some(data)
    }
}

pub(crate) fn error_with_data<E>(error: E) -> Response<(), E> {
    Response::Error {
        message: None,
        error: Some(error),
    }
}

pub(crate) fn error_with_message(message: String) -> Response<(), ()> {
    Response::Error {
        message: Some(message),
        error: None,
    }
}

pub(crate) fn error_with_message_and_data<E>(message: String, error: E) -> Response<(), E> {
    Response::Error {
        message: Some(message),
        error: Some(error),
    }
}

#[derive(Debug, Serialize)]
pub(crate) struct ApiInfo {
    pub(crate) api: Apis,
}

#[derive(Debug, Serialize)]
pub(crate) struct Apis {
    pub(crate) latest: String,
    pub(crate) supported: Vec<String>,
}
