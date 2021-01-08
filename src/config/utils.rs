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

use std::{fs::{File, create_dir_all}, path::PathBuf};
use std::env::var_os;
use log::error;
use directories::ProjectDirs;
use dialoguer::console::style;

// data to find project dir
const ORGANIZATION: &str = "userbot-rs";
const QUALIFIER: &str = "";
const APPLICATION_NAME: &str = "telegram-userbot";

// config file/path data
const CONFIG_PATH_ENV: &str = "USERBOT_CONFIG_PATH";
const CONFIG_FILENAME: &str = "config.ini";

/// get configuration file path according to Operating System
///
/// # Parameters
/// `create_file`: Creates new config file if none exists
pub fn get_config_path(create_file: bool) -> PathBuf {
    let project_dir = ProjectDirs::from(
        QUALIFIER,
        ORGANIZATION,
        APPLICATION_NAME,
    );
    let config_path = if let Some(paths) = project_dir {
        paths.config_dir().to_path_buf().join("config.ini")
    } else {
        let path_env_var = &var_os(CONFIG_PATH_ENV);
        if path_env_var.is_none() {
            error!(
                "Cannot determine the configuration path from Operating system, you could set custom config path using env var `{}`",
                style(CONFIG_PATH_ENV).blue().bold()
            );
            std::process::exit(1)
        } else {
            let path = path_env_var.as_ref().unwrap();
            PathBuf::from(path).join(CONFIG_FILENAME)
        }
    };
    if !config_path.exists() && create_file {
        // creates conf file if it doesnt exists
        if let Some(parent) = config_path.parent() {
            // make sure dir exists
            // TODO: handle Result
            if !parent.exists()  { create_dir_all(parent).unwrap(); }
        }
        // TODO: handle Result
        File::create(config_path.clone()).unwrap();
    }
    config_path
}
