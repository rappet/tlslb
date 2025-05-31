mod config;
mod state;

use std::{
    fs,
    net::{SocketAddr, SocketAddrV6},
    sync::Arc,
    time::Instant,
};

use anyhow::{Context, Result, bail};
use clap::Parser;
use mimalloc::MiMalloc;
use tls_client_hello_parser::{ClientHello, Ja4Fingerprint};
use tlslb::cli::Cli;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, copy},
    net::{TcpListener, TcpStream, lookup_host},
    spawn, try_join,
};
use tracing::{Level, info, instrument};

use crate::{config::Config, state::State};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let opts: Cli = Cli::parse();
    let opts = Arc::new(opts);

    let config: Config = toml::from_str(&fs::read_to_string(&opts.config_file)?)?;

    info!("{config:?}");

    let config = Arc::new(config);
    let state = Arc::new(State::new(Arc::clone(&config)).await?);

    let sock_addr: SocketAddr = config.frontends.https.listen_address;
    let listener = TcpListener::bind(sock_addr)
        .await
        .context("failed to bind https socket")?;

    while let Ok((stream, _addr)) = listener.accept().await {
        let state = Arc::clone(&state);
        spawn(async move { handle_client_connection(stream, state).await.unwrap() });
    }

    Ok(())
}

#[instrument(err, skip_all)]
async fn handle_client_connection(mut client_stream: TcpStream, state: Arc<State>) -> Result<()> {
    let connection_start = Instant::now();
    let mut buffer = vec![0u8; 16384];
    let len = client_stream
        .read(&mut buffer)
        .await
        .context("failed reading TLS header from stream")?;
    buffer.truncate(len);

    let tls_client_hello =
        ClientHello::try_from(buffer.as_slice()).context("failed parsing TLS header")?;
    let sni = tls_client_hello
        .sni()
        .context("TLS client hello does not contain SNI")?;
    let ja4_fingerprint = Ja4Fingerprint::calculate(&tls_client_hello);
    let peer_addr = client_stream.peer_addr()?;

    info!(
        sni,
        ja4 = ja4_fingerprint.as_ref(),
        ?peer_addr,
        "got TLS connection"
    );

    info!("sni extracted: {:?}", connection_start.elapsed());

    /*
    let addr = lookup_dns_v6(&sni).await?;

    info!("lookup ready: {:?}", connection_start.elapsed());

    let mut server_stream = TcpStream::connect(addr)
        .await
        .with_context(|| format!("can't connect to {sni} on {addr}"))?;

    info!("connected: {:?}", connection_start.elapsed());
     */

    let (mut client_read, mut client_write) = client_stream.into_split();

    let (mut server_stream, server_ref) = state
        .pools
        .get(sni)
        .context("domain is not configured")?
        .get_connection()
        .await?;

    server_stream
        .write_all(&buffer)
        .await
        .context("failed transferring TLS header from client to server")?;

    info!("header written: {:?}", connection_start.elapsed());

    //copy_bidirectional(&mut server_stream, &mut client_stream).await?;

    let (mut server_read, mut server_write) = server_stream.into_split();

    let client_read_ref = &mut client_read;
    let client_write_ref = &mut client_write;

    try_join!(
        async move {
            // TODO splice/sendfile?
            match copy(client_read_ref, &mut server_write)
                .await
                .context("failed transferring data from client to server")?
            {
                0 => {
                    bail!("client closed connection before sending any additional data");
                }
                _ => Ok(()),
            }
        },
        async move {
            // TODO splice/sendfile?
            match copy(&mut server_read, client_write_ref)
                .await
                .context("failed transferring data from server to client")?
            {
                0 => {
                    bail!("server closed connection before replying");
                }
                _ => Ok(()),
            }
        }
    )?;

    // reset counters
    drop(server_ref);

    info!("finish: {:?}", connection_start.elapsed());

    Ok(())
}

#[instrument(err, ret, level = Level::DEBUG)]
async fn lookup_dns_v6(sni: &str) -> Result<SocketAddrV6> {
    info!("looking up");
    let addrs: Vec<_> = lookup_host(format!("{sni}:443"))
        .await
        .with_context(|| format!("no DNS entry for {sni:?}"))?
        .collect();
    info!("{addrs:?}");
    let addr = addrs
        .iter()
        .filter_map(|addr| match addr {
            SocketAddr::V4(_) => None, // that's me!
            SocketAddr::V6(addr) => Some(addr),
        })
        .next()
        .with_context(|| format!("{sni} does not have a valid IPv6 address"))?;
    Ok(*addr)
}
