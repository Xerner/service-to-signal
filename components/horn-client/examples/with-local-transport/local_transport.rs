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

use env_logger::Env;
use horn_common::uri::default_horn_service_uri_provider;
use log::info;
use std::error::Error;
use std::sync::Arc;
use up_rust::communication::InMemoryRpcClient;
use up_rust::local_transport::LocalTransport;

use horn_client::{HornClient, HornRpcClient, PrebuiltHornRequests};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let horn_client = default_client_setup().await?;
    info!("Starting the client for the COVESA Horn service over a uProtocol local transport");
    example_horn_loop(horn_client).await?;
    Ok(())
}

async fn default_client_setup() -> Result<HornClient, Box<dyn Error>> {
    let horn_service_uri_provider = default_horn_service_uri_provider();
    let transport = Arc::new(LocalTransport::default());
    let rpc_client = InMemoryRpcClient::new(transport, horn_service_uri_provider.clone()).await?;
    let horn_rpc_client =
        HornRpcClient::new(horn_service_uri_provider, Box::new(rpc_client)).await?;
    let horn_client = HornClient::new(horn_rpc_client).await?;
    Ok(horn_client)
}

pub async fn example_horn_loop(horn_client: HornClient) -> Result<(), Box<dyn std::error::Error>> {
    info!("Activating horn for 1500 milliseconds with a sequenced horn");
    horn_client
        .activate_horn_with_prebuilt_request(PrebuiltHornRequests::Sequenced)
        .await?;
    sleep(1500).await;

    info!("Deactivating horn");
    horn_client.deactivate_horn().await?;

    info!("Activating horn for 4000 milliseconds with a continuous horn");
    horn_client
        .activate_horn_with_prebuilt_request(PrebuiltHornRequests::Continuous)
        .await?;
    sleep(4000).await;

    info!("Deactivating horn");
    horn_client.deactivate_horn().await?;

    Ok(())
}

/// tokio sleep wrapper for better code readability
pub(crate) async fn sleep(milliseconds: u64) {
    tokio::time::sleep(std::time::Duration::from_millis(milliseconds)).await;
}
