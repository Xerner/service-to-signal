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

use horn_proto::horn_service::{ActivateHornRequest, ActivateHornResponse};
use horn_proto::status::Status;
use log::info;
use protobuf::MessageField;
use tokio::sync::mpsc::Sender;
use up_rust::communication::{RequestHandler, ServiceInvocationError, UPayload};
use up_rust::UAttributes;

pub(crate) struct ActivateHornRpcRequestHandler {
    horn_request_receiver: Sender<Option<ActivateHornRequest>>,
}

impl ActivateHornRpcRequestHandler {
    pub fn new(horn_request_receiver: Sender<Option<ActivateHornRequest>>) -> Self {
        Self {
            horn_request_receiver,
        }
    }
}

#[async_trait::async_trait]
impl RequestHandler for ActivateHornRpcRequestHandler {
    async fn handle_request(
        &self,
        _resource_id: u16,
        _message_attributes: &UAttributes,
        request_payload: Option<UPayload>,
    ) -> Result<Option<UPayload>, ServiceInvocationError> {
        info!("ActivateHornRequest received");

        let req = request_payload
            .unwrap()
            .extract_protobuf::<ActivateHornRequest>()
            .unwrap();
        let _ = self.horn_request_receiver.send(Some(req.clone())).await;
        let response = ActivateHornResponse {
            status: MessageField::some(Status::new()),
            ..Default::default()
        };
        let payload = UPayload::try_from_protobuf(response).unwrap();
        Ok(Some(payload))
    }
}
