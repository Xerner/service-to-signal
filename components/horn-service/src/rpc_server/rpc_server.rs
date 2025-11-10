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

use std::{error::Error, sync::Arc};
use tokio::sync::mpsc::Sender;
use up_rust::communication::RpcServer;

use crate::rpc_server::{ActivateHornRpcRequestHandler, DeactivateHornRpcRequestHandler};
use horn_common::constants::{ACTIVATE_HORN_RESOURCE_ID, DEACTIVATE_HORN_RESOURCE_ID};
use horn_proto::horn_service::ActivateHornRequest;

pub struct HornRpcServer {
    rpc_server: Box<dyn RpcServer>,
    horn_request_sender: Sender<Option<ActivateHornRequest>>,
}

impl HornRpcServer {
    pub fn new(
        rpc_server: Box<dyn RpcServer>,
        horn_request_sender: Sender<Option<ActivateHornRequest>>,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(HornRpcServer {
            rpc_server,
            horn_request_sender,
        })
    }

    pub async fn register_handlers(&self) -> Result<(), Box<dyn Error>> {
        self.register_activate_horn_handler().await?;
        self.register_deactivate_horn_handler().await?;
        Ok(())
    }

    pub(crate) async fn register_activate_horn_handler(&self) -> Result<(), Box<dyn Error>> {
        let activate_horn_rpc_request_handler = Arc::new(ActivateHornRpcRequestHandler::new(
            self.horn_request_sender.clone(),
        ));
        self.rpc_server
            .register_endpoint(
                None,
                ACTIVATE_HORN_RESOURCE_ID,
                activate_horn_rpc_request_handler,
            )
            .await?;
        Ok(())
    }

    pub(crate) async fn register_deactivate_horn_handler(&self) -> Result<(), Box<dyn Error>> {
        let deactivate_horn_rpc_request_handler = Arc::new(DeactivateHornRpcRequestHandler::new(
            self.horn_request_sender.clone(),
        ));
        self.rpc_server
            .register_endpoint(
                None,
                DEACTIVATE_HORN_RESOURCE_ID,
                deactivate_horn_rpc_request_handler,
            )
            .await?;
        Ok(())
    }
}
