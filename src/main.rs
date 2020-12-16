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

#![feature(in_band_lifetimes)]

mod modules;
mod utils;

use grammers_client::{Client, ClientHandle, Config, SignInError, Update, UpdateIter};
use grammers_session::Session;
use modules::core::dispatcher::UpdateController;
use std::{self, process::exit};

use tokio::runtime;
use std::sync::Arc;
use fern::colors::ColoredLevelConfig;
use log::{self, debug, error, info};
use clap::{AppSettings, Clap, crate_version};

fn setup_logger() -> Result<(), fern::InitError> {
    let color = ColoredLevelConfig::new()
        .debug(fern::colors::Color::Blue);
    fern::Dispatch::new()
        .format(
            move |out, message, record| {
                out.finish(
                    format_args!(
                        "[{}][{}] {}",
                        record.target(),
                        color.color(record.level()),
                        message
                    )
                )
            }
        )
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

async fn async_main(api_hash: String, api_id: i32) -> std::result::Result<(), Box<dyn std::error::Error>> {
    info!("Connecting to Telegram...");
    let mut client = Client::connect(Config {
        session: Session::load_or_create("userbot").unwrap(),
        api_id,
        api_hash: api_hash.clone(),
        params: Default::default(),
    })
    .await
    .unwrap();
    debug!("Successfully connected..!");
    debug!("Checking whether user is authenticated...");
    if !client
        .is_authorized()
        .await?
    {
        info!("User isn't authorized, starting authentication process...");
        let mut phone = String::new();
        utils::startup::prompt(
            "Please enter your phone number (including dial code): ",
            &mut phone,
            false,
        )?;
        let login_token = match client
            .request_login_code(phone.as_str(), api_id, &*api_hash.clone())
            .await
        {
            Ok(token) => token,
            Err(e) => {
                error!("Authentication process failed {}", e);
                exit(1);
            }
        };
        // info!("Logging in...");
        let mut login_code = String::new();
        utils::startup::prompt("Enter login code: ", &mut login_code, false)?;
        let signin = client.sign_in(&login_token, &login_code.trim()).await;
        match signin {
            Ok(user) => info!(
                "Successfully signed in as {} ({})", user.first_name.unwrap_or("<No Name>".to_string()), user.id
            ),
            Err(e) => {
                match e {
                    SignInError::SignUpRequired { .. } => {},
                    SignInError::InvalidCode => {
                        //TODO: Retry support
                        error!("Invalid code...");
                        exit(1);
                    }
                    SignInError::InvalidPassword => {
                        // TODO: Retry support
                        error!("Invalid password");
                        exit(1);
                    }
                    SignInError::Other(err) => {
                        error!("Invocation error occured...");
                        panic!(err)
                    }
                    SignInError::PasswordRequired(token) => {
                        debug!("2FA enabled... starting 2FA authentication process!");
                        let no_hint_available = "< No Hint Available >".to_string();
                        let hint = token.hint().unwrap_or(&no_hint_available);
                        let prompt_message = format!("Enter the password (hint {})", hint);
                        let mut password = String::new();
                        utils::startup::prompt(prompt_message.as_str(), &mut password, false)?;
                        client
                            .check_password(token, &password.trim())
                            .await?;
                    }
                }
            },
        }
    }
    info!("Initialising modules...");
    let controller = Arc::new(modules::initialise());
    while let Some(updates) = client.next_updates().await.unwrap() {
        let client_handle = client.handle();
        let arc_controller = controller.clone();
        tokio::task::spawn(async move {
            match handle_updates(updates, arc_controller, client_handle).await {
                Ok(_) => {}
                Err(e) => {
                    error!("Error while handling updates {}", e);
                }
            }
        });
    };
    Ok(())
}

async fn handle_updates(
    updates: UpdateIter,
    controller: Arc<UpdateController>,
    client: ClientHandle,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    for update in updates {
        match update {
            Update::NewMessage(msg) => {
                let result = controller.notify(&msg, &client).await;
                match result {
                    Ok(_) => {},
                    Err(e) => {
                        match controller.propogate_error(msg.clone(), e.into()).await {
                            Ok(_) => {},
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

/// Yet another userbot, but in rust!
#[derive(Clap)]
#[clap(name = "userbot-rs", author = "Sabby", setting = AppSettings::ArgRequiredElseHelp, version = crate_version!())]
struct Args {
    /// App id, provided by telegram, get it from telegram.org
    #[clap(long)]
    app_id: i32,

    /// App hash, provided by telegram, get it from telegram.org
    #[clap(long)]
    app_hash: String
}

fn main() {
    let args = Args::parse();
    // setup the logger
    match setup_logger() {
        Ok(_) => {},
        Err(e) => {
            error!("Internal Error Occured at initiating logger [fern]");
            panic!(e);
        }
    }
    match runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main(args.app_hash, args.app_id))
    {
        Ok(_) => {},
        Err(e) => {
            error!("Unhandled error occured {}", e);
            // panic!(e);
        }
    };
}
