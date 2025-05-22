#![no_std]

mod ja4;
pub mod parser;

use heapless::String;
pub use parser::parse_tls_client_hello;
use tinyvec::ArrayVec;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ClientHello<'a> {
    pub tls_version: u16,
    pub sni: Option<&'a str>,
    pub cipher_suites: ArrayVec<[u16; 30]>,
    pub extensions: ArrayVec<[u16; 14]>,
    pub signature_algorithms: ArrayVec<[u16; 30]>,
    pub alpn: ArrayVec<[&'a [u8]; 4]>,
    pub ja4: String<37>,
}
