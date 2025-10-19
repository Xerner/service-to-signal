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

use std::path::PathBuf;
use up_transport_zenoh::zenoh_config;

#[derive(clap::Parser, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Args {
    #[arg(
        short,
        long,
        env = "ZENOH_CONFIG",
        default_value = "/zenoh-config.json5"
    )]
    /// A Zenoh configuration file.
    pub config: PathBuf,
}

impl Args {
    pub fn get_zenoh_config(&self) -> Result<zenoh_config::Config, Box<dyn std::error::Error>> {
        // Load the config from file path
        zenoh_config::Config::from_file(&self.config).map_err(|e| e as Box<dyn std::error::Error>)
    }
}
