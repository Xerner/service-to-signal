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

use horn_common::constants::{ACTIVATE_HORN_RESOURCE_ID, DEACTIVATE_HORN_RESOURCE_ID};
use horn_proto::horn_service::{ActivateHornResponse, DeactivateHornResponse};
use log::{error, info};
use protobuf::MessageFull;
use std::{error::Error, sync::Arc};
use up_rust::{
    communication::{CallOptions, RpcClient, ServiceInvocationError, UPayload},
    LocalUriProvider, StaticUriProvider, UUri,
};

pub struct HornRpcClient {
    uri_provider: Arc<StaticUriProvider>,
    rpc_client: Box<dyn RpcClient>,
}

impl HornRpcClient {
    pub async fn new(
        uri_provider: Arc<StaticUriProvider>,
        rpc_client: Box<dyn RpcClient>,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(HornRpcClient {
            uri_provider,
            rpc_client,
        })
    }

    pub async fn send_activate_horn_request(
        &self,
        request: UPayload,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let uri = self.get_activate_horn_uri();
        self.send_rpc_request::<ActivateHornResponse>(uri, request)
            .await
    }

    pub async fn send_deactivate_horn_request(
        &self,
        request: UPayload,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let uri = self.get_deactivate_horn_uri();
        self.send_rpc_request::<DeactivateHornResponse>(uri, request)
            .await
    }

    fn get_activate_horn_uri(&self) -> UUri {
        self.uri_provider
            .get_resource_uri(ACTIVATE_HORN_RESOURCE_ID)
    }

    fn get_deactivate_horn_uri(&self) -> UUri {
        self.uri_provider
            .get_resource_uri(DEACTIVATE_HORN_RESOURCE_ID)
    }

    async fn send_rpc_request<MessageImpl: MessageFull>(
        &self,
        uri: UUri,
        request: UPayload,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match self
            .rpc_client
            .invoke_method(
                uri.clone(),
                CallOptions::for_rpc_request(1_000, None, None, None),
                Some(request),
            )
            .await
        {
            Ok(Some(payload)) => self.handle_rcp_successful_response::<MessageImpl>(Some(payload)),
            Ok(None) => self.handle_rcp_empty_response(),
            Err(e) => self.handle_rpc_error_response(e),
        }
    }

    fn handle_rcp_successful_response<MessageImpl: MessageFull>(
        &self,
        returned_message: Option<UPayload>,
    ) -> Result<(), Box<dyn Error>> {
        if returned_message.is_none() {
            error!("successfully sent RPC request");
            return Ok(());
        }
        let response = returned_message
            .unwrap()
            .extract_protobuf::<MessageImpl>()
            .unwrap();
        info!(
            "successfully sent RPC request with response: {:?}",
            response
        );
        Ok(())
    }

    fn handle_rcp_empty_response(&self) -> Result<(), Box<dyn std::error::Error>> {
        error!("RPC request returned an empty response");
        Ok(())
    }

    fn handle_rpc_error_response(
        &self,
        error: ServiceInvocationError,
    ) -> Result<(), Box<dyn std::error::Error>> {
        error!("RPC request returned an error: {:?}", error);
        Ok(())
    }
}
