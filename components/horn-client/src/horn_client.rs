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

use log::{error, info};
use protobuf::MessageFull;
use std::sync::Arc;
use up_rust::{
    communication::{CallOptions, RpcClient, ServiceInvocationError, UPayload},
    LocalUriProvider, StaticUriProvider, UUri,
};

use horn_common::constants::{ACTIVATE_HORN_RESOURCE_ID, DEACTIVATE_HORN_RESOURCE_ID};
use horn_proto::horn_service::{ActivateHornRequest, ActivateHornResponse};

use crate::horn_requests::{
    create_deactivate_horn_request, get_prebuilt_activation_request, PrebuiltHornRequests,
};

/// A client for interacting with the COVESA Horn service over uProtocol
pub struct HornClient {
    rpc_client: Arc<dyn RpcClient>,
    uri_provider: Arc<StaticUriProvider>,
}

impl HornClient {
    pub fn new(rpc_client: Arc<dyn RpcClient>, uri_provider: Arc<StaticUriProvider>) -> Self {
        HornClient {
            rpc_client,
            uri_provider,
        }
    }

    /// Sends a request to the vehicles horn service to activate the horn with predefined behavior
    /// based on what [`PrebuiltHornRequests`] enum value is provided
    pub async fn activate_horn_with_prebuilt_request(
        &self,
        request_type: PrebuiltHornRequests,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let activate_horn_request = get_prebuilt_activation_request(request_type);
        self.activate_horn(activate_horn_request).await
    }

    /// Sends a request to the vehicles horn service to activate the horn with the provided
    /// [`ActivateHornRequest`] message that defines the horn behavior
    pub async fn activate_horn(
        &self,
        activate_horn_request: ActivateHornRequest,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let activate_horn_uri = self
            .uri_provider
            .get_resource_uri(ACTIVATE_HORN_RESOURCE_ID);

        let payload = UPayload::try_from_protobuf(activate_horn_request)?;
        self.send_rpc_request(activate_horn_uri, payload).await
    }

    /// Sends a request to the vehicles horn service to deactivate the horn
    pub async fn deactivate_horn(&self) -> Result<(), Box<dyn std::error::Error>> {
        let deactivate_horn_uri = self
            .uri_provider
            .get_resource_uri(DEACTIVATE_HORN_RESOURCE_ID);
        let deactivate_horn_request = create_deactivate_horn_request();
        let deactivate_payload = UPayload::try_from_protobuf(deactivate_horn_request)?;
        self.send_rpc_request(deactivate_horn_uri, deactivate_payload)
            .await
    }

    async fn send_rpc_request(
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
            Ok(Some(payload)) => self.handle_rcp_successful_response::<ActivateHornResponse>(
                "activate horn",
                Some(payload),
            ),
            Ok(None) => self.handle_rcp_empty_response("activate horn"),
            Err(e) => self.handle_rpc_error_response("activate horn", e),
        }
    }

    fn handle_rcp_successful_response<MessageImpl: MessageFull>(
        &self,
        context: &str,
        returned_message: Option<UPayload>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if returned_message.is_none() {
            error!(
                "The horn service RPC request to '{}' was successful",
                context
            );
            return Ok(());
        }
        let response = returned_message
            .unwrap()
            .extract_protobuf::<MessageImpl>()
            .unwrap();
        info!(
            "The horn service RPC request to '{}' was successful with the payload: {:?}",
            context, response
        );
        Ok(())
    }

    fn handle_rcp_empty_response(&self, context: &str) -> Result<(), Box<dyn std::error::Error>> {
        error!(
            "The horn service RPC request to '{}' returned an empty response",
            context
        );
        Ok(())
    }

    fn handle_rpc_error_response(
        &self,
        context: &str,
        error: ServiceInvocationError,
    ) -> Result<(), Box<dyn std::error::Error>> {
        error!(
            "The horn service RPC request to '{}' returned an error: {:?}",
            context, error
        );
        Ok(())
    }
}
