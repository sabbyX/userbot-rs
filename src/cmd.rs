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

use crate::utils::login::{ClientConnection, parse_query, Query};
use crossbeam_channel::TryRecvError;
use dialoguer::{Input, Password, Confirm};
use dialoguer::console::style;
use dialoguer::theme::ColorfulTheme;

pub fn no_gui_interface(backend_service: ClientConnection) {
    loop {
        match backend_service.external.try_recv() {
            Err(e) => {
                match e {
                    TryRecvError::Empty => continue,
                    TryRecvError::Disconnected => panic!("Backend thread disconnected, leaving out client")
                }
            },
            Ok(query) => {
                let backend_service = backend_service.clone();
                match parse_query(query) {
                    Query::Command(command) => {
                        match command.as_str() {
                            "requestPhone" => {
                                let phone_number = Input::<String>::with_theme(&ColorfulTheme::default())
                                    .with_prompt("Enter your phone number (International format)")
                                    .interact_text()
                                    .unwrap();
                                backend_service.answer(phone_number.as_str(), true);
                            },
                            "requestCode" => {
                                let code = Input::<String>::with_theme(&ColorfulTheme::default())
                                    .with_prompt("Enter the recieved login code")
                                    .interact_text()
                                    .unwrap();
                                backend_service.answer(code.as_str(), true);
                            },
                            "passwordAuth" => {
                                let password = Password::with_theme(&ColorfulTheme::default())
                                    .with_prompt("Enter your 2FA password")
                                    .with_confirmation("Confirm password", "Mismatched password")
                                    .interact()
                                    .unwrap();
                                backend_service.answer(password.as_str(), true);
                            },
                            "loginSuccess" => {
                                if Confirm::with_theme(&ColorfulTheme::default()).with_prompt("Successfully signed in, do you want to run the bot").interact().unwrap() {
                                    backend_service.answer("ok", true);
                                } else {
                                    backend_service.answer("ok", false);
                                }
                                break
                            },
                            _ => todo!()
                        }
                    },
                    Query::Error(error) => {
                        println!("{}", style(format!("Encountered an error: {}", error)).red().bold());
                        break
                    },
                }
            }
        };
    }
}
