use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub frontends: HashMap<String, Frontend>,
    pub backends: HashMap<String, Arc<Backend>>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Frontend {
    pub listen_address: SocketAddr,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Backend {
    pub addresses: Vec<SocketAddr>,
    /// Open the TLS connection itself in case of an error even if SNI routing is used
    /// Overwrites the setting from the frontend
    #[serde(default)]
    pub terminate_tls_on_error: Option<bool>,
    /// Count of connections that will be added to the pool in advance
    #[serde(default)]
    pub preconnect_count: Option<usize>,
}
