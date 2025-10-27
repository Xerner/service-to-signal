use http::Uri;
use log::info;
use std::sync::Arc;
use up_rust::{
    communication::{InMemoryRpcServer, RpcServer},
    LocalUriProvider, StaticUriProvider,
};
use up_transport_zenoh::{zenoh_config, UPTransportZenoh};

use horn_common::constants::{
    ACTIVATE_HORN_RESOURCE_ID, DEACTIVATE_HORN_RESOURCE_ID, HORN_SERVICE_ENTITY_ID,
    HORN_SERVICE_MAJOR_VERSION,
};

use crate::{config::Args, connections, request_handler};

pub struct HornServer {
    pub kuksa_enabled: bool,
    pub kuksa_address: Uri,
    rpc_server: InMemoryRpcServer,
    tx_kuksa: tokio::sync::mpsc::Sender<bool>,
}

impl HornServer {
    pub async fn new(
        kuksa_enabled: bool,
        kuksa_address: Uri,
        zenoh_config: zenoh_config::Config,
        horn_service_authority_name: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        UPTransportZenoh::try_init_log_from_env();
        let (tx_kuksa_horn_is_active, rx_kuksa_horn_is_active) = tokio::sync::mpsc::channel(32);
        let (tx_horn_requests, rx_horn_requests) = tokio::sync::mpsc::channel(4);

        let uri_provider = Arc::new(StaticUriProvider::new(
            horn_service_authority_name,
            HORN_SERVICE_ENTITY_ID,
            HORN_SERVICE_MAJOR_VERSION,
        ));

        let rpc_server = HornServer::create_rpc_server(uri_provider, zenoh_config).await?;
        tokio::spawn(self.receive(rx_horn_requests, tx_kuksa_horn_is_active));

        let activate_horn_op =
            Arc::new(request_handler::ActivateHorn::new(tx_horn_requests.clone()));
        rpc_server
            .register_endpoint(None, ACTIVATE_HORN_RESOURCE_ID, activate_horn_op)
            .await?;

        let deactivate_horn_op = Arc::new(request_handler::DeactivateHorn::new(
            tx_horn_requests.clone(),
        ));
        rpc_server
            .register_endpoint(None, DEACTIVATE_HORN_RESOURCE_ID, deactivate_horn_op)
            .await?;

        let horn_server = HornServer {
            kuksa_enabled,
            kuksa_address,
            rpc_server,
        };
        horn_server.initialize_connections(rx_kuksa_horn_is_active);

        Ok(horn_server)
    }

    async fn create_rpc_server(
        uri_provider: Arc<dyn LocalUriProvider>,
        zenoh_config: zenoh_config::Config,
    ) -> Result<InMemoryRpcServer, Box<dyn std::error::Error>> {
        let transport = UPTransportZenoh::builder(uri_provider.get_authority())
            .expect("invalid authority name")
            .with_config(zenoh_config)
            .build()
            .await
            .map(Arc::new)?;
        let rpc_server = InMemoryRpcServer::new(transport.clone(), uri_provider.clone());
        Ok(rpc_server)
    }

    fn initialize_connections(&self, rx_kuksa_horn_is_active: tokio::sync::mpsc::Receiver<bool>) {
        if self.kuksa_enabled {
            self.start_kuksa_databroker_connection(rx_kuksa_horn_is_active);
        } else {
            info!("Printing the horn signal to the terminal since the connection with Kuksa databroker is not enabled (use -k flag).");
            self.start_terminal_connection(rx_kuksa_horn_is_active);
        }
    }

    fn start_kuksa_databroker_connection(
        &self,
        rx_kuksa_horn_is_active: tokio::sync::mpsc::Receiver<bool>,
    ) {
        if self.kuksa_enabled {
            tokio::spawn(connections::send_to_kuksa_databroker(
                rx_kuksa_horn_is_active,
                self.kuksa_address.clone(),
            ));
        }
    }

    fn start_terminal_connection(
        &self,
        rx_kuksa_horn_is_active: tokio::sync::mpsc::Receiver<bool>,
    ) {
        tokio::spawn(connections::send_to_terminal(rx_kuksa_horn_is_active));
    }

    pub async fn from_args(args: Args) -> Result<Self, Box<dyn std::error::Error>> {
        let kuksa_address = args.kuksa_address.clone();
        HornServer::new(
            args.kuksa_enabled,
            kuksa_address,
            args.get_zenoh_config()
                .expect("Invalid Zenoh configuration"),
            "horn-service-kuksa",
        )
        .await
    }
}
