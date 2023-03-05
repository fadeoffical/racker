use serde::{Serialize};

#[derive(Debug, Serialize)]
pub(crate) struct RackerResponse<R>
where R: Serialize
{
    pub(crate) status: String,
    pub(crate) data: R,
}

#[derive(Debug, Serialize)]
pub(crate) struct RackerStatus {
    pub(crate) api: ApiStatus,
}

#[derive(Debug, Serialize)]
pub(crate) struct ApiStatus {
    pub(crate) latest: String,
    pub(crate) supported: Vec<String>,
}
