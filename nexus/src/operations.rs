use crate::constants::PROTOCOL_ID;
use crate::optypes::Krc20OpType;
use serde::{Deserialize, Serialize};

#[derive(thiserror::Error, Debug)]
pub enum Krc20OperationsError {
    #[error("Invalid ticker format: {0}")]
    InvalidTickerFormat(String),
}

struct Operation {
    op_type: Krc20OpType,
    base_data: BaseData,
}

impl Operation {
    pub fn new(operation_type: Krc20OpType, ticker: String) -> Self {
        let _validated_ticker = Self::validate_ticker(ticker.clone());
        let base_data = BaseData {
            p: PROTOCOL_ID.to_string(),
            op: operation_type.to_string(),
            tick: ticker.to_string(),
            max: None,
            lim: None,
            dec: None,
            amt: None,
        };
        Self {
            op_type: operation_type,
            base_data,
        }
    }

    fn validate_ticker(ticker: String) -> Result<bool, Krc20OperationsError> {
        let is_ascii_alpha = ticker.bytes().all(|b| b.is_ascii_lowercase());
        let is_4_to_6_chars_long = ticker.len() >= 4 && ticker.len() <= 6;

        match (is_ascii_alpha, is_4_to_6_chars_long) {
            (true, true) => Ok(true),
            (_, _) => Err(Krc20OperationsError::InvalidTickerFormat(ticker)),
        }
    }

    fn data(&self) -> BaseData {
        self.validate();
        self.base_data.clone()
    }

    fn set_max(&mut self, cap: u64) {
        self.base_data.max = Some(cap);
    }

    fn set_lim(&mut self, mint_limit: u64) {
        self.base_data.lim = Some(mint_limit);
    }

    fn set_amt(&mut self, transfer_amount: u64) {
        self.base_data.amt = Some(transfer_amount);
    }

    fn set_dec(&mut self, decimals: u8) {
        self.base_data.dec = Some(decimals);
    }

    fn validate(&self) -> bool {
        match self.op_type {
            Krc20OpType::Deploy => self.base_data.max.is_some() && self.base_data.lim.is_some(),
            Krc20OpType::Mint => true,
            Krc20OpType::Transfer => self.base_data.amt.is_some(),
        }
    }

    fn build_deploy(ticker: String, cap: u64, mint_limit: u64, decimals: Option<u8>) -> DeployData {
        let mut operation = Self::new(Krc20OpType::Deploy, ticker);
        operation.set_max(cap);
        operation.set_lim(mint_limit);
        if let Some(dec) = decimals {
            operation.set_dec(dec)
        };
        operation.data().into()
    }
    fn build_mint(ticker: String) -> MintData {
        let operation = Self::new(Krc20OpType::Mint, ticker);
        operation.data().into()
    }
    fn build_transfer(ticker: String, transfer_amount: u64) -> TransferData {
        let mut operation = Self::new(Krc20OpType::Transfer, ticker);
        operation.set_amt(transfer_amount);
        operation.data().into()
    }
}

impl From<BaseData> for TransferData {
    fn from(val: BaseData) -> Self {
        TransferData {
            p: val.p,
            op: val.op,
            tick: val.tick,
            amt: val.amt.unwrap(),
        }
    }
}

impl From<BaseData> for DeployData {
    fn from(val: BaseData) -> Self {
        DeployData {
            p: val.p,
            op: val.op,
            tick: val.tick,
            max: val.max.unwrap(),
            lim: val.lim.unwrap(),
            dec: val.dec,
        }
    }
}

impl From<BaseData> for MintData {
    fn from(val: BaseData) -> Self {
        MintData {
            p: val.p,
            op: val.op,
            tick: val.tick,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseData {
    p: String,
    op: String,
    tick: String,
    max: Option<u64>,
    lim: Option<u64>,
    amt: Option<u64>,
    dec: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeployData {
    p: String,
    op: String,
    tick: String,
    max: u64,
    lim: u64,
    dec: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MintData {
    p: String,
    op: String,
    tick: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TransferData {
    p: String,
    op: String,
    tick: String,
    amt: u64,
}

pub fn build_deploy_json_example() -> String {
    let payload = Operation::build_deploy("test".to_owned(), 21_000, 100, None);
    serde_json::to_string(&payload).unwrap()
}

pub fn build_mint_json_example() -> String {
    let payload = Operation::build_mint("test".to_owned());
    serde_json::to_string(&payload).unwrap()
}

pub fn build_transfer_json_example() -> String {
    let payload = Operation::build_transfer("test".to_owned(), 1_000);
    serde_json::to_string(&payload).unwrap()
}

pub fn deserialize(value: &[u8]) -> Option<BaseData> {
    // let data:  =
    serde_json::from_slice(value).unwrap_or(None).clone()
    // data
}
