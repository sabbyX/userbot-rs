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
