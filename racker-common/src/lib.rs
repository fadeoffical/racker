use crate::data::Network;

pub mod data;

pub struct Cluster {
    heads: Vec<Head>,
    workers: Vec<Worker>,
}

pub struct Head {
    name: String,
    meta: HeadMeta,
}

pub struct Worker {
    name: String,
}

pub struct HeadMeta {
    network: Network,
}
