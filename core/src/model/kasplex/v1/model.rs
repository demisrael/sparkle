use super::*;
use crate::imports::*;
use kaspa_wallet_core::utils::sompi_to_kaspa_string_with_suffix;
use std::fmt::Write;
// use serde::de::{self, Deserializer};
// use std::str::FromStr;

///
/// URL path: `//info`
///
/// ```json
/// {
///     "message": "text",
///     "result": {
///         "daaScore": 77993954,
///         "opScore": 779939200003,
///         "opTotal": 1200660,
///         "tokenTotal": 6502,
///         "feeTotal": 56036798346000
///     }
/// }
/// ```
///

#[derive(Debug, Deserialize)]
pub struct IndexerStatusResponse {
    pub message: String,
    pub result: IndexerStatus,
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct IndexerStatus {
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "daaScore")]
    pub daa_score: u64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "opScore")]
    pub op_score: u64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "opTotal")]
    pub op_total: u64,

    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "tokenTotal")]
    pub token_total: u64,

    // TODO - what is fee total?
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "feeTotal")]
    pub fee_total: u64,
}

impl IndexerStatus {
    pub fn format(&self, network: Network) -> String {
        let IndexerStatus {
            daa_score,
            op_score,
            op_total,
            token_total,
            fee_total,
        } = self;

        [
            ("DAA Score", daa_score),
            ("Op Score", op_score),
            ("Op Total", op_total),
            ("Tokens", token_total),
        ]
        .iter()
        .fold(String::new(), |mut output, (label, value)| {
            let _ = writeln!(output, "{:>12}: {}", label, value);
            output
        }) + &format!(
            "{:>12}: {}",
            "Fees",
            sompi_to_kaspa_string_with_suffix(*fee_total, &network.into()),
        )
    }
}

pub mod krc20 {
    use super::*;

    ///
    /// URL path: `/krc20/tokenlist`
    ///
    /// ```json
    /// {
    ///     "message": "text",
    ///     "prev": "text",
    ///     "next": "text",
    ///     "result": {
    ///         "tick": "KASP",
    ///         "max": 2100000000000000,
    ///         "lim": 100000000000,
    ///         "dec": 8,
    ///         "minted": 1500000000000000,
    ///         "opScoreAdd": 77993954,
    ///         "opScoreMod": 79993666,
    ///         "state": "deployed",
    ///         "hashRev": "eb1482705b07af..",
    ///         "mtsAdd": "1712808987852"
    ///     }
    /// }
    /// ```
    ///

    #[derive(Debug, Deserialize)]
    pub struct TokenListResponse {
        pub message: String,
        pub next: String,
        pub prev: String,
        pub result: Vec<Token>,
    }

    #[serde_as]
    #[derive(Debug, Deserialize)]
    pub struct Token {
        pub tick: String,
        #[serde_as(as = "DisplayFromStr")]
        pub max: u128,

        // TODO - rename to "limit"?
        #[serde_as(as = "DisplayFromStr")]
        #[serde(rename = "lim")]
        pub limit: u128,

        #[serde_as(as = "DisplayFromStr")]
        pub dec: u64,

        // TODO - rename to total_minted ?
        #[serde_as(as = "DisplayFromStr")]
        pub minted: u128,

        #[serde_as(as = "DisplayFromStr")]
        #[serde(rename = "opScoreAdd")]
        pub op_score_added: u64,

        #[serde_as(as = "DisplayFromStr")]
        #[serde(rename = "opScoreMod")]
        pub op_score_modified: u64,

        // TODO - what are the variants?
        pub state: State,

        #[serde(rename = "hashRev")]
        // TODO - rename to hash_revision ?
        pub hash_rev: Hash,

        #[serde(rename = "mtsAdd")]
        // TODO - what is mts ?  time stamp in milliseconds? should be `ts_msec_added`
        #[serde_as(as = "DisplayFromStr")]
        pub mts_add: u64,
    }

    ///
    /// URL path: `//krc20/token/{tick}`
    ///
    /// ```json
    /// {
    ///     "message": "text",
    ///     "result": {
    ///         "tick": "KASP",
    ///         "max": 2100000000000000,
    ///         "lim": 100000000000,
    ///         "dec": 8,
    ///         "minted": 1500000000000000,
    ///         "opScoreAdd": 77993954,
    ///         "opScoreMod": 79993666,
    ///         "state": "deployed",
    ///         "holder": [
    ///             {
    ///                 "address": "kaspa:qra0p5kyzeh54p37gqwfu...",
    ///                 "amount": "220000000000000"
    ///             }
    ///         ]
    ///     }
    /// }
    /// ```
    ///   

    #[derive(Debug, Deserialize)]
    pub struct TokenHolderResponse {
        pub message: String,
        pub result: TokenHolderResult,
    }

    #[serde_as]
    #[derive(Debug, Deserialize)]
    pub struct TokenHolderResult {
        pub tick: String,

        #[serde_as(as = "DisplayFromStr")]
        pub max: u128,

        #[serde_as(as = "DisplayFromStr")]
        pub lim: u128,

