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

use std::fmt::{Display, Formatter, Result};

/// More like filters
#[derive(Clone)]
pub enum Flags {
    /// Handler will only recieve updates if it's outgoing ones'
    SelfOnly,

    /// Recieves all updates for the handler.
    All,
}

impl Display for Flags {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Flags::SelfOnly => write!(f, "Flags::SelfOnly"),
            Flags::All => write!(f, "Flags::All"),
        }
    }
}
