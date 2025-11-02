use horn_proto::horn_service::ActivateHornRequest;
use http::Uri;
use log::info;
use std::sync::Arc;
use tokio::{
    sync::mpsc::{channel, Receiver, Sender},
    task::JoinHandle,
};
use up_rust::{LocalUriProvider, StaticUriProvider, UTransport};
use up_transport_zenoh::UPTransportZenoh;

use horn_common::constants::{HORN_SERVICE_ENTITY_ID, HORN_SERVICE_MAJOR_VERSION};

use crate::{config::Args, connections, horn_rpc_server::HornRpcServer, HornRequestKuksaRelay};

pub struct HornServer {
    pub kuksa_enabled: bool,
    pub kuksa_address: Uri,
    transport: Arc<dyn UTransport>,
    uri_provider: Arc<dyn LocalUriProvider>,
}

impl HornServer {
    pub async fn new(
        kuksa_enabled: bool,
        kuksa_uri: Uri,
        transport: Arc<dyn UTransport>,
        uri_provider: Arc<dyn LocalUriProvider>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let horn_server = HornServer {
            kuksa_enabled,
            kuksa_address: kuksa_uri,
            transport,
            uri_provider,
        };
        Ok(horn_server)
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        info!("Starting the Horn RPC server");
        let (kuksa_is_horn_active_sender, kuksa_is_horn_active_receiver) = channel(32);
        let (horn_request_sender, horn_request_receiver) = channel(4);
        let horn_rpc_server = HornRpcServer::new(
            self.transport.clone(),
            self.uri_provider.clone(),
            horn_request_sender,
        )?;
        start_relaying_requests_to_kuksa(
            horn_request_receiver,
            kuksa_is_horn_active_sender.clone(),
        );
        horn_rpc_server.register_handlers().await?;
        self.initialize_connections(kuksa_is_horn_active_receiver);
        std::thread::park();
        Ok(())
    }

    fn initialize_connections(
        &self,
        kuksa_is_horn_active_receiver: tokio::sync::mpsc::Receiver<bool>,
    ) {
        if self.kuksa_enabled {
            self.start_kuksa_databroker_connection(kuksa_is_horn_active_receiver);
        } else {
            info!("Printing the horn signal to the terminal since the connection with Kuksa databroker is not enabled (use -k flag).");
            self.start_terminal_connection(kuksa_is_horn_active_receiver);
        }
    }

    fn start_kuksa_databroker_connection(
        &self,
        kuksa_is_horn_active_receiver: tokio::sync::mpsc::Receiver<bool>,
    ) {
        if self.kuksa_enabled {
            tokio::spawn(connections::send_to_kuksa_databroker(
                kuksa_is_horn_active_receiver,
                self.kuksa_address.clone(),
            ));
        }
    }

    fn start_terminal_connection(
        &self,
        kuksa_is_horn_active_receiver: tokio::sync::mpsc::Receiver<bool>,
    ) {
        tokio::spawn(connections::send_to_terminal(kuksa_is_horn_active_receiver));
    }

    pub async fn from_args(args: Args) -> Result<Self, Box<dyn std::error::Error>> {
        let kuksa_address = args.kuksa_address.clone();
        let default_uri_provider = default_uri_provider();
        let zenoh_config = args.get_zenoh_config().expect("Failed to get zenoh config");
        let transport = get_zenoh_transport(default_uri_provider.clone(), zenoh_config).await;
        HornServer::new(
            args.kuksa_enabled,
            kuksa_address,
            transport,
            default_uri_provider,
        )
        .await
    }
}

pub async fn get_zenoh_transport(
    uri_provider: Arc<dyn LocalUriProvider>,
    zenoh_config: up_transport_zenoh::zenoh_config::Config,
) -> Arc<dyn UTransport> {
    UPTransportZenoh::try_init_log_from_env();
    let transport = UPTransportZenoh::builder(uri_provider.get_authority())
        .expect("invalid authority name")
        .with_config(zenoh_config)
        .build()
        .await
        .expect("Failed to build Zenoh transport");
    Arc::new(transport)
}

pub fn default_uri_provider() -> Arc<dyn LocalUriProvider> {
    Arc::new(StaticUriProvider::new(
        "horn-service-kuksa",
        HORN_SERVICE_ENTITY_ID,
        HORN_SERVICE_MAJOR_VERSION,
    ))
}

/**
 * Spawns a new tokio task that starts relaying horn requests to Kuksa.
 */
pub fn start_relaying_requests_to_kuksa(
    horn_request_receiver: Receiver<Option<ActivateHornRequest>>,
    kuksa_is_horn_active_sender: Sender<bool>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let horn_request_kuksa_relay =
            HornRequestKuksaRelay::new(horn_request_receiver, kuksa_is_horn_active_sender);
        horn_request_kuksa_relay.relay_requests_to_kuksa().await;
    })
}
