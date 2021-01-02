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

use clap::{AppSettings, Clap};
use grammers_client::types::Message;
use grammers_client::{ClientHandle, InputMessage};
use kantex_rs::*;
use userbot_rs_macros::handler;
use anyhow::Result;

use crate::versions;

/// Shows information about userbot
#[derive(Clap)]
#[clap(name = "userbot", setting = AppSettings::NoBinaryName, version = "0.1.0")]
struct Arguments {

    /// Check for updates, installs if any.
    #[clap(long, short)]
    update: bool
}

#[handler(command = "userbot")]
pub async fn alive_command(
    mut message: Message,
    _: ClientHandle,
) -> Result<()> {
    let args: Arguments = match Arguments::try_parse_from(
        message
            .text()
            .trim_start_matches("*userbot")
            .split_whitespace(),
    ) {
        Ok(v) => v,
        Err(e) => {
            let result = message.reply(InputMessage::text(format!("{}", e))).await;
            return if result.is_err() {
                Err(result.unwrap_err().into())
            } else {
                Ok(())
            }
        }
    };
    if args.update {}
    let text = Document::new().add_section(
        Sections::new("Userbot")
            .add(FormattedText::bold("github.com/sabbyX/userbot-rs"))
            .add(KeyValueItem::new(
                FormattedText::bold("version"),
                env!("CARGO_PKG_VERSION"),
            ))
            .add(KeyValueItem::new(
                FormattedText::bold("grammers"),
                versions::GRAMMERS_VERSION,
            )),
    );
    message.reply(InputMessage::html(text.stringify())).await?;
    Ok(())
}
