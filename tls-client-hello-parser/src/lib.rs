#![no_std]

mod ja4;
pub mod parser;

use heapless::{String, Vec};
pub use parser::parse_tls_client_hello;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ClientHello<'a> {
    pub tls_version: u16,
    pub sni: Option<&'a str>,
    pub cipher_suites: Vec<u16, 128>,
    pub extensions: Vec<u16, 128>,
    pub signature_algorithms: Vec<u16, 30>,
    pub alpn: Vec<&'a [u8], 4>,
    pub ja4: String<37>,
}
