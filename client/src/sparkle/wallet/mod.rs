use cliclack::log;
use kaspa_wallet_core::prelude::{
    AccountDescriptor, AccountId, ConnectRequest, Wallet as CoreWallet, WalletDescriptor,
};
use pad::{Alignment, PadStr};
use sparkle_rs::imports::*;
use sparkle_rs::result::Result;

mod account;
use account::Account;

type AccountHashMap = HashMap<AccountId, Arc<AccountDescriptor>>;

pub struct Context {
    pub network_id: NetworkId,
    pub node_url: Option<String>,
    pub wallet_file: Option<String>,
}

pub struct Wallet {
    pub wallet: Arc<CoreWallet>,
    pub account: Option<Arc<AccountDescriptor>>,
}

impl Wallet {
    pub async fn try_new(context: Context, connect: bool) -> Result<Self> {
        let Context {
            network_id,
            node_url,
            wallet_file,
        } = context;

        let wallet = CoreWallet::default()
            .with_resolver(Default::default())
            .with_url(node_url.as_deref())
            .with_network_id(network_id)
            .to_arc();

        // check if user-supplied wallet exists
        if let Some(wallet_file) = wallet_file.as_ref() {
            if !wallet.exists(Some(wallet_file.as_str())).await? {
                return Err(Error::custom(format!(
                    "Wallet not found: `{}`",
                    wallet_file
                )));
            }
        }

        wallet.start().await?;

        if connect {
            let request = ConnectRequest::default()
                .with_network_id(&network_id)
                .with_url(node_url)
                .with_retry_on_error(false)
                .with_require_sync(false);

            wallet.as_api().connect_call(request).await?;

            log::success(format!(
                "Connected to `{network_id}` at `{}`",
                wallet.wrpc_client().url().unwrap_or_default()
            ))?;
        }

        let wallet_file = if let Some(wallet_file) = wallet_file {
            wallet_file
        } else {
            let wallet_descriptors = wallet.as_api().wallet_enumerate().await?;

            if wallet_descriptors.is_empty() {
                return Err(Error::custom(
                    "No wallets found, please use `kaspa-wallet` to create accounts",
                ));
            } else if wallet_descriptors.len() == 1 {
                wallet_descriptors.first().unwrap().filename.clone()
            } else {
                let mut selector = cliclack::select(format!("Please select a wallet:"));
                for WalletDescriptor { filename, title } in wallet_descriptors {
                    selector = selector.item(filename.clone(), title.as_deref().unwrap_or(""), "");
                }
                selector.interact().map_err(|_| Error::UserAbort)?
            }
        };

        let password = cliclack::password("Enter wallet password:")
            .mask('▪')
            .interact()
            .map_err(|_| Error::UserAbort)?;

        let spinner = cliclack::spinner();
        spinner.start("Loading wallet...");

        // let accounts =
        wallet
            .as_api()
            .wallet_open(password.into(), Some(wallet_file), true, true)
            .await?
            .unwrap();

        wallet.as_api().accounts_activate(None).await?;

        let accounts = wallet
            .as_api()
            .accounts_enumerate()
            .await?
            .into_iter()
            .map(|descriptor| Account::new(descriptor, &network_id))
            .collect::<Vec<_>>();

        if accounts.is_empty() {
            return Err(Error::custom(
                "Wallet has no accounts, please use `kaspa-wallet` to create an account",
            ));
        }

        let name_len = accounts.iter().fold(0, |acc, account| {
            // let AccountDescriptor { account_name, .. } = account.as_ref();
            account
                .descriptor
                .account_name
                .as_ref()
                .map(|name| name.len())
                .unwrap_or(0)
                .max(acc)
        });

        let balance_len = accounts.iter().fold((0, 0, 0), |acc, account| {
            let (a, b, c) = account
                .balance
                .as_ref()
                .map(|v| v.len())
                .unwrap_or((0, 0, 0));
            (a.max(acc.0), b.max(acc.1), c.max(acc.2))
        });

        // let balances = accounts.iter().map(|account|account)
        let account_map = AccountHashMap::from_iter(accounts.iter().map(|account| {
            (
                account.descriptor.account_id.clone(),
                account.descriptor.clone(),
            )
        }));

        spinner.stop("Loading wallet...");

        let account_id = if accounts.len() == 1 {
            accounts.first().unwrap().descriptor.account_id.clone()
        } else {
            let mut selector = cliclack::select("Please select an account:");
            for account in accounts {
                let Account {
                    descriptor,
                    short_id,
                    balance,
                    ..
                } = account;

                let descr = [
                    short_id.pad_to_width_with_alignment(9, Alignment::Left),
                    descriptor
                        .account_name
                        .clone()
                        .unwrap_or_else(|| "".to_string())
                        .pad_to_width(name_len),
                    balance
                        .map(|balance| balance.format(balance_len))
                        .unwrap_or(" - ".to_string()),
                ]
                .join("");

                selector = selector.item(descriptor.account_id.clone(), descr, "");
            }
            selector.interact().map_err(|_| Error::UserAbort)?
        };

        let account = account_map.get(&account_id).cloned(); //.unwrap().cloned();

        Ok(Self { wallet, account })
    }
}
