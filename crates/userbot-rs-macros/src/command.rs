/*
 * This file is part of the userbot-rs-macros (github.com/sabbyX/userbot-rs-macros).
 * Copyright (c) 2021 Sabby.
 * Copyright (c) Carapax authors (github.com/tg-rs/carapax)
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

use syn::{parse::{Parse, ParseStream}, Token, Ident, LitStr, ExprArray, Result as SynResult, Expr, Lit};

pub enum CommandMeta {
    Command(LitStr),
    MultiCommand(ExprArray)
}

impl Parse for CommandMeta {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let command = input.parse::<Ident>()?;
        match command.to_string().as_ref() {
            "command" | "cmd" => {
                input.parse::<Token![=]>()?;
                Ok(Self::Command(input.parse::<LitStr>()?))
            }
            "commands" | "cmds" => {
                input.parse::<Token![=]>()?;
                let token = input.parse::<ExprArray>()?;
                for expression in token.elems.iter() {
                    match expression {
                        Expr::Lit(lit) => {
                            match lit.lit {
                                Lit::Str(_) => continue,
                                _ => return Err(input.error("Expected string literals")),
                            }
                        }
                        _ => return Err(input.error("Expected string literals")),
                    }
                }
                Ok(Self::MultiCommand(token))
            }
            arg => {
                Err(input.error(format!("Unexpected arg {}", arg)))
            }
        }
    }
}
