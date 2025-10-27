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

use clap::Parser;
use env_logger::Env;
use log::info;
use std::sync::Arc;
use up_rust::{
    communication::{InMemoryRpcServer, RpcServer},
    LocalUriProvider, StaticUriProvider,
};
use up_transport_zenoh::UPTransportZenoh;

use horn_service_kuksa::{config, request_handler, request_processor, HornServer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let args = config::Args::parse();
    info!("Starting the Horn service");
    let horn_server = HornServer::from_args(args.clone());
    horn_server.start().await?;
    std::thread::park();
    Ok(())
}
