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

//! Implementation of `dispatcher` aka function to designate command to a handler
//! Inspired from [Carapax](https://github.com/tg-rs/carapax) dispatcher implementation 
mod handler;
mod command;

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn handler(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as handler::HandlerMeta);
    let args = if args.is_empty() {
        None
    } else {
        Some(parse_macro_input!(args as command::CommandMeta))
    };
    TokenStream::from(build(input, args))
}

fn build(handler: handler::HandlerMeta, args: Option<command::CommandMeta>) -> proc_macro2::TokenStream {
    let handler::HandlerMeta {
        ident,
        handler,
        ident_inner,
    } = handler;
    let inner_call = quote!(#ident_inner(message, data).await);
    let command_policy = match args {
        None => quote!(crate::modules::core::command::CommandPolicy::Undefined),
        Some(v) => {
            match v {
                command::CommandMeta::MultiCommand(e) => quote!(crate::modules::core::CommandPolicy::MultiCommand(vec!#e)),
                command::CommandMeta::Command(cmd) => quote!(crate::modules::core::CommandPolicy::Command(#cmd))
            }
        }
    };
    quote!(
        #handler
        #[allow(non_camel_case_types)]
        #[derive(Clone)]
        pub struct #ident;
        #[::async_trait::async_trait]
        impl super::core::handler::Handler for #ident {
            async fn handle(&self, message: Message, data: UpdateData) -> ::anyhow::Result<()> {
                #inner_call
            }

            fn command_policy(&self) -> crate::modules::core::CommandPolicy { #command_policy }
        }
    )
}
