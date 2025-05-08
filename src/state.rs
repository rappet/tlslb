use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use anyhow::{Context, Result};
use futures::FutureExt;
use tokio::net::TcpStream;
use tracing::error;

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

pub struct Pool {
    pub connections: Arc<parking_lot::Mutex<VecDeque<TcpStream>>>,
    pub config: Arc<Backend>,
}

impl Pool {
    pub fn new(config: Arc<Backend>) -> Self {
        let pool = Self {
            connections: Arc::new(Default::default()),
            config,
        };

        for _ in 0..3 {
            pool.request_connection();
        }

        pool
    }

    pub fn request_connection(&self) {
        let connections = Arc::clone(&self.connections);
        let sock_addr = *self.config.addresses.first().unwrap();

        tokio::spawn(async move {
            match TcpStream::connect(sock_addr).await {
                Ok(connection) => {
                    connection.set_nodelay(true).unwrap();
                    connections.lock().push_back(connection);
                }
                Err(err) => {
                    error!(?err, ?sock_addr, "failed to request connection");
                }
            }
        });
    }

    pub async fn get_connection(&self) -> Result<TcpStream> {
        while let Some(conn) = self.connections.lock().pop_front() {
            self.request_connection();

            // that's how you check if a socket is closed
            let mut buf = [0u8; 16];
            if let Some(Ok(0)) = conn.peek(&mut buf).now_or_never() {
                error!("connection was closed by remote - try next connection");
            } else {
                return Ok(conn);
            }
        }

        // fallback if pool is empty

        let sock_addr = *self.config.addresses.first().unwrap();
        let connection = TcpStream::connect(sock_addr)
            .await
            .context("pool is empty and failed to open connection as fallback")?;
        connection.set_nodelay(true)?;
        Ok(connection)
    }
}
