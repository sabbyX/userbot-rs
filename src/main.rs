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
pub(crate) mod versions;
mod tui;
mod cmd;

use grammers_client::{Client, ClientHandle, Config, Update, UpdateIter};
use grammers_session::Session;
use modules::core::dispatcher::UpdateController;
use std::{self, process::exit};

use clap::{crate_version, AppSettings, Clap};
use fern::colors::ColoredLevelConfig;
use log::{self, debug, error, info};
use std::sync::{Arc};
use tokio::runtime;
use std::thread::Builder;
use utils::login::{create_client_backend_connection};
use crate::utils::login::handle_signin_result;

fn setup_logger() -> Result<(), fern::InitError> {
    let color = ColoredLevelConfig::new().debug(fern::colors::Color::Blue);
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                color.color(record.level()),
                message
            ))
        })
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

async fn async_main(
    api_hash: String,
    api_id: i32,
    no_gui: bool,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
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
    if !client.is_authorized().await? {
        info!("User isn't authorized, starting authentication process...");
        let (backend_service, client_service) = create_client_backend_connection();
        Builder::new()
            .spawn(
                move || if no_gui { cmd::no_gui_interface(backend_service) } else { tui::terminal_interface_login(backend_service) }
            )
            .unwrap();
        let (_, phone) = client_service.request("requestPhone");
        match client
            .request_login_code(&phone, api_id, &*api_hash.clone())
            .await
        {
            Ok(token) => {
                let (_, login_code) = client_service.request("requestCode");
                let result = client.sign_in(&token, &login_code).await;
                handle_signin_result(result, &client_service, &mut client).await;
            },
            Err(e) => {
                let error_desc = format!("{}", e);
                // propagate error to client side, should exit
                client_service.error(error_desc.as_str());
                error!("Encountered error: {}", error_desc);
                exit(1);
            }
        };
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
    }
    Ok(())
}

async fn handle_updates(
    updates: UpdateIter,
    controller: Arc<UpdateController>,
    client: ClientHandle,
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    for update in updates {
        if let Update::NewMessage(msg) = update {
            let result = controller.notify(&msg, &client).await;
            match result {
                Ok(_) => {}
                Err(e) => match controller.propogate_error(msg.clone(), e).await {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(e);
                    }
                },
            }
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
    app_hash: String,

    /// Launch userbot in No-GUI way
    #[clap(long)]
    no_gui: bool,
}

fn main() {
    let args: Args = Args::parse();
    // setup the logger
    match setup_logger() {
        Ok(_) => {}
        Err(e) => {
            error!("Internal Error Occured at initiating logger [fern]");
            panic!(e);
        }
    }
    match runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async_main(args.app_hash, args.app_id, args.no_gui))
    {
        Ok(_) => {}
        Err(e) => {
            error!("Unhandled error occured {}", e);
            // panic!(e);
        }
    };
}
