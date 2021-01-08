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
use grammers_client::InputMessage;
use grammers_tl_types as tl;
use std::time;
use userbot_rs_macros::handler;
use kantex_rs::{Document, Sections, FormattedText, Stringify};
use anyhow::Result;
use crate::modules::core::UpdateData;

#[derive(Clap)]
#[clap(name = "ping", setting = AppSettings::NoBinaryName, version = "0.1.0")]
struct Arguments;

#[handler(command = "ping")]
pub async fn ping(mut message: Message, mut data: UpdateData,) -> Result<()> {
    if let Err(args) = Arguments::try_parse_from(
        message
            .text()
            .trim_start_matches("*ping")
            .split_whitespace()
    ) {
        let result = message.reply(InputMessage::text(format!("{}", args))).await;
        if let Err(e) = result { message.reply(InputMessage::text(e.to_string())).await? }
    } else {
        let start = time::Instant::now();
        data.client.invoke(&tl::functions::Ping { ping_id: 0 }).await?;
        let mut ping_ = (time::Instant::now() - start).as_millis().to_string();
        ping_.push_str(" ms");
        message.reply(InputMessage::html(
            Document::new()
                .add_section(
                    Sections::new("Pong")
                        .add(FormattedText::italics(ping_.as_str()))
                ).stringify()
        )).await?;
    }
    Ok(())
}
