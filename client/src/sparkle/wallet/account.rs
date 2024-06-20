use kaspa_wallet_core::prelude::{sompi_to_kaspa_string_with_suffix, AccountDescriptor};
use pad::{Alignment, PadStr};
use sparkle_rs::imports::*;
use sparkle_rs::result::Result;
use std::str::FromStr;

pub struct Balance(String, Option<String>, String);

impl Balance {
    pub fn format(&self, cols: (usize, usize, usize)) -> String {
        [
            self.0.pad_to_width_with_alignment(cols.0, Alignment::Right),
            self.1
                .as_ref()
                .map(|s| format!(".{}", s))
                .unwrap_or_default()
                .pad_to_width_with_alignment(cols.1, Alignment::Left),
            self.2.pad_to_width_with_alignment(cols.2, Alignment::Right),
        ]
        .join("")
    }

    pub fn len(&self) -> (usize, usize, usize) {
        (
            self.0.len() + 1,
            self.1.as_ref().map(|s| s.len()).unwrap_or_default(),
            self.2.len() + 1,
        )
    }
}

impl FromStr for Balance {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let parts = s.split_whitespace().collect::<Vec<&str>>();
        let mut parts = parts.iter();
        let digits = parts.next().unwrap().split('.').collect::<Vec<_>>();
        let mut digits = digits.iter();
        let integer = digits.next().map(|v| v.to_string()).unwrap();
        let fraction = digits.next().map(|v| v.to_string());

        let suffix = parts.next().map(|v| v.to_string()).unwrap_or_default();
        Ok(Self(integer, fraction, suffix))
    }
}

pub struct Account {
    pub descriptor: Arc<AccountDescriptor>,
    pub short_id: String,
    pub balance: Option<Balance>,
}

impl Account {
    pub fn new(descriptor: AccountDescriptor, network_id: &NetworkId) -> Self {
        let short_id = descriptor.account_id().to_hex()[0..8].to_string();
        let balance: Option<Balance> = descriptor.balance.as_ref().map(|balance| {
            sompi_to_kaspa_string_with_suffix(balance.mature, &(*network_id).into())
                .parse()
                .unwrap()
        });
        Self {
            descriptor: Arc::new(descriptor),
            short_id,
            balance,
        }
    }
}
