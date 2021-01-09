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

use async_trait::async_trait;
use grammers_client::types::Message;
use anyhow::{Result, Error};

/// Implement `Error Handler`
/// # Examples
/// ```
/// struct Some;
/// impl ErrorHandler for Some {
///     async fn handle(...) {...}
/// }
/// ```
#[async_trait]
pub trait ErrorHandler: HandlerClone + Send + Sync {
    /// This method will first to be called by [`UpdateController`](../dispatcher/struct.UpdateController.html)
    async fn handle(&self, message: Message, error: Error) -> Result<()>;
}

pub trait HandlerClone {
    fn clone_box(&self) -> Box<dyn ErrorHandler>;
}

impl<T> HandlerClone for T
where 
    T: 'static + ErrorHandler + Clone 
{
    fn clone_box(&self) -> Box<dyn ErrorHandler> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn ErrorHandler> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
