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
 
use super::core::error_handler::ErrorHandler;
use async_trait::async_trait;
use grammers_client::types::{Message, InputMessage};
use log::{error, warn};

#[derive(Clone)]
pub struct ProdErrorHandler;

#[async_trait]
impl ErrorHandler for ProdErrorHandler {
    async fn handle(&self, mut message: Message, error: Box<dyn std::error::Error + Send + Sync>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        warn!("Handling error {}", error);
        match message.reply(
            InputMessage::text(format!("{}", error))
        ).await {
            Ok(_) => {},
            Err(e) => {
                warn!("Error occured when sending 'handled' error message");
                error!("{}", e);
            }
        };
        Ok(())
    }
}
