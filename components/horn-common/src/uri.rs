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

use std::sync::Arc;
use up_rust::StaticUriProvider;

use crate::constants::{
    HORN_SERVICE_AUTHORITY_NAME, HORN_SERVICE_ENTITY_ID, HORN_SERVICE_MAJOR_VERSION,
};

pub fn default_horn_service_uri_provider() -> Arc<StaticUriProvider> {
    Arc::new(StaticUriProvider::new(
        HORN_SERVICE_AUTHORITY_NAME,
        HORN_SERVICE_ENTITY_ID,
        HORN_SERVICE_MAJOR_VERSION,
    ))
}
