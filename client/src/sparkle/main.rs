pub const GIT_DESCRIBE: &str = env!("VERGEN_GIT_DESCRIBE");
pub const GIT_SHA: &str = env!("VERGEN_GIT_SHA");
pub const RUSTC_CHANNEL: &str = env!("VERGEN_RUSTC_CHANNEL");
pub const RUSTC_COMMIT_DATE: &str = env!("VERGEN_RUSTC_COMMIT_DATE");
pub const RUSTC_COMMIT_HASH: &str = env!("VERGEN_RUSTC_COMMIT_HASH");
pub const RUSTC_HOST_TRIPLE: &str = env!("VERGEN_RUSTC_HOST_TRIPLE");
pub const RUSTC_LLVM_VERSION: &str = env!("VERGEN_RUSTC_LLVM_VERSION");
pub const RUSTC_SEMVER: &str = env!("VERGEN_RUSTC_SEMVER");
pub const CARGO_TARGET_TRIPLE: &str = env!("VERGEN_CARGO_TARGET_TRIPLE");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        fn main() {}
    } else {
        pub mod args;
        pub mod client;
        pub mod wallet;
        pub mod log;

        use client::Client;
        use workflow_log::prelude::*;
        use sparkle_core::runtime::{Runtime, Signals};
        use sparkle_rs::error::Error;

        #[tokio::main]
        async fn main() {
            let runtime = Runtime::default();
            Signals::bind(&runtime);

            match Client.run(&runtime).await {
                Ok(_) => log_info!(""),
                Err(Error::UserAbort) => {
                    println!();
                },
                Err(err) => { log_error!("Error: {err}") },
            }
        }

    }
}
