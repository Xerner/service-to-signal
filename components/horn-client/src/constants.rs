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

// see the up-spec to learn more about what these constants represent in the world of uProtocol
// https://github.com/eclipse-uprotocol/up-spec/blob/main/basics/uri.adoc#2-data-model

/// The expected name of the deployed network location/domain that the horn service lives on
///
/// This value should match the name of the container in the docker compose file used to deploy
/// the horn service
pub const HORN_SERVICE_AUTHORITY_NAME: &str = "horn-service-kuksa";

/// The expected ID that represents the horn services software component. It is simply an
/// identifier with no deeper meaning
///
/// see https://github.com/COVESA/uservices/blob/main/src/main/proto/vehicle/body/horn/v1/horn_service.proto
pub const HORN_SERVICE_ENTITY_ID: u32 = 28;

/// The version of the horn services uProtocol interface
pub const HORN_SERVICE_MAJOR_VERSION: u8 = 1;

/// The identifier that maps to the horn services "ActivateHorn" RPC method
pub const ACTIVATE_HORN_RESOURCE_ID: u16 = 0x0001;

/// The identifier that maps to the horn services "DeactivateHorn" RPC method
pub const DEACTIVATE_HORN_RESOURCE_ID: u16 = 0x0002;
