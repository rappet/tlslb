mod parser;

pub use parser::parse_tls_client_hello;
use tinyvec::TinyVec;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ClientHello<'a> {
    pub tls_version: u16,
    pub sni: Option<&'a str>,
    pub cipher_suites: TinyVec<[u16; 30]>,
    pub extensions: TinyVec<[u16; 14]>,
    pub signature_algorithms: TinyVec<[u16; 14]>,
    pub alpn: TinyVec<[&'a [u8]; 4]>,
}
