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

use super::utils::get_config_path;
use log::{error, warn};
use dialoguer::console::style;

pub fn load_config() -> Option<ini::Ini> {
    match ini::Ini::load_from_file(get_config_path()) {
        Ok(val) => Some(val),
        Err(e) => {
            match e {
                ini::Error::Parse(e) => {
                    error!("Failed to parse configuration file: {}", style(e.to_string()).red().bold());
                    warn!(
                        "Exiting due to previous error, try using `{}` to reset configuration file safely!",
                        style("--reset-conf").green().bold()
                    );
                    std::process::exit(1);
                },
                ini::Error::Io(e) => {
                    warn!("Io Error occured when fetching configuration file: {}", style(e.to_string()).bold().red());
                    None
                }
            }
        }
    }
}