        #[serde_as(as = "DisplayFromStr")]
        pub dec: u64,

        #[serde_as(as = "DisplayFromStr")]
        #[serde(rename = "daas")]
        pub mint_start_daa_score: u64,

        #[serde_as(as = "DisplayFromStr")]
        #[serde(rename = "daae")]
        pub mint_end_daa_score: u64,

        #[serde_as(as = "DisplayFromStr")]
        pub minted: u128,

        #[serde_as(as = "DisplayFromStr")]
        #[serde(rename = "opScoreAdd")]
        pub op_score_add: u64,

        #[serde_as(as = "DisplayFromStr")]
        #[serde(rename = "opScoreMod")]
        pub op_score_mod: u64,

        pub state: State,

        pub holder: Vec<TokenHolder>,
    }

    #[serde_as]
    #[derive(Debug, Deserialize)]
    pub struct TokenHolder {
        pub address: String,
        #[serde_as(as = "DisplayFromStr")]
        pub amount: u128,
    }

    ///
    /// URL path: `//krc20/address/{address}/token/{tick}`
    ///
    /// ```json
    /// {
    ///     "message": "text",
    ///     "result": [
    ///         {
    ///             "tick": "text",
    ///             "balance": "text",
    ///             "locked": "0",
    ///             "dec": "text",
    ///             "opScoreMod": "79993666"
    ///         }
    ///     ]
    /// }
    /// ```
    ///

    #[derive(Debug, Deserialize)]
    pub struct TokenBalanceResponse {
        pub message: String,
        pub result: Vec<TokenBalance>,
    }

    #[derive(Debug, Deserialize)]
    #[serde_as]
    pub struct TokenBalance {
        pub tick: String,

        #[serde_as(as = "DisplayFromStr")]
        pub balance: u128,

        #[serde_as(as = "DisplayFromStr")]
        pub locked: u64,

        /// TODO - what is dec?
        #[serde_as(as = "DisplayFromStr")]
        pub dec: u128,

        #[serde_as(as = "DisplayFromStr")]
        #[serde(rename = "opScoreMod")]
        pub op_score_mod: u64,
    }

    /// URLpath : `https://tn11api.kasplex.org/v1/krc20/address/{address}/tokenlist`
    ///
    /// ```json
    /// {
    ///     "message": "text",
    ///     "prev": "text",
    ///     "next": "text",
    ///     "result": [
    ///       {
    ///         "tick": "text",
    ///         "balance": "text",
    ///         "locked": "0",
    ///         "dec": "text",
    ///         "opScoreMod": "79993666"
    ///       }
    ///     ]
    ///   }
    /// ```

    #[derive(Debug, Deserialize)]
    pub struct TokenBalanceListByAddressResponse {
        pub message: String,
        pub next: String,
        pub prev: String,
        pub result: Vec<TokenBalance>,
    }

    ///
    /// URL path: `//krc20/oplist/{op}`
    ///
    /// ```json
    /// {
    ///     "message": "text",
    ///     "prev": "text",
    ///     "next": "text",
    ///     "result": [
    ///         {
    ///             "p": "KRC-20",
    ///             "op": "DEPLOY",
    ///             "tick": "KEKE",
    ///             "max": "2100000000000000",
    ///             "lim": "100000000000",
    ///             "dec": "8",
    ///             "amt": "2300000000",
    ///             "from": "kaspa:qra0p5ky...",
    ///             "to": "kaspa:qqabb6cz...",
    ///             "opScore": "779066550003",
    ///             "hashRev": "eb1482705b07af...",
    ///             "feeRev": "100010000",
    ///             "txAccept": "text",
    ///             "opAccept": "text",
    ///             "opError": "Insufficient fee",
    ///             "mtsAdd": "1712808987852",
    ///             "mtsMod": "1712808990016"
    ///         }
    ///     ]
    /// }
    /// ```

    #[derive(Debug, Deserialize)]
    pub struct TokenTransactionResponse {
        pub message: String,
        pub next: String,
        pub prev: String,
        pub result: Vec<TokenTransaction>,
    }

    #[serde_as]
    #[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
    pub struct TokenTransaction {
        #[serde_as(as = "DisplayFromStr")]
        #[serde(rename = "p")]
        pub protocol: Protocol,

        pub op: String,

        pub tick: String,

        #[serde_as(as = "Option<DisplayFromStr>")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub max: Option<u128>,

        #[serde_as(as = "Option<DisplayFromStr>")]
        #[serde(rename = "lim")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub limit: Option<u128>,

        #[serde_as(as = "Option<DisplayFromStr>")]
        #[serde(rename = "dec")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub dec: Option<u64>,

        #[serde_as(as = "Option<DisplayFromStr>")]
        #[serde(rename = "amt")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub amount: Option<u128>,

        #[serde_as(as = "Option<DisplayFromStr>")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub pre: Option<u128>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub from: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        pub to: Option<String>,

        #[serde_as(as = "Option<DisplayFromStr>")]
        #[serde(rename = "opScore")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub op_score: Option<u64>,

        #[serde(rename = "hashRev")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub hash_rev: Option<Hash>,

