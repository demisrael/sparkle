#[cfg(test)]
mod test {

    // cargo test test_evm -- --nocapture

    use evm::backend::{MemoryBackend, MemoryVicinity};
    use evm::executor::stack::*;
    #[allow(unused_imports)]
    use evm::{Config, ExitError, ExitReason, ExitSucceed};
    use hex_literal::hex;
    use primitive_types::{H160, U256};
    use std::collections::BTreeMap;
    use std::fs;
    use std::str::FromStr;

    #[test]
    fn test_evm() {
        let vicinity = MemoryVicinity {
            gas_price: U256::from(1), //Default::default(),
            origin: Default::default(),
            block_hashes: vec![],
            block_number: Default::default(),
            block_coinbase: Default::default(),
            block_timestamp: Default::default(),
            block_difficulty: Default::default(),
            block_gas_limit: U256::from(1_000_000_000),
            chain_id: U256::from(1),
            block_base_fee_per_gas: U256::from(1),
            // block_base_fee_per_gas: Default::default(),
            block_randomness: Default::default(),
        };

        // Initialize state with sufficient balance
        let mut state = BTreeMap::new();
        state.insert(
            // H160::zero(),
            H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
            evm::backend::MemoryAccount {
                nonce: U256::one(),
                balance: U256::from(10_000_000_000_000_000u64), // Sufficient balance
                storage: BTreeMap::new(),
                code: vec![],
            },
        );

        state.insert(
            // H160::zero(),
            H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
            evm::backend::MemoryAccount {
                nonce: U256::one(),
                balance: U256::from(10_000_000_000_000_000u64), // Sufficient balance
                storage: BTreeMap::new(),
                code: vec![],
            },
        );

        let mut backend = MemoryBackend::new(&vicinity, state);

        // let mut backend = MemoryBackend::new(&vicinity, Default::default());
        let config = Config::london();

        let metadata = StackSubstateMetadata::new(100_000_000, &config);
        let state = MemoryStackState::new(metadata, &mut backend);

        let precompiles = BTreeMap::new();
        let mut executor = StackExecutor::new_with_precompiles(state, &config, &precompiles);
        // let mut executor = StackExecutor::new_with_precompiles(state, &config, &());

        // Load contract bytecode
        let file_path = "misc/build/SimpleStorage.bin";

        let code = match fs::read_to_string(file_path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!("Error reading file {}: {}", file_path, err);
                return; // Exit the function or handle the error as needed
            }
        };

        let code = hex::decode(code).unwrap();

        // Deploy the contract
        println!("deploying...");
        // let address = executor.create_address(evm::CreateScheme::Legacy { caller: Default::default() });
        let (reason, _data) = executor.transact_create(
            // Default::default(),
            // H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
            // Default::default(),
            H160::zero(),
            U256::zero(),
            code,
            // 10_000_000,
            u64::MAX,
            vec![],
        );
        // println!("transact_create: {_data:?}");
        println!("transact_create - reason: {reason:?} data: {_data:?}");

        assert_eq!(reason, ExitReason::Succeed(ExitSucceed::Returned));

        // Interact with the contract
        let input_data = hex!(
            "60fe47b1"
            "000000000000000000000000000000000000000000000000000000000000002a"
        ); // set(42)
        let (reason, _data) = executor.transact_call(
            // Default::default(),
            H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
            H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
            // address,
            Default::default(),
            input_data.to_vec(),
            100_000_000,
            vec![],
        );
        println!("transact_call - reason: {reason:?} data: {_data:?}");

        // assert_eq!(reason, ExitReason::Succeed(ExitSucceed::Returned));

        let input_data = hex!("6d4ce63c"); // get()
        let (reason, data) = executor.transact_call(
            // Default::default(),
            H160::from_str("0xf000000000000000000000000000000000000000").unwrap(),
            H160::from_str("0x1000000000000000000000000000000000000000").unwrap(),
            // address,
            Default::default(),
            input_data.to_vec(),
            100_000_000,
            vec![],
        );
        println!("transact_call - reason: {reason:?} data: {data:?}");
        // println!("transact_call: {data:?}");

        // assert_eq!(reason, ExitReason::Succeed(ExitSucceed::Returned));
        assert_eq!(
            data,
            hex!("000000000000000000000000000000000000000000000000000000000000002a")
        );
    }
}
