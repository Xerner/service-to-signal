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

use zenoh::Config;

#[derive(clap::Parser)]
pub struct Args {
    #[arg(short, long, env = "ZENOH_CONFIG")]
    /// A Zenoh configuration file.
    config: PathBuf,
    #[arg(short, long, default_value = "true", env = "IS_SOUND_ENABLED")]
    sound: bool,
}

impl Args {
    pub fn get_zenoh_config(&self) -> Result<Config, Box<dyn std::error::Error>> {
        // Load the config from file path
        zenoh::config::Config::from_file(&self.config).map_err(|e| e as Box<dyn std::error::Error>)
    }
}
