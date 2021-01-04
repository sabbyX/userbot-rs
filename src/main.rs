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

mod modules;
mod utils;
pub(crate) mod versions;
mod tui;
mod cmd;
mod config;

use grammers_client::{Client, ClientHandle, Config, Update, UpdateIter};
use grammers_session::Session;
use modules::core::dispatcher::UpdateController;
use std::process::exit;
use anyhow::Result;

use clap::{crate_version, AppSettings, Clap};
use fern::colors::ColoredLevelConfig;
use log::{debug, error, info, LevelFilter};
use std::sync::{Arc};
use tokio::runtime;
use std::thread::Builder;
use utils::login::{create_client_backend_connection};
use crate::utils::login::handle_signin_result;
use dialoguer::console::style;
use crate::config::ConfigControl;

fn setup_logger() -> Result<(), fern::InitError> {
    let color = ColoredLevelConfig::new()
        .debug(fern::colors::Color::BrightBlue)
        .info(fern::colors::Color::BrightGreen);
    // check whether if app is launched in debug target
    let level = if cfg!(debug_assertions) { LevelFilter::Trace } else { LevelFilter::Info };
    fern::Dispatch::new()
        .level(level)
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {}",
                color.color(record.level()),
                record.target(),
                message
            ))
        })
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

/*
Main function

initialise modules or launch interactive login if user isn't signed in already
*/
async fn async_main(
    config_control: config::ConfigControl,
) -> anyhow::Result<()> {
    info!("Connecting to Telegram...");
    let telegram_conf = config_control.get_telegram_conf()
        .ok_or_else(|| anyhow::anyhow!("Failed to get telegram configuration"))?;
    let mut client = Client::connect(Config {
        session: Session::load_or_create("userbot")?,
        api_id: telegram_conf.api_id,
        api_hash: telegram_conf.api_hash.clone(),
        params: Default::default(),
    })
    .await?;
    debug!("Successfully connected..!");
    debug!("Checking whether user is authenticated...");
    if !client.is_authorized().await? {
        info!("User isn't authorized, starting authentication process...");
        let (backend_service, client_service) = create_client_backend_connection();
        /*if !args.no_gui {
            // TODO:
            warn!("GUI login experienced has disabled, see https://github.com/sabbyX/userbot-rs/issues/1");
        }*/
        Builder::new()
            .spawn(
                move || cmd::no_gui_interface(backend_service)
            )?;
        let (_, phone) = client_service.request("requestPhone");
        match client
            .request_login_code(
                &phone,
                telegram_conf.api_id,
                telegram_conf.api_hash.as_str()
            )
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
    info!("Sucessfully intialized all modules");
    while let Ok(Some(updates)) = client.next_updates().await {
        // copy values
        let client_handle = client.handle();
        let controller = controller.clone();
        tokio::task::spawn(async move {
            debug!("Starting to recieve updates");
            match handle_updates(updates, controller, client_handle).await {
                Ok(_) => {}
                Err(e) => {
                    error!("Error while handling updates {}", e);
                }
            }
        });
    }
    Ok(())
}

/// Function to handle updates
async fn handle_updates(
    updates: UpdateIter,
    controller: Arc<UpdateController>,
    client: ClientHandle,
) -> Result<()> {
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
    debug!("Checking for saved telegram configuration");
    let conf_is_exist = ConfigControl::check_section_exists("telegram");
    let config_control = match conf_is_exist {
        true => ConfigControl::get_config().unwrap(),
        false => {
            let args = Args::parse() as Args;
            ConfigControl::write_raw_telegram_conf(args.app_id, args.app_hash);
            ConfigControl::get_config().unwrap()
        }
    };
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
        .block_on(async_main(config_control))
    {
        Ok(_) => {}
        Err(e) => {
            error!("Unhandled error occured: {}", style(e.to_string()).red().bold());
            // panic!(e);
        }
    };
}
