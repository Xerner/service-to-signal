/*******************************************************************************
* Copyright (c) 2025 Contributors to the Eclipse Foundation
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

use clap::Parser;
use env_logger::Env;
use log::info;
use std::sync::Arc;
use up_rust::StaticUriProvider;

use horn_client::config::Args;
use horn_client::constants::{
    HORN_SERVICE_AUTHORITY_NAME, HORN_SERVICE_ENTITY_ID, HORN_SERVICE_MAJOR_VERSION,
};
use horn_client::horn_loop::example_horn_loop;
use horn_client::rpc_client::create_rpc_client;
use horn_client::HornClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = Args::parse();
    let uri_provider: Arc<StaticUriProvider> = Arc::new(StaticUriProvider::new(
        HORN_SERVICE_AUTHORITY_NAME,
        HORN_SERVICE_ENTITY_ID,
        HORN_SERVICE_MAJOR_VERSION,
    ));
    let rpc_client = Arc::new(create_rpc_client(args, uri_provider.clone()).await?);
    let horn_client = HornClient::new(rpc_client.clone(), uri_provider.clone());
    info!("Starting the client for the COVESA Horn service over uProtocol");
    example_horn_loop(horn_client).await?;
    Ok(())
}
