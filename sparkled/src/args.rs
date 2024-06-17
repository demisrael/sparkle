use kaspa_consensus_core::network::{NetworkId, NetworkType};
use kaspa_utils::networking::ContextualNetAddress;
use kaspa_wrpc_client::WrpcEncoding;

#[derive(Debug)]
pub struct Args {
    pub trace_log_level: bool,
    pub enable_debug_mode: bool,
    pub enable_http_server: bool,
    pub http_listen: ContextualNetAddress,
    pub rpc_listen: ContextualNetAddress,
    pub network_id: NetworkId,
    pub node_rpc: Option<String>,
}

impl Args {
    pub fn parse() -> Args {
        #[allow(unused)]
        use clap::{arg, command, Arg, Command};

        let cmd = Command::new("sparkled")
            .about(format!(
                "sparkle node v{}-{} (rusty-kaspa v{})",
                crate::VERSION,
                crate::GIT_DESCRIBE,
                kaspa_wallet_core::version()
            ))
            .arg(arg!(--version "Display software version"))
            .arg(arg!(--trace "Enable trace log level"))
            .arg(arg!(--debug "Enable debug mode"))
            .arg(arg!(--http "Enable HTTP Server"))
            .arg(
                Arg::new("rpc-listen")
                    .long("rpc-listen")
                    .value_name("ip[:port]")
                    .num_args(0..=1)
                    .require_equals(true)
                    .value_parser(clap::value_parser!(ContextualNetAddress))
                    .help(
                        "Interface:port to listen for wRPC connections (default: 127.0.0.1:7878).",
                    ),
            )
            .arg(
                Arg::new("http-listen")
                    .long("http-listen")
                    .value_name("ip[:port]")
                    .num_args(0..=1)
                    .require_equals(true)
                    .value_parser(clap::value_parser!(ContextualNetAddress))
                    .help(
                        "Interface:port to listen for HTTP connections (default 127.0.0.1:7676).",
                    ),
            )
            .arg(
                Arg::new("network")
                    .long("network")
                    .value_name("mainnet | testnet-10 | testnet-11")
                    .num_args(0..=1)
                    .require_equals(true)
                    .value_parser(clap::value_parser!(NetworkId))
                    .help("Network id."),
            )
            .arg(
                Arg::new("node-rpc")
                    .long("node-rpc")
                    .value_name("ws://address[:port] or wss://address[:port]")
                    .num_args(0..=1)
                    .require_equals(true)
                    .help("wRPC URL of the node (disables resolver)."),
            );

        let matches = cmd.get_matches();

        let trace_log_level = matches.get_one::<bool>("trace").cloned().unwrap_or(false);

        let enable_debug_mode = matches.get_one::<bool>("debug").cloned().unwrap_or(false);

        let enable_http_server = matches.get_one::<bool>("http").cloned().unwrap_or(false);

        let http_listen = matches
            .get_one::<ContextualNetAddress>("http-listen")
            .cloned()
            .unwrap_or("127.0.0.1:7676".parse().unwrap());

        let rpc_listen = matches
            .get_one::<ContextualNetAddress>("rpc-listen")
            .cloned()
            .unwrap_or("127.0.0.1:7878".parse().unwrap());

        let network_id = matches
            .get_one::<NetworkId>("network")
            .cloned()
            .unwrap_or(NetworkId::new(NetworkType::Mainnet));

        let node_rpc = matches.get_one::<String>("node-rpc").cloned();

        if let Some(node_url) = &node_rpc {
            if let Err(err) = kaspa_wrpc_client::KaspaRpcClient::parse_url(
                node_url.to_string(),
                WrpcEncoding::Borsh,
                network_id.into(),
            ) {
                eprintln!("Invalid node-rpc URL: {}", err);
                std::process::exit(1);
            }
        }

        if matches.get_one::<bool>("version").cloned().unwrap_or(false) {
            println!("v{}-{}", crate::VERSION, crate::GIT_DESCRIBE);
            std::process::exit(0);
        } else {
            Args {
                trace_log_level,
                enable_debug_mode,
                enable_http_server,
                http_listen,
                rpc_listen,
                network_id,
                node_rpc,
            }
        }
    }
}
