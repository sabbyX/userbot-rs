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
/// Unifies all data that may not directly related to the update the receives.
/// Such as [Config](/userbot_rs/config/struct.Config.html), [ClientHandle](/grammers_client/struct.ClientHandle.html)
pub struct UpdateData {
    /// [ClientHandle](/grammers_client/struct.ClientHandle.html), used to invoke API calls
    pub client: ClientHandle,
    /// Handlers could access the configuration file using this field
    pub config: Config,
}

impl UpdateData {
    /// Creates new instance of `UpdateData`
    /// # Example
    /// ```
    /// # use grammers_client::Client;
    /// # let client: Client
    /// let update_data = UpdateData::new(
    ///         client.handle(),
    ///         Config {
    ///             telegram: Telegram {
    ///                 api_id: 0,
    ///                 api_hash: "123",
    ///             }
    ///         }
    ///     )
    /// ```
    pub fn new(client: ClientHandle, config: Config) -> Self {
        Self {
            client,
            config
        }
    }
}
