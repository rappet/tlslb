#![no_main]

use libfuzzer_sys::fuzz_target;
use tls_client_hello_parser::Ja4Fingerprint;

fuzz_target!(|data: &[u8]| {
    // do not panic!
    if let Ok(client_hello) = tls_client_hello_parser::parse_tls_client_hello(data) {
        let _ja4 = Ja4Fingerprint::calculate(&client_hello);
    }
});
