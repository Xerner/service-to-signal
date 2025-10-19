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

use crate::horn_client::HornClient;
use crate::horn_requests::PrebuiltHornRequests;
use crate::sleep;

pub async fn example_horn_loop(horn_client: HornClient) -> Result<(), Box<dyn std::error::Error>> {
    info!("Activating horn for 1500 milliseconds with a sequenced horn");
    horn_client
        .activate_horn_with_prebuilt_request(PrebuiltHornRequests::Sequenced)
        .await?;
    sleep(1500).await;

    info!("Deactivating horn");
    horn_client.deactivate_horn().await?;

    info!("Activating horn for 4000 milliseconds with a continuous horn");
    horn_client
        .activate_horn_with_prebuilt_request(PrebuiltHornRequests::Continuous)
        .await?;
    sleep(4000).await;

    info!("Deactivating horn");
    horn_client.deactivate_horn().await?;

    Ok(())
}
