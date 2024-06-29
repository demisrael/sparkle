use crate::imports::*;
// use std::sync::mpsc;
use std::thread;
// use workflow_core::
use kaspa_rpc_core::{
    VirtualChainChangedNotification, //, BlockAddedNotification,
};
use sparkle_database::prelude::*;
use std::fs;

pub enum Ingest {
    // NoOp,
    // Block(Block),
    VirtualChainChanged(Arc<VirtualChainChangedNotification>),
    Transaction(Arc<RpcTransaction>),

    Halt,
}

struct Inner {
    sender: mpsc::Sender<Ingest>,
    receiver: Mutex<Option<mpsc::Receiver<Ingest>>>,
    ingest: Mutex<Option<thread::JoinHandle<()>>>,
    utxo_db: Arc<Db>,
}

#[derive(Clone)]
pub struct Processor {
    inner: Arc<Inner>,
}

impl Processor {
    pub fn try_new() -> Result<Self> {
        let db_dir = get_app_dir().join("db");
        // fs::create_dir_all(&folder_db)?;
        // if !folder_db.exists() {
        // }

        let db_dir_utxo = db_dir.join("utxo");
        fs::create_dir_all(&db_dir_utxo)?;

        println!("db_dir_utxo: {:?}", db_dir_utxo);

        println!("utxo_db init...");

        let utxo_db = ConnBuilder::default()
            .with_parallelism(num_cpus::get())
            // .with_files_limit(default_fd)
            // TODO: set the files limit
            .with_files_limit(1000)
            .with_db_path(db_dir_utxo)
            .with_create_if_missing(true)
            .build()
            .unwrap();

        println!("utxo_db init done");

        // let db = load_existing_db!(input_dir, conn_builder);

        let (sender, receiver) = mpsc::channel();

        Ok(Self {
            inner: Arc::new(Inner {
                sender,
                receiver: Mutex::new(Some(receiver)),
                ingest: Default::default(),
                utxo_db,
            }),
        })
    }

    pub fn sender(&self) -> mpsc::Sender<Ingest> {
        self.inner.sender.clone()
    }

    pub fn ingest(&self) -> Result<()> {
        let receiver = self.inner.receiver.lock().unwrap().take().unwrap();

        loop {
            match receiver.recv() {
                Ok(msg) => match msg {
                    Ingest::Transaction(_tx) => {
                        // println!("[PROC] Received transaction: {:?}", tx);
                    }
                    Ingest::VirtualChainChanged(_vcc) => {
                        // println!("[PROC] Received virtual chain changed: {:?}", vcc);
                    }
                    Ingest::Halt => {
                        break;
                    }
                },
                // Ok(Ingest::Block(block)) => {
                //     self.process_block(block);
                // }
                // Ok(Ingest::Transaction(transaction)) => {
                //     self.process_transaction(transaction);
                // }
                Err(err) => {
                    log_error!("[PROC] error receiving message: {err}");
                    break;
                }
            }
        }
        Ok(())
    }
}

const SERVICE: &str = "PROC";

#[async_trait]
impl Service for Processor {
    async fn spawn(self: Arc<Self>, _runtime: Runtime) -> ServiceResult<()> {
        let this = self.clone();
        let thread = thread::Builder::new()
            .name("ingest".to_string())
            .spawn(move || {
                this.ingest()
                    .unwrap_or_else(|err| log_error!("{SERVICE} error: {err}"));
            })
            .expect("failed to spawn ingest thread");
        self.inner.ingest.lock().unwrap().replace(thread);

        Ok(())
    }

    fn terminate(self: Arc<Self>) {
        // log_trace!("sending an exit signal to {SERVICE}");
        // self.inner.shutdown.request.try_send(()).unwrap();
        self.inner.sender.send(Ingest::Halt).unwrap();
    }

    async fn join(self: Arc<Self>) -> ServiceResult<()> {
        let thread = self.inner.ingest.lock().unwrap().take();
        if let Some(thread) = thread {
            spawn_blocking(move || {
                thread.join().unwrap();
            })
            .await
            .map_err(ServiceError::custom)?;
            // thread.join().unwrap();
        }
        // self.inner.shutdown.response.recv().await?;
        Ok(())
    }
}
