use crate::imports::*;
// use kaspa_rpc_core::model::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    Test,

    Connect {
        #[serde(rename = "networkId")]
        network_id: NetworkId,
        /// Node RPC url on which connection
        /// has been established
        url: Option<String>,
    },
    /// RPC disconnection
    Disconnect {
        #[serde(rename = "networkId")]
        network_id: NetworkId,
        url: Option<String>,
    },

    /// Data processor has started
    Start,
    /// Data processor is synced
    Synced,
    /// Data processor has stopped (disconnected)
    Stop,
    /// DAA Score has changed
    DaaScoreChange {
        current_daa_score: u64,
    },
    /// Protocol transaction has been detected
    Transaction {
        transaction: Box<RpcTransaction>,
    },
}
