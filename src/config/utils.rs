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
use std::ffi::OsString;

pub fn get_config_path() -> PathBuf {
    let path = PathBuf::from(&var_os("USERBOT_CONFIG_PATH").unwrap_or_else(|| OsString::from("data/config.ini")));
    if !path.exists() {
        // creates conf file if it doesnt exists
        if let Some(parent) = path.parent() {
            // make sure dir exists
            if !parent.exists()  { create_dir_all(parent); }
        }
        File::create(path.clone());
    }
    path
}
