use crate::args::{Action, Args, WalletAction};
use kaspa_wallet_core::prelude::*;
use rpassword::read_password;
use sparkle_core::runtime::Runtime;
use sparkle_rpc_client::prelude::*;
use sparkle_rs::imports::*;
use sparkle_rs::result::Result;
use std::io::Write; // for std::io::stdout().flush()
                    // use kaspa_wrpc_client::Resolver;

#[derive(Default)]
pub struct Client;

impl Client {
    pub async fn run(&self, _runtime: &Runtime) -> Result<()> {
        let Args {
            action,
            sparkled_url,
            network_id,
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

                // ensure that sparkled network matches ours
                let _status = client.negotiate(&network_id).await?;

                println!("📡 Pinging...");
                client.ping().await?;
                println!("🥂 Ok...");
                client.disconnect().await?;
            }
            Action::Wallet { action } => {
                match action {
                    WalletAction::List => {
                        let wallet = Self::wallet(None, network_id);

                        let wallet_descriptors = wallet.as_api().wallet_enumerate().await?;
                        println!();
                        for WalletDescriptor { filename, title } in wallet_descriptors {
                            println!("{:>12} {:?}", filename, title);
                        }
                        println!();
                    }
                    WalletAction::Accounts { wallet_file } => {
                        let wallet = Self::wallet(None, network_id);

                        let wallet_file = wallet_file.or_else(|| Some("default".to_string()));
                        if !wallet.exists(wallet_file.as_deref()).await? {
                            println!("Wallet not found: '{}'", wallet_file.unwrap());
                            return Ok(());
                        }

                        let accounts = wallet
                            .as_api()
                            .wallet_open(Self::wallet_secret(), wallet_file, true, false)
                            .await?
                            .unwrap();
                        // println!("Accounts: {:?}", descriptors);
                        println!();
                        for account in accounts {
                            let AccountDescriptor {
                                kind,
                                account_id,
                                account_name,
                                ..
                            } = account;
                            let account_name = account_name.unwrap_or_else(|| "-".to_string());
                            println!("{} {} - {}", account_id.short(), kind, account_name);
                        }
                        println!();
                    }
                    _ => {
                        println!("Action not implemented yet");
                    }
                }
            }
        }

        Ok(())
    }

    pub fn wallet(url: Option<&str>, network_id: NetworkId) -> Arc<Wallet> {
        Wallet::default()
            .with_resolver(Default::default())
            .with_url(url)
            .with_network_id(network_id)
            .to_arc()
    }

    pub fn wallet_secret() -> Secret {
        print!("Enter wallet password: ");
        std::io::stdout().flush().unwrap();
        Secret::from(read_password().unwrap())
    }
}
