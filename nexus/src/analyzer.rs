use crate::imports::*;
// use kaspa_rpc_core::model::*;

pub enum AnalyzerEvent {
    Transaction(Transaction),
}

pub struct Inner {
    pub shutdown: DuplexChannel<()>,
    pub nexus: Nexus,
}

#[derive(Clone)]
pub struct Analyzer {
    inner: Arc<Inner>,
}

impl Analyzer {
    pub async fn try_new(nexus: &Nexus) -> Result<Self> {
        let inner = Inner {
            shutdown: DuplexChannel::oneshot(),
            nexus: nexus.clone(),
        };

        let analyzer = Self {
            inner: Arc::new(inner),
        };

        Ok(analyzer)
    }

    pub fn nexus(&self) -> &Nexus {
        &self.inner.nexus
    }

    async fn task(self: Arc<Self>) -> Result<()> {
        let events = self.nexus().multiplexer().channel();

        loop {
            select_biased! {
                msg = events.receiver.recv().fuse() => {
                    match msg {
                        Ok(msg) => {

                            // handle RPC channel connection and disconnection events
                            #[allow(clippy::single_match)]
                            match &*msg {
                                Event::Transaction { transaction } => {

                                    let txid = transaction.verbose_data.as_ref().map(|data| data.transaction_id.to_string()).unwrap_or_else(||"N/A".to_string());
                                    println!("Received transaction: {txid}");

                                },
                                _ => { } // consume unrelated events
                            }
                        }
                        Err(err) => {
                            log_error!("Analyzer channel failure: {err}");
                        }
                    }
                }

                _ = self.inner.shutdown.request.recv().fuse() => {
                    break;
                },

            }
        }

        self.inner.shutdown.response.send(()).await?;

        Ok(())
    }
}

const SERVICE: &str = "ANALYZER";

#[async_trait]
impl Service for Analyzer {
    async fn spawn(self: Arc<Self>, _runtime: Runtime) -> ServiceResult<()> {
        // log_trace!("starting {SERVICE}...");

        task::spawn(async move {
            self.task()
                .await
                .unwrap_or_else(|err| log_error!("{SERVICE} error: {err}"));
        });

        Ok(())
    }

    fn terminate(self: Arc<Self>) {
        // log_trace!("sending an exit signal to {}", SERVICE);
        self.inner.shutdown.request.try_send(()).unwrap();
    }

    async fn join(self: Arc<Self>) -> ServiceResult<()> {
        self.inner.shutdown.response.recv().await?;
        Ok(())
    }
}
