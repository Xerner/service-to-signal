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

pub(crate) mod horn_client;
pub(crate) mod horn_requests;

pub mod config;
pub mod constants;
pub use horn_client::HornClient;
pub mod horn_loop;
pub mod rpc_client;

/// tokio sleep wrapper for better code readability
pub(crate) async fn sleep(milliseconds: u64) {
    tokio::time::sleep(std::time::Duration::from_millis(milliseconds)).await;
}
