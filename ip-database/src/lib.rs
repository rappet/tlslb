// mod asn;
mod ip;

// pub use asn::{AsDatabase, AsRef};
pub use ip::{IpDatabase, IpRef};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoadIpDatabaseError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("input file is malformed: {0}")]
    MalformedInputFile(String),
    #[error("forbidden to fetch source data")]
    Forbidden,
}
