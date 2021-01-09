/*
 * This file is part of the userbot-rs (github.com/sabbyX/userbot-rs).
 * Copyright (c) 2020 Sabby.
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

use std::env;
use std::fs;
use std::path;
use std::io::{Read, Write};

const PACKAGES: [&str; 1] = ["grammers-client"];

fn main() {
    // Read Cargo.lock and de-toml it
    let mut lock_buf = String::new();
    fs::File::open("../../Cargo.lock").unwrap().read_to_string(&mut lock_buf).unwrap();
    let lock_toml = toml::Parser::new(&lock_buf).parse().unwrap();

    // Get the table of [[package]]s. This is the deep list of dependencies and dependencies of
    // dependencies.
    let mut version: &str = "0.0.0";
    for package in lock_toml.get("package").unwrap().as_slice().unwrap() {
        let package = package.as_table().unwrap();
        if !PACKAGES.contains(&package.get("name").unwrap().as_str().unwrap()) {continue}
        version = package.get("version").unwrap().as_str().unwrap();
    }

    // Write out the file to be included in the module stub
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut versions_file = fs::File::create(&path::Path::new(&out_dir).join("versions.rs")).unwrap();
    versions_file.write_all(format!("pub const GRAMMERS_VERSION: &str = \"{}\";", version).as_ref()).unwrap();
}
