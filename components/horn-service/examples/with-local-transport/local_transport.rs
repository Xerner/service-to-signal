/*******************************************************************************
* Copyright (c) 2024 Contributors to the Eclipse Foundation
*
* See the NOTICE file(s) distributed with this work for additional
* information regarding copyright ownership.
*
* This program and the accompanying materials are made available under the
* terms of the Eclipse Public License 2.0 which is available at
* http://www.eclipse.org/legal/epl-2.0
*
* SPDX-License-Identifier: EPL-2.0
*******************************************************************************/

use std::sync::Arc;

use clap::Parser;
use env_logger::Env;
use log::info;
use tokio::{
    select,
    sync::mpsc::{channel, Receiver, Sender},
};
use up_rust::{communication::InMemoryRpcServer, local_transport::LocalTransport};

use horn_common::uri::default_horn_service_uri_provider;
use horn_proto::horn_service::ActivateHornRequest;

use horn_service::{
    actuator_client::HornActuatorClient,
    config::{self, Args},
    HornKuksaClient, HornRpcServer, HornService, TerminalClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = config::Args::parse();
    // a channel is used because
    // - one thread will listen for incoming RPC requests
    // - another thread will process the requests and send signals to the horn actuator
    // and the channel allows communication between these two threads in a thread-safe manner
    let (horn_request_sender, horn_request_receiver) = channel(4);
    let (horn_service, horn_rpc_server) = default_setup(args, horn_request_sender).await?;
    // on a separate thread, process horn requests
    tokio::spawn(async move {
        relay_requests_to_the_horn_service(horn_service, horn_request_receiver).await;
    });
    // on the current thread, listen and relay horn requests
    info!("Listening for incoming horn RPC requests using a local transport");
    horn_rpc_server.register_handlers().await?;
    std::thread::park();
    Ok(())
}

async fn default_setup(
    args: Args,
    horn_request_sender: Sender<Option<ActivateHornRequest>>,
) -> Result<(HornService, HornRpcServer), Box<dyn std::error::Error>> {
    let uri_provider = default_horn_service_uri_provider();
    let transport = Arc::new(LocalTransport::default());
    let horn_actuator_client: Box<dyn HornActuatorClient> = if args.kuksa_enabled {
        info!("Kuksa Databroker integration enabled");
        Box::new(HornKuksaClient::new(args.kuksa_address))
    } else {
        info!("Kuksa Databroker integration disabled, using terminal client");
        Box::new(TerminalClient::default())
    };
    let rpc_server = InMemoryRpcServer::new(transport, uri_provider.clone());
    let horn_rpc_server = HornRpcServer::new(Box::new(rpc_server), horn_request_sender)?;
    let horn_service = HornService::new(horn_actuator_client).await?;
    Ok((horn_service, horn_rpc_server))
}

/**
Blocks and continuously receives horn requests. Unblocks and stops receiving
requests when the request channel returns None.

Listens to the activate horn request channel and sends the requests to the
horn service, which then sends signals to the horn actuator.
*/
pub(crate) async fn relay_requests_to_the_horn_service(
    horn_service: HornService,
    mut horn_request_receiver: Receiver<Option<ActivateHornRequest>>,
) {
    while let Some(activate_horn_request) = horn_request_receiver.recv().await {
        // this select! macro will kill the execution of the previous request
        // if a new request is received
        let _: Option<()> = select! {
            _ = horn_request_receiver.recv() => None,
            _ = horn_service.process_horn_activation_request(activate_horn_request) => None,
        };
    }
}
