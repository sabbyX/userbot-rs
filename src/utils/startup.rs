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

use crate::config::{Config, Telegram};

/// Generates [Config](../../../config/schema/struct.Config.html) from given `api id` and `api hash`
/// As its not generated from config.toml, its considered to be fake
pub fn generate_fake_config(api_id: i32, api_hash: String) -> Config {
    Config {
        telegram: Telegram {
            api_id,
            api_hash
        }
    }
}
