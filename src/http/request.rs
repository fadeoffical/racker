use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Request<T> {
    pub(crate) auth: Option<Auth>,
    pub(crate) data: Option<T>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) struct Auth {
    pub(crate) username: String,
    pub(crate) password: String,
}
