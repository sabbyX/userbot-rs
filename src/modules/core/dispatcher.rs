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

use grammers_client::ClientHandle;
use log::debug;

use super::{error_handler::ErrorHandler, handler::Handler};
use grammers_client::types::Message;

#[derive(Clone)]
pub struct UpdateController {
    handlers: Vec<Box<dyn Handler>>,
    error_handler: Option<Box<dyn ErrorHandler>>,
    //pub client: Client,
}

impl UpdateController {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            error_handler: None,
            //client,
        }
    }

    pub async fn notify(
        &self,
        message: &Message,
        client: &ClientHandle,
    ) -> ::std::result::Result<(), Box<dyn ::std::error::Error + Send + Sync>> {
        for handler in &self.handlers {
            handler.handle(message.clone(), client.clone()).await?;
        }
        Ok(())
    }

    pub fn add_handler(&mut self, handler: Box<dyn Handler>) {
        debug!("Adding handler... ({:p})", &handler);
        self.handlers.push(handler);
    }

    pub fn add_error_handler(&mut self, handler: Box<dyn ErrorHandler>) {
        self.error_handler = Some(handler);
    }

    pub async fn propogate_error(&self, message: Message,error: Box<dyn Error + Send + Sync>) -> Result<(), Box<dyn Error + Send + Sync>> {
        if let Some(handler) = &self.error_handler {
            let result = handler.handle(message, error).await;
            if result.is_err() {
                Err(result.unwrap_err())
            } else {
                Ok(())
            }
        } else {
            Err(error)
        }
    }
}
