#![allow(unused_imports)]

#[cfg(all(
    any(
        feature = "wasm32-sdk",
        feature = "wasm32-rpc",
        feature = "wasm32-core",
        feature = "wasm32-keygen"
    ),
    not(target_arch = "wasm32")
))]
compile_error!("`sparkle-wasm` crate for WASM32 target must be built with `--features wasm32-sdk|wasm32-rpc|wasm32-core|wasm32-keygen`");

mod version;
pub use version::*;

cfg_if::cfg_if! {

    if #[cfg(feature = "wasm32-sdk")] {

        pub use kaspa_addresses::{Address, Version as AddressVersion};
        pub use kaspa_consensus_core::tx::{ScriptPublicKey, Transaction, TransactionInput, TransactionOutpoint, TransactionOutput};
        // pub use kaspa_pow::wasm::*;

        pub mod rpc {
            //! Kaspa RPC interface
            //!

            pub mod messages {
                //! Kaspa RPC messages
                pub use kaspa_rpc_core::model::message::*;
            }
            pub use kaspa_rpc_core::api::rpc::RpcApi;
            pub use kaspa_rpc_core::wasm::message::*;

            pub use kaspa_wrpc_wasm::client::*;
            pub use kaspa_wrpc_wasm::resolver::*;
            pub use kaspa_wrpc_wasm::notify::*;
        }

        pub use kaspa_consensus_wasm::*;
        pub use kaspa_wallet_keys::prelude::*;
        pub use kaspa_wallet_core::wasm::*;

    } else if #[cfg(feature = "wasm32-core")] {

        pub use kaspa_addresses::{Address, Version as AddressVersion};
        pub use kaspa_consensus_core::tx::{ScriptPublicKey, Transaction, TransactionInput, TransactionOutpoint, TransactionOutput};
        pub use kaspa_pow::wasm::*;

        pub mod rpc {
            //! Kaspa RPC interface
            //!

            pub mod messages {
                //! Kaspa RPC messages
                pub use kaspa_rpc_core::model::message::*;
            }
            pub use kaspa_rpc_core::api::rpc::RpcApi;
            pub use kaspa_rpc_core::wasm::message::*;

            pub use kaspa_wrpc_wasm::client::*;
            pub use kaspa_wrpc_wasm::resolver::*;
            pub use kaspa_wrpc_wasm::notify::*;
        }

        pub use kaspa_consensus_wasm::*;
        pub use kaspa_wallet_keys::prelude::*;
        pub use kaspa_wallet_core::wasm::*;

    } else if #[cfg(feature = "wasm32-rpc")] {

        pub use kaspa_rpc_core::api::rpc::RpcApi;
        pub use kaspa_rpc_core::wasm::message::*;
        pub use kaspa_rpc_core::wasm::message::IPingRequest;
        pub use kaspa_wrpc_wasm::client::*;
        pub use kaspa_wrpc_wasm::resolver::*;
        pub use kaspa_wrpc_wasm::notify::*;
        pub use kaspa_wasm_core::types::*;

    } else if #[cfg(feature = "wasm32-keygen")] {

        pub use kaspa_addresses::{Address, Version as AddressVersion};
        pub use kaspa_wallet_keys::prelude::*;
        pub use kaspa_wasm_core::types::*;

    }
}
