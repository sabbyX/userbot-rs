/*
 * This file is part of the userbot-rs (github.com/sabbyX/userbot-rs).
 * Copyright (c) 2021 Sabby.
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

pub enum CommandPolicy {
    /// No policies will be applied, all updates, no matter of command will be propagated.
    Undefined,

    /// Set the policy to only allow only <strong>ONE</strong> command.
    Command(&'static str),

    /// Set the policy to, allow updates which matches either one of the command
    MultiCommand(Vec<&'static str>),
}
