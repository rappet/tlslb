use faster_hex::hex_encode;
use sha2::{Digest, Sha256};

pub(crate) fn hash12(data: &[u8]) -> [u8; 12] {
    let hashed = Sha256::digest(data);
    let mut hex = [0u8; 12];
    hex_encode(&hashed[..6], &mut hex[..]).expect("Should not fail");
    hex
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_hash12() {
        assert_eq!(b"1d37bd780c83"[..], hash12(b"002f,0033,0035,0039,003c,003d,0067,006b,009c,009d,009e,009f,1301,1302,1303,c009,c00a,c013,c014,c023,c024,c027,c028,c02b,c02c,c02f,c030,cca8,cca9,ccaa"));
    }
}
