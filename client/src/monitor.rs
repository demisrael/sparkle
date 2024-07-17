use crate::imports::*;
pub use async_channel::{
    bounded, unbounded, Receiver, RecvError, SendError, Sender, TryRecvError, TrySendError,
};
pub use futures::{select, select_biased, FutureExt, Stream, StreamExt, TryStreamExt};
use kaspa_addresses::Address;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use workflow_core::channel::{oneshot, Channel, DuplexChannel};
use workflow_core::task::spawn;
use workflow_log::prelude::*;

use kaspa_wrpc_client::prelude::*;

struct Inner {
    task_ctl: DuplexChannel<()>,
    client: Arc<KaspaRpcClient>,
    is_connected: AtomicBool,
    notification_channel: Channel<Notification>,
    listener_id: Mutex<Option<ListenerId>>,
    notifier: Sender<()>,
    lookup: Address,
}

#[derive(Clone)]
pub struct Listener {
    inner: Arc<Inner>,
}

impl Listener {
    pub fn try_new(
        network_id: NetworkId,
        url: Option<String>,
        sender: Sender<()>,
        // stopper: Receiver<()>,
        lookup: Address,
    ) -> Result<Self> {
        let (resolver, url) = if let Some(url) = url {
            (None, Some(url))
        } else {
            (Some(Resolver::default()), None)
        };

        // Create a basic Kaspa RPC client instance using Borsh encoding.
        let client = Arc::new(KaspaRpcClient::new_with_args(
            WrpcEncoding::Borsh,
            url.as_deref(),
            resolver,
            Some(network_id),
            None,
        )?);

        let inner = Inner {
            task_ctl: DuplexChannel::oneshot(),
            client,
            is_connected: AtomicBool::new(false),
            notification_channel: Channel::unbounded(),
            listener_id: Mutex::new(None),
            notifier: sender,
            lookup: lookup.clone(),
        };
        println!("Monitor: {}", lookup);
        Ok(Self {
            inner: Arc::new(inner),
        })
    }

    fn is_connected(&self) -> bool {
        self.inner.is_connected.load(Ordering::SeqCst)
    }

    async fn start(&self) -> Result<()> {
        let options = ConnectOptions {
            block_async_connect: false,
            ..Default::default()
        };
        self.start_event_task().await?;
        self.client().connect(Some(options)).await?;
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        println!("Stopping monitor");
        // Disconnect the RPC client
        self.client().disconnect().await?;
        // make sure to stop the event task after
        // the RPC client is disconnected to receive
        // and handle disconnection events.
        self.stop_event_task().await?;
        Ok(())
    }

    pub fn client(&self) -> &Arc<KaspaRpcClient> {
        &self.inner.client
    }

    async fn register_notification_listeners(&self) -> Result<()> {
        let listener_id = self
            .client()
            .rpc_api()
            .register_new_listener(ChannelConnection::new(
                "wrpc-example-subscriber",
                self.inner.notification_channel.sender.clone(),
                ChannelType::Persistent,
            ));
        *self.inner.listener_id.lock().unwrap() = Some(listener_id);
        let address = vec![self.inner.lookup.clone()];
        self.client()
            .rpc_api()
            .start_notify(
                listener_id,
                Scope::UtxosChanged(UtxosChangedScope { addresses: address }),
            )
            .await?;
        Ok(())
    }

    async fn unregister_notification_listener(&self) -> Result<()> {
        let listener_id = self.inner.listener_id.lock().unwrap().take();
        if let Some(id) = listener_id {
            self.client().rpc_api().unregister_listener(id).await?;
        }
        Ok(())
    }

    async fn handle_notification(&self, notification: Notification) -> Result<()> {
        match notification {
            Notification::BlockAdded(_block_notification) => {}
            Notification::VirtualDaaScoreChanged(_virtual_daa_score_changed_notification) => {}

            Notification::UtxosChanged(utxos_changed_notification) => {
                for _utxo in utxos_changed_notification.added.iter() {
                    self.inner
                        .notifier
                        .try_send(())
                        .expect("Error sending shutdown signal...");
                }
            }
            _ => {
                log_warn!("unknown notification: {:?}", notification);
            }
        }
        Ok(())
    }

    async fn handle_connect(&self) -> Result<()> {
        println!("Connected to {:?}", self.client().url());
        let server_info = self.client().get_server_info().await?;
        log_info!("Server info: {server_info:?}");
        self.register_notification_listeners().await?;
        self.inner.is_connected.store(true, Ordering::SeqCst);
        Ok(())
    }

    async fn handle_disconnect(&self) -> Result<()> {
        println!("Disconnected from {:?}", self.client().url());
        self.unregister_notification_listener().await?;
        self.inner.is_connected.store(false, Ordering::SeqCst);
        Ok(())
    }

    async fn start_event_task(&self) -> Result<()> {
        let listener = self.clone();
        let rpc_ctl_channel = self.client().rpc_ctl().multiplexer().channel();
        let task_ctl_receiver = self.inner.task_ctl.request.receiver.clone();
        let task_ctl_sender = self.inner.task_ctl.response.sender.clone();
        let notification_receiver = self.inner.notification_channel.receiver.clone();

        spawn(async move {
            loop {
                select_biased! {
                    msg = rpc_ctl_channel.receiver.recv().fuse() => {
                        match msg {
                            Ok(msg) => {
                                match msg {
                                    RpcState::Connected => {
                                        if let Err(err) = listener.handle_connect().await {
                                            log_error!("Error in connect handler: {err}");
                                        }
                                    },
                                    RpcState::Disconnected => {
                                        if let Err(err) = listener.handle_disconnect().await {
                                            log_error!("Error in disconnect handler: {err}");
                                        }
                                    }
                                }
                            }
                            Err(err) => {
                                log_error!("RPC CTL channel error: {err}");
                                panic!("Unexpected: RPC CTL channel closed, halting...");
                            }
                        }
                    }
                    notification = notification_receiver.recv().fuse() => {
                        match notification {
                            Ok(notification) => {
                                if let Err(err) = listener.handle_notification(notification).await {
                                    log_error!("Error while handling notification: {err}");
                                }
                            }
                            Err(err) => {
                                panic!("RPC notification channel error: {err}");
                            }
                        }
                    },
                    _ = task_ctl_receiver.recv().fuse() => {
                        break;
                    },

                }
            }

            log_info!("Event task existing...");
            if listener.is_connected() {
                listener
                    .handle_disconnect()
                    .await
                    .unwrap_or_else(|err| log_error!("{err}"));
            }
            task_ctl_sender.send(()).await.unwrap();
        });
        Ok(())
    }

    async fn stop_event_task(&self) -> Result<()> {
        self.inner
            .task_ctl
            .signal(())
            .await
            .expect("stop_event_task() signal error");
        Ok(())
    }
}

pub async fn monitor(lookup: Address) -> Result<(Listener, async_channel::Receiver<()>)> {
    let (notifier, notify_receiver) = oneshot();
    // let (stopper, stopper_receiver) = oneshot();
    let url = "ws://192.168.178.36:17210".to_string();
    let listener = Listener::try_new(
        NetworkId::with_suffix(NetworkType::Testnet, 11),
        Some(url),
        notifier,
        // stopper_receiver,
        lookup,
    )
    .map_err(|e| Error::ListenerError(e.to_string()))?;

    listener
        .start()
        .await
        .map_err(|e| Error::ListenerError(e.to_string()))?;

    Ok((listener, notify_receiver))
}
