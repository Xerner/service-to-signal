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

use horn_proto::horn_service::{ActivateHornRequest, ActivateHornResponse};
use log::{error, info};
use std::sync::Arc;
use up_rust::{
    communication::{CallOptions, RpcClient, UPayload},
    LocalUriProvider, StaticUriProvider,
};

use crate::{
    constants::{ACTIVATE_HORN_RESOURCE_ID, DEACTIVATE_HORN_RESOURCE_ID},
    horn_requests::{
        create_deactivate_horn_request, get_prebuilt_activation_request, PrebuiltHornRequests,
    },
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
        match self
            .rpc_client
            .invoke_method(
                activate_horn_uri.clone(),
                CallOptions::for_rpc_request(1_000, None, None, None),
                Some(payload),
            )
            .await
        {
            Ok(Some(payload)) => {
                let response = payload.extract_protobuf::<ActivateHornResponse>()?;
                info!("Activate Horn returned message: {}", response);
                Ok(())
            }
            Ok(None) => {
                info!("The activate horn request returned an empty response");
                Ok(())
            }
            Err(e) => {
                error!("The activate horn request returned the error: {:?}", e);
                Ok(())
            }
        }
    }

    /// Sends a request to the vehicles horn service to deactivate the horn
    pub async fn deactivate_horn(&self) -> Result<(), Box<dyn std::error::Error>> {
        let deactivate_horn_uri = self
            .uri_provider
            .get_resource_uri(DEACTIVATE_HORN_RESOURCE_ID);
        let deactivate_horn_request = create_deactivate_horn_request();
        let deactivate_payload = UPayload::try_from_protobuf(deactivate_horn_request)?;

        match self
            .rpc_client
            .invoke_method(
                deactivate_horn_uri.clone(),
                CallOptions::for_rpc_request(1_000, None, None, None),
                Some(deactivate_payload),
            )
            .await
        {
            Ok(Some(_)) => {
                info!("The deactivate horn request returned successfully");
                Ok(())
            }
            Ok(None) => {
                error!("The deactivate horn request returned an empty response");
                Ok(())
            }
            Err(e) => {
                error!("The deactivate horn request returned the error: {:?}", e);
                Ok(())
            }
        }
    }
}
