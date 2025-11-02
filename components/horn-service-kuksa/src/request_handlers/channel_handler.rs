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

use log::info;
use protobuf::MessageFull;
use tokio::sync::mpsc::Sender;
use up_rust::{
    communication::{RequestHandler, ServiceInvocationError, UPayload},
    UAttributes,
};

pub(crate) struct ChannelRpcRequestHandler<TRequest, TResponse>
where
    TRequest: MessageFull,
    TResponse: MessageFull,
{
    transfer_channel: Sender<Option<TRequest>>,
    response_generator: fn(TRequest) -> TResponse,
}

impl<TRequest, TResponse> ChannelRpcRequestHandler<TRequest, TResponse>
where
    TRequest: MessageFull,
    TResponse: MessageFull,
{
    pub fn new(
        transfer_channel: Sender<Option<TRequest>>,
        response_generator: fn(TRequest) -> TResponse,
    ) -> Self {
        Self {
            transfer_channel,
            response_generator,
        }
    }
}

#[async_trait::async_trait]
impl<TRequest, TResponse> RequestHandler for ChannelRpcRequestHandler<TRequest, TResponse>
where
    TRequest: MessageFull,
    TResponse: MessageFull,
{
    async fn handle_request(
        &self,
        _resource_id: u16,
        _message_attributes: &UAttributes,
        request_payload: Option<UPayload>,
    ) -> Result<Option<UPayload>, ServiceInvocationError> {
        info!("Handle new request to apply horn sequence");

        let req = request_payload
            .unwrap()
            .extract_protobuf::<TRequest>()
            .unwrap();
        let _ = self.transfer_channel.send(Some(req.clone())).await;
        let response = self.response_generator(req);
        let payload = UPayload::try_from_protobuf(response).unwrap();
        Ok(Some(payload))
    }
}
