use crate::constants::{FEE_DEPLOY, FEE_MINT};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(thiserror::Error, Debug)]
pub enum Krc20OpTypeError {
    #[error("Invalid op type: {0}")]
    InvalidOpType(String),
}

pub enum Krc20OpType {
    Deploy,
    Mint,
    Transfer,
}

impl Krc20OpType {
    pub fn additional_fee(&self) -> u64 {
        match self {
            Krc20OpType::Deploy => FEE_DEPLOY,
            Krc20OpType::Mint => FEE_MINT,
            Krc20OpType::Transfer => 0,
        }
    }
}

impl Display for Krc20OpType {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Krc20OpType::Mint => "mint",
            Krc20OpType::Deploy => "deploy",
            Krc20OpType::Transfer => "transfer",
        };
        f.write_str(s)
    }
}

impl FromStr for Krc20OpType {
    type Err = Krc20OpTypeError;
    fn from_str(op_type: &str) -> Result<Self, Self::Err> {
        match op_type.to_lowercase().as_str() {
            "deploy" => Ok(Krc20OpType::Deploy),
            "mint" => Ok(Krc20OpType::Mint),
            "transfer" => Ok(Krc20OpType::Transfer),
            _ => Err(Krc20OpTypeError::InvalidOpType(op_type.to_string())),
        }
    }
}
