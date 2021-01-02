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
use grammers_client::ClientHandle;
use anyhow::Result;
use crate::modules::core::Flags;
use crate::modules::core::command::CommandPolicy;

/// Implement `Handler` using the trait
/// # Examples
/// ```
/// struct Some;
/// impl Handler for Some {...}
/// ```
#[async_trait]
pub trait Handler: HandlerClone + Send + Sync {
    /// This method will first to be called by [`UpdateController`](../dispatcher/struct.UpdateController.html)
    async fn handle(&self, message: Message, client: ClientHandle) -> Result<()>;
    fn command_policy(&self) -> CommandPolicy;
}

impl dyn Handler {
    pub fn validate_command(&self, v: &str) -> bool {
        match self.command_policy() {
            CommandPolicy::Undefined => true,
            CommandPolicy::Command(cmd) => {
                println!("{}", cmd);
                cmd == v
            },
            CommandPolicy::MultiCommand(cmds) => cmds.contains(&v)
        }
    }
}

pub trait HandlerClone {
    fn clone_box(&self) -> Box<dyn Handler>;
}

impl<T> HandlerClone for T
where 
    T: 'static + Handler + Clone 
{
    fn clone_box(&self) -> Box<dyn Handler> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Handler> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

#[derive(Clone)]
pub(super) struct InternalHandlerStructure(pub Box<dyn Handler>, pub Flags);
