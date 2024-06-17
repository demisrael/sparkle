// use crate::{connection::Connection, result::Result, service::WrpcOptions};
use crate::{connection::Connection, service::WrpcOptions};
use async_trait::async_trait;
use sparkle_nexus::prelude::Nexus;
use sparkle_rpc_core::ops::RpcApiOps;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};
use workflow_rpc::server::prelude::*;

struct Inner {
    pub nexus: Nexus,
    pub options: Arc<WrpcOptions>,
    // pub _encoding: Encoding,
    // pub handler: Arc<Server>,
    pub next_connection_id: AtomicU64,
    pub sockets: Mutex<HashMap<u64, Connection>>,
}

#[derive(Clone)]
pub struct Server {
    inner: Arc<Inner>,
}

#[async_trait]
impl RpcHandler for Server {
    type Context = Connection;

    async fn handshake(
        self: Arc<Self>,
        peer: &SocketAddr,
        _sender: &mut WebSocketSender,
        _receiver: &mut WebSocketReceiver,
        messenger: Arc<Messenger>,
    ) -> WebSocketResult<Connection> {
        println!("WebSocket connected: {}", peer);
        let id = self.inner.next_connection_id.fetch_add(1, Ordering::SeqCst);
        let connection = Connection::new(id, peer, messenger);
        self.inner
            .sockets
            .lock()
            .map_err(|err| err.to_string())?
            .insert(id, connection.clone());

        Ok(connection)
    }

    async fn disconnect(self: Arc<Self>, connection: Self::Context, _result: WebSocketResult<()>) {
        self.inner.sockets.lock().unwrap().remove(&connection.id());
    }
}

impl Server {
    pub fn new(
        nexus: &Nexus,
        options: Arc<WrpcOptions>,
        // _encoding: Encoding,
        // handler : Arc<Server>,
    ) -> Self {
        Server {
            inner: Arc::new(Inner {
                // _encoding,
                // handler,
                nexus: nexus.clone(),
                options,
                next_connection_id: AtomicU64::new(0),
                sockets: Mutex::new(HashMap::new()),
            }),
        }
    }

    pub fn nexus(&self) -> &Nexus {
        &self.inner.nexus
    }

    pub fn handler(&self, _op: RpcApiOps, _connection: &Connection) -> ServerResult<&Nexus> {
        Ok(&self.inner.nexus)
    }

    pub fn verbose(&self) -> bool {
        self.inner.options.verbose
    }
}
