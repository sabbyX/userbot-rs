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
 
pub mod core;

// modules
mod info;
mod error;
mod ping;
mod process;

use self::core::dispatcher::UpdateController;
use crate::modules::core::Flags;

/// Initialize all modules and returns the [UpdateController](./core/dispatcher/struct.UpdateController.html)
pub fn initialise() -> UpdateController {
    let mut controller = UpdateController::new();
    controller.add_error_handler(Box::new(error::ProdErrorHandler));
    controller.add_handler(Box::new(info::alive_command), Flags::SelfOnly);
    controller.add_handler(Box::new(ping::ping), Flags::SelfOnly);
    controller.add_handler(Box::new(process::process_command), Flags::SelfOnly);
    controller
}
