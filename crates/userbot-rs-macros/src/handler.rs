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

use syn::{Ident, ItemFn, parse::{Parse, ParseStream}, Result as SynResult};
use proc_macro2::Span;

pub(crate) struct HandlerMeta {
    pub(super) ident_inner: Ident,
    pub(super) handler: ItemFn,
    pub(super) ident: Ident,
}

impl Parse for HandlerMeta {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut handler = input.parse::<ItemFn>()?;
        if handler.sig.asyncness.is_none() {
            return Err(input.error("function must be async"));
        }
        let ident = handler.sig.ident.clone();
        let ident_inner = Ident::new(&format!("__userbot_{}", ident), Span::call_site());
        handler.sig.ident = ident_inner.clone();
        Ok(HandlerMeta {
            handler,
            ident_inner,
            ident
        })
    }
}
