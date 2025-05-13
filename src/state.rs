use std::{
    collections::{HashMap, VecDeque},
    net::SocketAddr,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    time::Duration,
};

use anyhow::{Context, Result};
use futures::FutureExt;
use socket2::{SockRef, TcpKeepalive};
use tokio::net::TcpStream;
use tracing::{error, warn};

use crate::config::{Backend, Config};

pub struct State {
    pub pools: HashMap<String, Pool>,
}

impl State {
    pub fn new(config: Arc<Config>) -> Self {
        let mut pools = HashMap::new();

        for (domain, backend) in &config.backends {
            pools.insert(domain.clone(), Pool::new(Arc::clone(backend)));
        }

        Self { pools }
    }
}

pub struct BackendState {
    pub addr: SocketAddr,
    pub open_connections: AtomicU32,
}

impl BackendState {
    pub fn new(addr: SocketAddr) -> Self {
        Self {
            addr,
            open_connections: AtomicU32::new(0),
        }
    }
}

pub struct ConnectionRef {
    backend_state: Arc<BackendState>,
}

impl ConnectionRef {
    pub fn new(backend_state: Arc<BackendState>) -> Self {
        backend_state
            .open_connections
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self { backend_state }
    }
}

impl Drop for ConnectionRef {
    fn drop(&mut self) {
        self.backend_state
            .open_connections
            .fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }
}

pub struct Pool {
    pub backends: Vec<Arc<BackendState>>,
    pub slots: Arc<parking_lot::Mutex<VecDeque<(TcpStream, ConnectionRef)>>>,
    pub config: Arc<Backend>,
}

impl Pool {
    pub fn new(config: Arc<Backend>) -> Self {
        let backends = config
            .addresses
            .iter()
            .map(|addr| Arc::new(BackendState::new(*addr)))
            .collect();

        let pool = Self {
            backends,
            slots: Arc::new(Default::default()),
            config,
        };

        for _ in 0..3 {
            pool.request_connection();
        }

        pool
    }

    pub fn request_connection(&self) {
        let connections = Arc::clone(&self.slots);
        let backend = self
            .backends
            .iter()
            .min_by_key(|state| state.open_connections.load(Ordering::Relaxed))
            .expect("pool has at least one backend");
        let sock_addr = backend.addr;
        let connection_ref = ConnectionRef::new(Arc::clone(backend));

        tokio::spawn(async move {
            match TcpStream::connect(sock_addr).await {
                Ok(connection) => {
                    connection.set_nodelay(true).unwrap();
                    let sf = SockRef::from(&connection);
                    sf.set_keepalive(true).unwrap();
                    let ka = TcpKeepalive::new().with_time(Duration::from_secs(30));
                    sf.set_tcp_keepalive(&ka).unwrap();
                    connections.lock().push_back((connection, connection_ref));
                }
                Err(err) => {
                    error!(?err, ?sock_addr, "failed to request connection");
                }
            }
        });
    }

    pub async fn get_connection(&self) -> Result<(TcpStream, ConnectionRef)> {
        while let Some((conn, connection_ref)) = self.slots.lock().pop_front() {
            self.request_connection();

            // that's how you check if a socket is closed
            let mut buf = [0u8; 16];
            if let Some(Ok(0)) = conn.peek(&mut buf).now_or_never() {
                warn!("connection was closed by remote - try next connection");
            } else {
                return Ok((conn, connection_ref));
            }
        }

        // fallback if pool is empty

        let backend = self
            .backends
            .iter()
            .min_by_key(|state| state.open_connections.load(Ordering::Relaxed))
            .expect("pool has at least one backend");
        let sock_addr = backend.addr;
        let connection_ref = ConnectionRef::new(Arc::clone(backend));
        let connection = TcpStream::connect(sock_addr)
            .await
            .context("pool is empty and failed to open connection as fallback")?;
        connection.set_nodelay(true)?;
        Ok((connection, connection_ref))
    }
}
