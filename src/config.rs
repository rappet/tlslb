use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    pub frontends: Frontends,
    pub backends: HashMap<String, Arc<Backend>>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Frontends {
    pub https: Frontend,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Frontend {
    pub listen_address: SocketAddr,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Backend {
    /// List of addresses to connect to
    ///
    /// Either a socket address like `192.0.2.0:8443` or `[2001:db8::1]:8443`
    /// or an address like `backend.tld:8443` that will result in a DNS lookup
    ///
    /// The set of all addresses/DNS responses will be used.
    /// If one address appears multiple times during the lookup, it will only be used once.
    pub addresses: Vec<String>,
    /// Open the TLS connection itself in case of an error even if SNI routing is used
    /// Overwrites the setting from the frontend
    #[serde(default)]
    pub terminate_tls_on_error: Option<bool>,
    /// Count of connections that will be held idle in the pool as preparation for new connections
    ///
    /// The default value is 0, which means, that for each connecting client a new connection
    /// to the backend will be made.
    /// If the value is greater than 0, connections will be made in advance and used for future
    /// connections on the frontend, which can result in faster round trip times.
    #[serde(default)]
    pub preconnect_count: Option<usize>,
}
