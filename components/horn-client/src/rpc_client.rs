/*******************************************************************************
* Copyright (c) 2025 Contributors to the Eclipse Foundation
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

use log::{info, trace};
use std::sync::Arc;
use up_rust::{communication::InMemoryRpcClient, LocalUriProvider, StaticUriProvider};
use up_transport_zenoh::UPTransportZenoh;

use crate::config::Args;

/// Creates an in-memory uProtocol RPC client using uProtocols Zenoh transport
pub async fn create_rpc_client(
    args: Args,
    uri_provider: Arc<StaticUriProvider>,
) -> Result<InMemoryRpcClient, Box<dyn std::error::Error>> {
    info!("Reading Zenoh config at: {:?}", args.config);
    let config = args.get_zenoh_config()?;
    trace!("Using Zenoh config: {:?}", config);
    let transport = UPTransportZenoh::builder(uri_provider.get_authority())
        .expect("invalid authority name")
        .with_config(config)
        .build()
        .await
        .map(Arc::new)?;

    Ok(InMemoryRpcClient::new(transport.clone(), uri_provider.clone()).await?)
}
