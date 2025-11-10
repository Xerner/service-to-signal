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

use horn_proto::{horn_service::ActivateHornRequest, horn_topics::HornMode};
use log::{debug, error};

use crate::actuator_client::HornActuatorClient;

pub struct HornService {
    horn_actuator_client: Box<dyn HornActuatorClient>,
}

impl HornService {
    pub async fn new(
        horn_actuator_client: Box<dyn HornActuatorClient>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let horn_server = HornService {
            horn_actuator_client,
        };
        Ok(horn_server)
    }

    /**
    Handles sending horn requests to Kuksa.

    - If the request is None, it deactivates the horn.
    - If the request is Some, it checks the horn mode and sends the appropriate request to Kuksa
    */
    pub async fn process_horn_activation_request(&self, request: Option<ActivateHornRequest>) {
        if request.is_none() {
            // treat None as a signal to deactivate the horn
            self.send_deactivate_horn_request().await;
            return;
        }
        let request = request.unwrap();
        let horn_mode = request.mode.enum_value();
        if let Err(e) = horn_mode {
            error!("Error in Horn Mode value {:?}", e);
        };
        let mode = horn_mode.unwrap();
        match mode {
            HornMode::HM_SEQUENCED => {
                self.send_sequential_horn_request_to_actuator(request).await;
            }
            HornMode::HM_CONTINUOUS => self.send_continuous_horn_request_to_actuator().await,
            HornMode::HM_UNKNOWN => println!("Horn Mode: Unknown"),
            HornMode::HM_UNSPECIFIED => println!("Horn Mode: Unspecified"),
        };
    }

    /**
    Sends a deactivate horn request to Kuksa.
    */
    async fn send_deactivate_horn_request(&self) {
        debug!("Sending deactivate horn request");
        self.horn_actuator_client.set_horn_active(false).await;
    }

    /**
    Sends a continuous horn request to Kuksa.
    */
    async fn send_continuous_horn_request_to_actuator(&self) {
        debug!("Sending continuous horn request");
        self.horn_actuator_client.set_horn_active(true).await;
    }

    /**
    Sends sequential horn requests to Kuksa.

    For each sequence in the request, it iterates through the horn cycles,
    activating and deactivating the horn based on the specified on and off times.
    */
    async fn send_sequential_horn_request_to_actuator(&self, request: ActivateHornRequest) {
        debug!("Sending sequential horn request(s)");
        let sequences = request.command;
        for sequence in sequences {
            for cycle in sequence.horn_cycles {
                debug!("\nOn Time: {}, Off Time: {}", cycle.on_time, cycle.off_time);
                self.horn_actuator_client.set_horn_active(true).await;
                let _ = tokio::time::sleep(std::time::Duration::from_millis(cycle.on_time as u64))
                    .await;
                self.horn_actuator_client.set_horn_active(false).await;
                let _ = tokio::time::sleep(std::time::Duration::from_millis(cycle.off_time as u64))
                    .await;
            }
        }
    }
}
