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

use log::debug;

use grammers_client::types::Message;
use anyhow::{Result, Error};
use super::{error_handler::ErrorHandler, handler::{Handler, InternalHandlerStructure}, Flags, UpdateData};

/// Dispatcher, used to register handlers, propagate updates.
/// # Examples
/// ```
/// let controller = UpdateController::new();
/// controller.add_handler(Box::new(some_handler));
/// // Propagate some updates...
/// controller.notify(...);
/// ```
#[derive(Clone)]
pub struct UpdateController {
    handlers: Vec<InternalHandlerStructure>,
    error_handler: Option<Box<dyn ErrorHandler>>,
}

impl UpdateController {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            error_handler: None,
        }
    }

    /// Method to propagate updates to handlers
    /// # Parameters
    /// * `message`: Currently only supports [message](/grammers_client/types/struct.Message.html) _update type_ to be propagated!
    pub async fn notify(&self, message: &Message, data: UpdateData,) -> Result<()> {
        if !message.text().starts_with('*') { return Ok(()); };
        let command = &message.text().split_whitespace().next().unwrap()[1..];
        for handler in &self.handlers {
            // validate command
            if !handler.0.validate_command(command) { continue }
            // handle flags
            match handler.1 {
                Flags::All => (),
                Flags::SelfOnly => {
                    // check whether message is outgoing
                    if !message.outgoing() { continue }
                }
            }
            handler.0.handle(message.clone(), data.clone()).await?
        }
        Ok(())
    }

    /// Method used to register a handler (can be command handler or handler the listens all [messages](/grammers_client/types/struct.Message.html))
    /// # Parameters
    /// * `handler`: An implemenation of trait [`Handler`](../handler/struct.Handler.html) or async function that derives macro [`handler`](/userbot_rs_macros/fn.handler.html)
    /// * `flags`: Something like `filters`; to filter the updates to the handler, see [`Flags`](../flags/enum.Flags.html)
    pub fn add_handler(&mut self, handler: Box<dyn Handler>, flags: Flags) {
        debug!("Adding handler {:p} with flag `{}`", &handler, &flags);
        self.handlers.push(InternalHandlerStructure(handler, flags))
    }

    /// Method used to register an error handler.
    /// # Note
    /// * There should be only <strong>ONE</strong> error handler at a time.
    /// * If registering an error handler will overwrite the one registered before (if any).
    /// * If any `error` happens in [handler](../error_handler/trait.ErrorHandler.html) they will result in _controlled_ exiting of program
    /// # Parameters
    /// * `handler`: Should be implementation of trait [`ErrorHandler`](../error_handler/trait.ErrorHandler.html)
    pub fn add_error_handler(&mut self, handler: Box<dyn ErrorHandler>) {
        debug!("Registering error handler");
        self.error_handler = Some(handler);
    }

    /// Method used to propagate errors to error handler.
    ///
    /// # Parameters
    /// `message`: [message](/grammers_client/types/struct.Message.html) on which handling it causes error.
    /// `error`: Any kind of error, if its unsupported type, use the [converter](/anyhow/macro.anyhow.html).
    pub async fn propogate_error(&self, message: Message, error: Error) -> Result<()> {
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