        #[serde(rename = "feeRev")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub fee_rev: Option<String>,

        #[serde(rename = "txAccept")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tx_accept: Option<String>,

        #[serde(rename = "opAccept")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub op_accept: Option<String>,

        #[serde(rename = "opError")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub op_error: Option<String>,

        #[serde(rename = "mtsAdd")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mts_add: Option<String>,

        #[serde(rename = "mtsMod")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mts_mod: Option<String>,
    }

    ///
    /// URL path: `//krc20/op/{id}`
    ///
    /// ```json
    /// {
    ///     "message": "text",
    ///     "result": [
    ///         {
    ///             "p": "KRC-20",
    ///             "op": "DEPLOY",
    ///             "tick": "KEKE",
    ///             "max": "2100000000000000",
    ///             "lim": "100000000000",
    ///             "dec": "8",
    ///             "amt": "2300000000",
    ///             "from": "kaspa:qra0p5ky...",
    ///             "to": "kaspa:qqabb6cz...",
    ///             "opScore": "779066550003",
    ///             "hashRev": "eb1482705b07af...",
    ///             "feeRev": "100010000",
    ///             "txAccept": "text",
    ///             "opAccept": "text",
    ///             "opError": "Insufficient fee",
    ///             "mtsAdd": "1712808987852",
    ///             "mtsMod": "1712808990016"
    ///         }
    ///     ]
    /// }
    /// ```
    ///

    #[derive(Deserialize)]
    pub struct TokenTransactionIdResponse {
        pub message: String,
        pub result: Vec<TokenTransaction>,
    }

    pub struct TokenTransactionBuilder {
        protocol: Protocol,
        op: String,
        tick: String,
        max: Option<u128>,
        limit: Option<u128>,
        pre: Option<u128>,
        dec: Option<u64>,
        amount: Option<u128>,
        from: Option<String>,
        to: Option<String>,
        op_score: Option<u64>,
        hash_rev: Option<Hash>,
        fee_rev: Option<String>,
        tx_accept: Option<String>,
        op_accept: Option<String>,
        op_error: Option<String>,
        mts_add: Option<String>,
        mts_mod: Option<String>,
    }

    impl TokenTransactionBuilder {
        pub fn new(protocol: Protocol, op: String, tick: String) -> Self {
            Self {
                protocol,
                op,
                tick,
                max: None,
                limit: None,
                pre: None,
                dec: None,
                amount: None,
                from: None,
                to: None,
                op_score: None,
                hash_rev: None,
                fee_rev: None,
                tx_accept: None,
                op_accept: None,
                op_error: None,
                mts_add: None,
                mts_mod: None,
            }
        }

        pub fn max(mut self, max: u128) -> Self {
            self.max = Some(max);
            self
        }

        pub fn amount(mut self, amount: u128) -> Self {
            self.amount = Some(amount);
            self
        }

        pub fn limit(mut self, limit: u128) -> Self {
            self.limit = Some(limit);
            self
        }

        pub fn dec(mut self, dec: u64) -> Self {
            self.dec = Some(dec);
            self
        }

        pub fn build(self) -> TokenTransaction {
            TokenTransaction {
                protocol: self.protocol,
                op: self.op,
                tick: self.tick,
                max: self.max,
                limit: self.limit,
                pre: self.pre,
                dec: self.dec,
                amount: self.amount,
                from: self.from,
                to: self.to,
                op_score: self.op_score,
                hash_rev: self.hash_rev,
                fee_rev: self.fee_rev,
                tx_accept: self.tx_accept,
                op_accept: self.op_accept,
                op_error: self.op_error,
                mts_add: self.mts_add,
                mts_mod: self.mts_mod,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use serde_json;

        // Struct to hold test case data
        struct TestCase {
            json_str: &'static str,
            expected: std::result::Result<TokenTransaction, serde_json::Error>,
        }

        // List of test cases
        fn get_test_cases() -> Vec<TestCase> {
            vec![TestCase {
                json_str: r#"
                    {
                        "p": "KRC-20",
                        "op": "transfer",
                        "tick": "SPARKL",
                        "amt": "500"
                    }
                    "#,
                expected: Ok(TokenTransactionBuilder::new(
                    Protocol::Krc20,
                    "transfer".to_string(),
                    "SPARKL".to_string(),
                )
                .amount(500)
                .build()),
            }]
        }

        // Generic test function
        fn run_test_case(test_case: TestCase) {
            let deserialized: std::result::Result<TokenTransaction, serde_json::Error> =
                serde_json::from_str(test_case.json_str);
            match (deserialized, test_case.expected) {
                (Ok(result), Ok(expected)) => assert_eq!(result, expected),
                (Err(_), Err(_)) => {} // Both are errors, which is expected
                (Ok(_), Err(_)) | (Err(_), Ok(_)) => {
                    panic!("Test case failed: mismatched result and expected")
                }
            }
        }

        #[test]
        fn test_token_transaction_serialization() {
            let test_cases = get_test_cases();
            for test_case in test_cases {
                run_test_case(test_case);
            }
        }
    }
}
