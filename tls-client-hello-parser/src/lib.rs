#![no_std]
extern crate alloc;

mod client_hello;
mod ja4;

pub use client_hello::ClientHello;
pub use ja4::Ja4Fingerprint;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq, Eq)]
pub enum TlsParseError {
    #[error("TLS client hello is incomplete")]
    Incomplete,
    #[error("Error parsing the outer packet")]
    ParseOuterPacket,
    #[error("TODO")]
    Todo,
}
