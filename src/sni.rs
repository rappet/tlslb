use tls_parser::{
    IResult, SNIType, TlsExtension, TlsMessage, TlsMessageHandshake, TlsPlaintext,
    parse_tls_extensions, parse_tls_plaintext,
};
use tracing::warn;

pub(crate) fn extract_sni_from_header(buffer: &[u8]) -> IResult<&[u8], Option<String>> {
    let (rest, header) = parse_tls_plaintext(buffer)?;
    Ok((rest, extract_sni(header).map(|sni| sni.to_owned())))
}

fn extract_sni(header: TlsPlaintext) -> Option<&str> {
    for msg in &header.msg {
        if let TlsMessage::Handshake(TlsMessageHandshake::ClientHello(hello)) = &msg {
            if let Some((_rest, extensions)) =
                hello.ext.and_then(|ext| parse_tls_extensions(ext).ok())
            {
                for extension in extensions {
                    if let Some(extension) = extract_sni_from_extensions(extension) {
                        return Some(extension);
                    }
                }
            }
        }
    }

    None
}

fn extract_sni_from_extensions(extension: TlsExtension) -> Option<&str> {
    match extension {
        TlsExtension::SNI(sni) => {
            for (sni_type, sni_content) in sni {
                if sni_type == SNIType::HostName {
                    if let Ok(sni_content_str) = std::str::from_utf8(sni_content) {
                        return Some(sni_content_str);
                    }
                }
            }
            None
        }
        TlsExtension::EncryptedServerName { .. } => {
            warn!("got ESNI :(");
            None
        }
        _ => None,
    }
}
