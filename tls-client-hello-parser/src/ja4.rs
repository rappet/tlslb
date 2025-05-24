use core::str::FromStr;
use faster_hex::hex_encode;
use heapless::{String, Vec};
use sha2::{Digest, Sha256};

pub(crate) fn hash12(data: &[u8]) -> String<12> {
    let hashed = Sha256::digest(data);
    let mut hex = [0u8; 12];
    let s = hex_encode(&hashed[..6], &mut hex[..]).expect("Should not fail");
    String::from_str(s).expect("fits")
}

pub(crate) const fn format_version(v: u16) -> &'static str {
    match v {
        0x0304 => "13", // TLS 1.3
        0x0303 => "12", // TLS 1.2
        0x0302 => "11", // TLS 1.1
        0x0301 => "10", // TLS 1.0
        0x0300 => "s3", // SSL 3.0
        0x0002 => "s2", // SSL 2.0
        0xfeff => "d1", // DTLS 1.0
        0xfefd => "d2", // DTLS 1.2
        0xfefc => "d3", // DTLS 1.3
        _ => "00",      // unknown
    }
}

pub(crate) fn format_alpn(alpn: &[u8]) -> String<2> {
    if alpn.len() > 2 {
        let alpn_bytes = [alpn[0], alpn[alpn.len() - 1]];
        if alpn_bytes.iter().all(u8::is_ascii) {
            core::str::from_utf8(&alpn_bytes).unwrap().try_into().unwrap()
        } else {
            "99".try_into().unwrap()
        }
    } else if alpn.iter().all(u8::is_ascii) {
        core::str::from_utf8(alpn).unwrap().try_into().unwrap()
    } else {
        "99".try_into().unwrap()
    }
}

/// Converts values to a hex byte string
pub(crate) fn u16_slice_to_hex(data: &[u16]) -> Result<Vec<u8, 256>, ()> {
    let mut out = Vec::new();
    for v in data {
        let bytes = u16::to_be_bytes(*v);
        let mut hex = [0u8; 4];
        hex_encode(&bytes, &mut hex).expect("slice has correct size");
        out.extend_from_slice(&hex).map_err(|_err| ())?;
        out.push(b',').map_err(|_err| ())?;
    }
    let _ = out.pop();
    Ok(out)
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_hash12() {
        assert_eq!("1d37bd780c83"[..], hash12(b"002f,0033,0035,0039,003c,003d,0067,006b,009c,009d,009e,009f,1301,1302,1303,c009,c00a,c013,c014,c023,c024,c027,c028,c02b,c02c,c02f,c030,cca8,cca9,ccaa"));
    }

    #[test]
    fn test_format_alpn() {
        assert_eq!("h2", format_alpn(b"h2").as_str());
        assert_eq!("h1", format_alpn(b"http/1.1").as_str());
        assert_eq!("b", format_alpn(b"b").as_str());
        assert_eq!("t1", format_alpn(b"tls-alpn-01").as_str());
    }

    #[test]
    fn test_u16_slice_to_hex() {
        assert_eq!(b"", u16_slice_to_hex(&[]).unwrap());
        assert_eq!(b"1234", u16_slice_to_hex(&[0x1234]).unwrap());
        assert_eq!(b"0001,0002", u16_slice_to_hex(&[0x1, 0x2]).unwrap());
        assert_eq!(b"1234,5678", u16_slice_to_hex(&[0x1234, 0x5678]).unwrap());
    }
}
