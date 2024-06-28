use crate::constants::*;
use crate::model::kasplex::v1::krc20::TokenTransaction;
use crate::model::kasplex::v1::Protocol;
use kaspa_addresses::Address;
use kaspa_txscript::opcodes::codes::*;
use kaspa_txscript::script_builder::{ScriptBuilder, ScriptBuilderResult};

use std::str::FromStr;

use kaspa_txscript::{extract_script_pub_key_address, pay_to_script_hash_script};

fn redeem_pubkey(redeem_script: &[u8], pubkey: &[u8]) -> ScriptBuilderResult<Vec<u8>> {
    Ok(ScriptBuilder::new()
        .add_data(pubkey)?
        .add_op(OpCheckSig)?
        .add_op(OpFalse)?
        .add_op(OpIf)?
        .add_data(PROTOCOL_NAMESPACE.as_bytes())?
        .add_data(&[0])?
        .add_data(redeem_script)?
        .add_op(OpEndIf)?
        .drain())
}

pub fn deploy_demo(recipient: Address) -> Address {
    let transaction: TokenTransaction = TokenTransaction {
        protocol: Protocol::from_str("krc-20").unwrap(),
        op: "deploy".to_string(),
        tick: "TTTT".to_string(),
        max: Some(21000),
        limit: None,
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
    };

    let json = serde_json::to_string(&transaction).unwrap();
    let script_sig: Vec<u8> = redeem_pubkey(json.as_bytes(), &recipient.payload[1..32]).unwrap();
    let redeem_lock_p2sh = pay_to_script_hash_script(&script_sig);

    extract_script_pub_key_address(&redeem_lock_p2sh, "kaspatest".try_into().unwrap()).unwrap()
}

pub fn deploy_script_sig(recipient: Address) -> Vec<u8> {
    let transaction: TokenTransaction = TokenTransaction {
        protocol: Protocol::from_str("krc-20").unwrap(),
        op: "deploy".to_string(),
        tick: "TTTT".to_string(),
        max: Some(21000),
        limit: None,
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
    };
    let json = serde_json::to_string(&transaction).unwrap();
    let script_sig: Vec<u8> = redeem_pubkey(json.as_bytes(), &recipient.payload[1..32]).unwrap();
    script_sig
}
