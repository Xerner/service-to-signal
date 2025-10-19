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

use horn_proto::{
    horn_service::{ActivateHornRequest, DeactivateHornRequest},
    horn_topics::{HornCycle, HornMode, HornSequence},
};

pub enum PrebuiltHornRequests {
    Sequenced,
    Continuous,
}

pub fn get_prebuilt_activation_request(request_type: PrebuiltHornRequests) -> ActivateHornRequest {
    match request_type {
        PrebuiltHornRequests::Sequenced => create_sequenced_horn_request(),
        PrebuiltHornRequests::Continuous => create_continuous_horn_request(),
    }
}

/// Constructs a horn activation request that commands the horn to cycle through a predefined
/// sequence of on/off durations (milliseconds).
///   1. on: 100, off: 100
///   2. on: 200, off: 300
///   3. on: 100, off: 200
///   4. on: 10000, off: 500
fn create_sequenced_horn_request() -> ActivateHornRequest {
    ActivateHornRequest {
        mode: HornMode::HM_SEQUENCED.into(),
        command: vec![HornSequence {
            horn_cycles: vec![
                HornCycle {
                    on_time: 100,
                    off_time: 100,
                    ..Default::default()
                },
                HornCycle {
                    on_time: 200,
                    off_time: 300,
                    ..Default::default()
                },
                HornCycle {
                    on_time: 100,
                    off_time: 200,
                    ..Default::default()
                },
                HornCycle {
                    on_time: 10000,
                    off_time: 500,
                    ..Default::default()
                },
            ],
            ..Default::default()
        }],
        ..Default::default()
    }
}

/// Constructs a horn activation request that commands the horn to sound continuously
/// until deactivated by a separate request.
fn create_continuous_horn_request() -> ActivateHornRequest {
    ActivateHornRequest {
        mode: HornMode::HM_CONTINUOUS.into(),
        command: vec![HornSequence {
            horn_cycles: vec![],
            ..Default::default()
        }],
        ..Default::default()
    }
}

/// Constructs a request to simply deactivate the horn
pub(crate) fn create_deactivate_horn_request() -> DeactivateHornRequest {
    DeactivateHornRequest::default()
}
