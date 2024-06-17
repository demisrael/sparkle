use crate::args::{Action, Args};
use sparkle_core::runtime::Runtime;
use sparkle_rpc_client::prelude::*;
use sparkle_rs::result::Result;

#[derive(Default)]
pub struct Client;

impl Client {
    pub async fn run(&self, _runtime: &Runtime) -> Result<()> {
        let Args {
            action,
            sparkled_url,
            network_id: _,
            enable_debug_mode,
            trace_log_level,
        } = Args::parse();

        if trace_log_level {
            workflow_log::set_log_level(workflow_log::LevelFilter::Trace);
        }

        sparkle_core::debug::enable(enable_debug_mode);

        // ---

        println!(
            "sparkle client v{}-{} (rusty-kaspa v{})",
            crate::VERSION,
            crate::GIT_DESCRIBE,
            kaspa_wallet_core::version()
        );

        let url = sparkled_url.unwrap_or_else(|| "ws://127.0.0.1:7878".to_string());

        match action {
            Action::Ping => {
                let client = SparkleRpcClient::try_new(url.as_str(), None)?;
                client.connect(None).await?;
                println!(
                    "Connected to {}",
                    client.url().unwrap_or_else(|| "🤷".to_string())
                );
                println!("📡 Pinging...");
                client.ping().await?;
                println!("🥂 Ok...");
                client.disconnect().await?;
            }
        }

        Ok(())
    }
}
