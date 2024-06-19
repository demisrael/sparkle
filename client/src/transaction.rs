use crate::imports::*;
use kaspa_txscript_errors::TxScriptError;
#[cfg(not(target_arch = "wasm32"))]
use sparkle_nexus::analyzer::detect_krc20;

use kaspa_consensus_core::hashing::sighash::SigHashReusedValues;
use kaspa_txscript::caches::Cache;
use kaspa_txscript::SigCacheKey;
use kaspa_txscript::TxScriptEngine;

use kaspa_consensus_core::sign::sign;
use kaspa_consensus_core::tx::{
    MutableTransaction, ScriptPublicKey, Transaction, TransactionId, TransactionInput,
    TransactionOutpoint, TransactionOutput, UtxoEntry,
};
use kaspa_txscript::opcodes::codes::*;
use kaspa_txscript::script_builder::{ScriptBuilder, ScriptBuilderResult};
//
use std::vec;

use kaspa_consensus_core::{subnets::SubnetworkId, tx::*};
use secp256k1::{rand, Secp256k1};
use std::str::FromStr;

use kaspa_txscript::{pay_to_script_hash_script, pay_to_script_hash_signature_script};

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

// Same function as test function for consuming deps usage.
#[cfg(not(target_arch = "wasm32"))]
fn showcase() {
    use sparkle_nexus::operations::build_deploy_json_example;

    let secp = Secp256k1::new();
    let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
    let test_pubkey = ScriptVec::from_slice(&public_key.serialize());

    let script_sig: Vec<u8> = redeem_pubkey(
        build_deploy_json_example().as_bytes(),
        &public_key.serialize()[1..33],
    )
    .unwrap();

    let redeem_lock_p2sh = pay_to_script_hash_script(&script_sig);

    let prev_tx_id =
        TransactionId::from_str("770eb9819a31821d9d2399e2f35e2433b72637e393d71ecc9b8d0250f49153c3")
            .unwrap();
    let mut unsigned_tx = Transaction::new(
        0,
        vec![TransactionInput {
            previous_outpoint: TransactionOutpoint {
                transaction_id: prev_tx_id,
                index: 0,
            },
            signature_script: vec![],
            sequence: 0,
            sig_op_count: 1, // when signed it turns to 1
        }],
        vec![
            // Send to same address as origin only in this test.
            TransactionOutput {
                value: 115,
                script_public_key: ScriptPublicKey::new(0, test_pubkey.clone()),
            },
        ],
        1615462089000,
        SubnetworkId::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
        0,
        vec![],
    );

    let entries = vec![UtxoEntry {
        amount: 115,
        script_public_key: redeem_lock_p2sh.clone(),
        block_daa_score: 0,
        is_coinbase: false,
    }];

    // Signing the transaction with keypair.
    let tx_clone = unsigned_tx.clone();
    let entries_clone = entries.clone();
    let schnorr_key =
        secp256k1::Keypair::from_seckey_slice(secp256k1::SECP256K1, &secret_key.secret_bytes())
            .unwrap();
    let mut signed_tx = sign(
        MutableTransaction::with_entries(tx_clone, entries_clone),
        schnorr_key,
    );
    let _signature = signed_tx.tx.inputs[0].signature_script.clone();

    // Prepend the signature to the unlock script.
    let script_sig = pay_to_script_hash_signature_script(script_sig.clone(), _signature).unwrap();
    unsigned_tx.inputs[0]
        .signature_script
        .clone_from(&script_sig);
    signed_tx.tx.inputs[0].signature_script = script_sig;

    let tx = MutableTransaction::with_entries(unsigned_tx, entries);
    use kaspa_consensus_core::tx::VerifiableTransaction;
    let tx = tx.as_verifiable();
    let cache: Cache<SigCacheKey, bool> = Cache::new(10_000);
    let mut reused_values = SigHashReusedValues::new();

    let script_run: Result<(), TxScriptError> =
        tx.populated_inputs()
            .enumerate()
            .try_for_each(|(idx, (input, entry))| {
                TxScriptEngine::from_transaction_input(
                    &tx,
                    input,
                    idx,
                    entry,
                    &mut reused_values,
                    &cache,
                )?
                .execute()
            });

    assert!(script_run.is_ok());
    let found_inscription = detect_krc20(tx.tx());
    assert!(found_inscription.is_some());
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_and_verify_sign() {
        use sparkle_nexus::operations::build_deploy_json_example;

        let secp = Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
        let test_pubkey = ScriptVec::from_slice(&public_key.serialize());

        let script_sig: Vec<u8> = redeem_pubkey(
            build_deploy_json_example().as_bytes(),
            &public_key.serialize()[1..33],
        )
        .unwrap();

        let redeem_lock_p2sh = pay_to_script_hash_script(&script_sig);

        let prev_tx_id = TransactionId::from_str(
            "770eb9819a31821d9d2399e2f35e2433b72637e393d71ecc9b8d0250f49153c3",
        )
        .unwrap();
        let mut unsigned_tx = Transaction::new(
            0,
            vec![TransactionInput {
                previous_outpoint: TransactionOutpoint {
                    transaction_id: prev_tx_id,
                    index: 0,
                },
                signature_script: vec![],
                sequence: 0,
                sig_op_count: 1, // when signed it turns to 1
            }],
            vec![
                // Send to same address as origin only in this test.
                TransactionOutput {
                    value: 115,
                    script_public_key: ScriptPublicKey::new(0, test_pubkey.clone()),
                },
            ],
            1615462089000,
            SubnetworkId::from_bytes([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            0,
            vec![],
        );

        let entries = vec![UtxoEntry {
            amount: 115,
            script_public_key: redeem_lock_p2sh.clone(),
            block_daa_score: 0,
            is_coinbase: false,
        }];

        // Signing the transaction with keypair.
        let tx_clone = unsigned_tx.clone();
        let entries_clone = entries.clone();
        let schnorr_key =
            secp256k1::Keypair::from_seckey_slice(secp256k1::SECP256K1, &secret_key.secret_bytes())
                .unwrap();
        let mut signed_tx = sign(
            MutableTransaction::with_entries(tx_clone, entries_clone),
            schnorr_key,
        );
        let _signature = signed_tx.tx.inputs[0].signature_script.clone();

        // Prepend the signature to the unlock script.
        let script_sig =
            pay_to_script_hash_signature_script(script_sig.clone(), _signature).unwrap();
        unsigned_tx.inputs[0]
            .signature_script
            .clone_from(&script_sig);
        signed_tx.tx.inputs[0].signature_script = script_sig;

        let tx = MutableTransaction::with_entries(unsigned_tx, entries);
        use kaspa_consensus_core::tx::VerifiableTransaction;
        let tx = tx.as_verifiable();
        let cache: Cache<SigCacheKey, bool> = Cache::new(10_000);
        let mut reused_values = SigHashReusedValues::new();

        let script_run: Result<(), TxScriptError> =
            tx.populated_inputs()
                .enumerate()
                .try_for_each(|(idx, (input, entry))| {
                    TxScriptEngine::from_transaction_input(
                        &tx,
                        input,
                        idx,
                        entry,
                        &mut reused_values,
                        &cache,
                    )?
                    .execute()
                });

        assert!(script_run.is_ok());
        let found_inscription = detect_krc20(tx.tx());
        assert!(found_inscription.is_some());
    }
}
