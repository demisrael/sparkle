use crate::imports::*;
use serde_json::from_slice;
use sparkle_core::inscription::ascii_debug_payload;
use sparkle_core::model::kasplex;
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

                                    if let Some(token) = detect_krc20(transaction){

                                        if token.has_tick("toitoi") {
                                            println!("Filter tick");
                                            dbg!(&token);
                                        }
                                        // Debug
                                        if token.op == kasplex::v1::krc20::Op::Deploy {
                                            println!("Filter deploy");
                                            dbg!(&token);
                                        }

                                    }
                                    // let txid = transaction.verbose_data.as_ref().map(|data| data.transaction_id.to_string()).unwrap_or_else(||"N/A".to_string());
                                    // println!("Received transaction: {txid}");

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

#[inline]
fn window_find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    // Ensure we don't start beyond the end of the haystack
    let offset = 10;
    if haystack.len() <= offset {
        return None;
    }

    // Optization: iterate starting from the nth byte
    for (position, window) in haystack[offset..].windows(needle.len()).enumerate() {
        if window == needle {
            return Some(position + offset); // Adjust the position to account for the byte offset
        }
    }
    None
}

fn parse_script<T: VerifiableTransaction>(
    script: &[u8],
) -> impl Iterator<Item = std::result::Result<Box<dyn OpCodeImplementation<T>>, TxScriptError>> + '_
{
    script.iter().batching(|it| deserialize_next_opcode(it))
}

pub trait ITransaction {
    fn signature_script(&self) -> Option<&[u8]>;
    fn rcv(&self) -> Address;
}

impl ITransaction for &RpcTransaction {
    fn signature_script(&self) -> Option<&[u8]> {
        Some(&self.inputs[0].signature_script[..])
    }
    fn rcv(&self) -> Address {
        extract_script_pub_key_address(
            &self.outputs[0].script_public_key,
            Prefix::try_from("kaspatest").unwrap(),
        )
        .unwrap()
    }
}

impl ITransaction for &Transaction {
    fn signature_script(&self) -> Option<&[u8]> {
        Some(&self.inputs[0].signature_script[..])
    }
    fn rcv(&self) -> Address {
        extract_script_pub_key_address(
            &self.outputs[0].script_public_key,
            Prefix::try_from("kaspatest").unwrap(),
        )
        .unwrap()
    }
}
impl ITransaction for &Box<RpcTransaction> {
    fn signature_script(&self) -> Option<&[u8]> {
        if self.inputs.is_empty() {
            return None;
        }
        Some(&self.inputs[0].signature_script[..])
    }
    fn rcv(&self) -> Address {
        extract_script_pub_key_address(
            &self.outputs[0].script_public_key,
            Prefix::try_from("kaspatest").unwrap(),
        )
        .unwrap()
    }
}

pub fn detect_krc20_header(haystack: &[u8]) -> bool {
    window_find(haystack, &KRC20_HEADER_UC).is_some()
        || window_find(haystack, &KRC20_HEADER_LC).is_some()
}

pub fn detect_kasplex_header(haystack: &[u8]) -> bool {
    window_find(haystack, &KASPLEX_HEADER_LC).is_some()
        || window_find(haystack, &KASPLEX_HEADER_UC).is_some()
}

pub fn detect_krc20_receiver<T: ITransaction>(sigtx: T) -> Address {
    sigtx.rcv()
}

pub fn detect_krc20<T: ITransaction>(sigtx: T) -> Option<TokenTransaction> {
    let mut inscription: Option<TokenTransaction> = None;

    if let Some(signature_script) = sigtx.signature_script() {
        if detect_kasplex_header(signature_script) {
            // Get the second opcode
            let mut opcodes_iter = parse_script(signature_script);
            let second_opcode: Option<
                std::result::Result<
                    Box<dyn OpCodeImplementation<PopulatedTransaction>>,
                    TxScriptError,
                >,
            > = opcodes_iter.nth(1);

            // println!("------------------ {} {}", sigtx.gas(), sigtx.mass());

            match second_opcode {
                Some(Ok(opcode)) => {
                    if !opcode.is_empty()
                        && opcode.is_push_opcode()
                        && detect_krc20_header(opcode.get_data())
                    {
                        let inner_opcodes: Vec<_> =
                            parse_script::<PopulatedTransaction>(opcode.get_data()).collect();
                        if inner_opcodes.len() >= 2 {
                            if let Some(Ok(second_to_last_opcode)) =
                                inner_opcodes.get(inner_opcodes.len() - 2)
                            {
                                ascii_debug_payload(second_to_last_opcode.get_data());

                                println!("Receiver {:}", sigtx.rcv().address_to_string());

                                match from_slice::<TokenTransaction>(
                                    second_to_last_opcode.get_data(),
                                ) {
                                    Ok(token_transaction) => {
                                        // Debug
                                        if token_transaction.op == kasplex::v1::krc20::Op::Transfer
                                        {
                                            ascii_debug_payload(opcode.get_data());
                                        }
                                        // Debug
                                        if token_transaction.op == kasplex::v1::krc20::Op::Deploy {
                                            ascii_debug_payload(opcode.get_data());
                                        }
                                        // Debug
                                        if token_transaction.has_tick("toitoi") {
                                            ascii_debug_payload(opcode.get_data());
                                        }

                                        inscription = Some(token_transaction);
                                    }
                                    Err(e) => {
                                        ascii_debug_payload(second_to_last_opcode.get_data());

                                        // Handle the error if necessary
                                        eprintln!("Failed to deserialize: {:?}", e);
                                    }
                                }
                            }
                        }
                    }
                }
                Some(Err(e)) => {
                    // Handle the error
                    println!("Error while parsing opcodes: {:?}", e);
                }
                None => {
                    // Handle the case where there are fewer than two opcodes
                    println!("There are fewer than two opcodes in the script.");
                }
            }
        }
    }

    inscription
}
