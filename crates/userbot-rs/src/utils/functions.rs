/*
 * This file is part of the userbot-rs (github.com/sabbyX/userbot-rs).
 * Copyright (c) 2021 Sabby.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use crate::config;
use dialoguer::console::style;
use std::fs::remove_file;
use std::path::PathBuf;
use crate::config::{ConfigControl, get_config_path};

/// Function to reset the configuration file, delicately made to function `--reset-conf` flag.
/// Yet, it will exit immediately after the reset
pub fn reset_configuration_file() -> ! {
    let config_path = config::get_config_path(false);
    if !config_path.exists() {
        println!("{}", style(format!("No configurations files found at default path ({}), aborting reset.", config_path.display())).yellow());
    } else {
        let result = remove_file(config_path);
        if let Err(e) = result {
            println!("{}", style(format!("Failed to reset configuration file: {}", e)));
        } else {
            println!("{}", style("Successfully reset the configuration file.").green())
        }
    }
    std::process::exit(0)
}

/// Alerts user that no `API_ID` / `API_HASH` found in configuration files
pub fn alert_no_api_conf_found() -> ! {
    println!(
        "Can't fetch `{}` or `{}`, view help by using flag `{}` or `{}`",
        style("api_id").blue(),
        style("api_hash").blue(),
        style("-h").green(),
        style("--help").green()
    );
    std::process::exit(1)
}

// returns configuration file, if not found exits
pub fn extract_ok_configuration(path: Option<PathBuf>) -> ConfigControl {
    match ConfigControl::get_config(path.clone()) {
        None => {
            log::error!("Failed to get configuration file from ({})", path.unwrap_or_else(|| get_config_path(false)).display());
            std::process::exit(1)
        }
        Some(val) => val
    }
}
