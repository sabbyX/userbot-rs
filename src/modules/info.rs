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
 
use std::error::Error;

use clap::{AppSettings, Clap};
use grammers_client::types::Message;
use grammers_client::{ClientHandle, InputMessage};
use userbot_rs_macros::handler;

/// Shows information about userbot
#[derive(Clap)]
#[clap(name = "info", setting = AppSettings::NoBinaryName, version = "0.1.0")]
struct Arguments {}

#[handler(command = "*info")]
pub async fn alive_command(mut message: Message, _: ClientHandle) -> Result<(), Box<dyn Error + Send + Sync>> {
    match Arguments::try_parse_from(
        message
            .text()
            .trim_start_matches("*info")
            .split_whitespace()
    )
    {
        Ok(v) => v,
        Err(e) => {
            let result = message.reply(
                InputMessage::text(format!("{}", e))
            ).await;
            if result.is_err() {
                return Err(result.unwrap_err().into());
            } else {
                return Ok(());
            }
        }
    };
    Ok(())
}
