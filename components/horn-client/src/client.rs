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

use std::error::Error;
use up_rust::communication::UPayload;

use horn_proto::horn_service::ActivateHornRequest;

use crate::{
    prebuilt_horn_requests::{
        create_deactivate_horn_request, get_prebuilt_activation_request, PrebuiltHornRequests,
    },
    rpc_client::HornRpcClient,
};

/// A client for interacting with the COVESA Horn service over uProtocol
pub struct HornClient {
    rpc_client: HornRpcClient,
}

impl HornClient {
    pub async fn new(rpc_client: HornRpcClient) -> Result<Self, Box<dyn Error>> {
        Ok(HornClient { rpc_client })
    }

    /// Sends a request to the vehicles horn service to activate the horn with predefined behavior
    /// based on what [`PrebuiltHornRequests`] enum value is provided
    pub async fn activate_horn_with_prebuilt_request(
        &self,
        request_type: PrebuiltHornRequests,
    ) -> Result<(), Box<dyn Error>> {
        let activate_horn_request = get_prebuilt_activation_request(request_type);
        self.activate_horn(activate_horn_request).await
    }

    /// Sends a request to the vehicles horn service to activate the horn with the provided
    /// [`ActivateHornRequest`] message that defines the horn behavior
    pub async fn activate_horn(
        &self,
        activate_horn_request: ActivateHornRequest,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let payload = UPayload::try_from_protobuf(activate_horn_request)?;
        self.rpc_client.send_activate_horn_request(payload).await
    }

    /// Sends a request to the vehicles horn service to deactivate the horn
    pub async fn deactivate_horn(&self) -> Result<(), Box<dyn std::error::Error>> {
        let deactivate_horn_request = create_deactivate_horn_request();
        let deactivate_payload = UPayload::try_from_protobuf(deactivate_horn_request)?;
        self.rpc_client
            .send_deactivate_horn_request(deactivate_payload)
            .await
    }
}
