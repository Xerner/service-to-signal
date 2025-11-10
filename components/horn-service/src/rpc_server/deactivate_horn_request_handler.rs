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

use horn_proto::horn_service::{
    ActivateHornRequest, DeactivateHornRequest, DeactivateHornResponse,
};
use horn_proto::status::Status;
use log::info;
use protobuf::MessageField;
use up_rust::communication::{RequestHandler, ServiceInvocationError, UPayload};
use up_rust::UAttributes;

pub(crate) struct DeactivateHornRpcRequestHandler {
    horn_request_receiver: tokio::sync::mpsc::Sender<Option<ActivateHornRequest>>,
}

impl DeactivateHornRpcRequestHandler {
    pub fn new(
        horn_request_receiver: tokio::sync::mpsc::Sender<Option<ActivateHornRequest>>,
    ) -> Self {
        Self {
            horn_request_receiver,
        }
    }
}

#[async_trait::async_trait]
impl RequestHandler for DeactivateHornRpcRequestHandler {
    async fn handle_request(
        &self,
        _resource_id: u16,
        _message_attributes: &UAttributes,
        request_payload: Option<UPayload>,
    ) -> Result<Option<UPayload>, ServiceInvocationError> {
        info!("DeactivateHornRequest received");

        // Expect the deactivate horn request
        // to be empty.
        let _req = request_payload
            .unwrap()
            .extract_protobuf::<DeactivateHornRequest>()
            .unwrap();
        let _ = self.horn_request_receiver.send(None).await;
        let response = DeactivateHornResponse {
            status: MessageField::some(Status::new()),
            ..Default::default()
        };
        let payload = UPayload::try_from_protobuf(response).unwrap();
        Ok(Some(payload))
    }
}
