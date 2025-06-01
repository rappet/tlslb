// mod asn;
mod ip;

use std::io::{Read, Write};

// pub use asn::{AsDatabase, AsRef};
pub use ip::{IpDatabase, IpRef};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadIpDatabaseError {
    #[error(transparent)]
    Io(std::io::Error),
    #[error("database is malformed")]
    MalformedDatabase(String),
    #[error("forbidden to fetch source data")]
    Forbidden,
}
