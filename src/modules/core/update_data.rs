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

use grammers_client::ClientHandle;
use crate::config::Config;

#[derive(Clone)]
pub struct UpdateData {
    pub client: ClientHandle,
    pub config: Config,
}

impl UpdateData {
    pub fn new(client: ClientHandle, config: Config) -> Self {
        Self {
            client,
            config
        }
    }
}
