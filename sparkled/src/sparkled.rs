use sparkle_core::runtime::Runtime;
use sparkle_http_server::HttpServer;
use sparkle_nexus::prelude::{Analyzer, Nexus};
use sparkle_rpc_server::{WrpcOptions, WrpcService};
use std::sync::Arc;
#[allow(unused_imports)]
use workflow_core::dirs::home_dir;

use crate::args::Args;
use crate::result::Result;

#[derive(Default)]
pub struct Server {}

impl Server {
    pub async fn run(&self, runtime: &Runtime) -> Result<()> {
        let Args {
            trace_log_level,
            enable_debug_mode,
            network_id,
            enable_http_server,
            http_listen,
            rpc_listen,
            node_rpc,
        } = Args::parse();

        if trace_log_level {
            workflow_log::set_log_level(workflow_log::LevelFilter::Trace);
        }

        sparkle_core::debug::enable(enable_debug_mode);

        // --- Services ---

        let nexus = Nexus::try_new(network_id, node_rpc.as_deref())
            .await
            .expect("Unable to create nexus instance.");
        runtime.bind(Arc::new(nexus.clone()));

        let analyzer = Analyzer::try_new(&nexus)
            .await
            .expect("Unable to create analyzer instance.");
        runtime.bind(Arc::new(analyzer));

        let wrpc_options = WrpcOptions::default().listen(rpc_listen.to_string().as_str());
        let wrpc_server = WrpcService::try_new(&nexus, wrpc_options)
            .await
            .expect("Unable to create wRPC service.");
        runtime.bind(Arc::new(wrpc_server));

        if enable_http_server {
            let http_server = HttpServer::try_new(&nexus, http_listen.to_string().as_str(), None)
                .await
                .expect("Unable to create HTTP server.");
            runtime.bind(Arc::new(http_server));
        }

        runtime.run().await?;

        Ok(())
    }
}
