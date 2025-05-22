#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    // do not panic!
    let _ = tls_client_hello_parser::parse_tls_client_hello(data);
});
