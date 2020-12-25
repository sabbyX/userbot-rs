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

use std::thread;

use crossbeam_channel::{Sender, TryRecvError};
use cursive::Cursive;
use cursive::traits::{Nameable, Resizable};
use cursive::views::{
    Dialog, LinearLayout, PaddedView, TextArea, TextView,
};

use crate::utils::login::{ClientConnection, parse_query, Query};

type CbSinkSenderType = Sender<Box<dyn FnOnce(&mut Cursive) + Send>>;

pub fn terminal_interface_login(backend_service: ClientConnection) {
    let mut siv = cursive::default();
    siv.add_global_callback('q', |c| c.quit());
    siv.add_layer(Dialog::new()
        .title("userbot.rs")
        .content(TextView::new("Loading...")));
    let cb_sink = siv.cb_sink().clone();
    thread::Builder::new()
        .spawn(
            || handle_backend_service(backend_service, cb_sink)
        )
        .unwrap();
    siv.run();
}

fn handle_backend_service(backend_service: ClientConnection, cb_sink: CbSinkSenderType) {
    loop {
        // let backend_service_ = backend_service.clone();
        match backend_service.external.try_recv() {
            Err(error) => {
                match error {
                    TryRecvError::Empty => continue,
                    TryRecvError::Disconnected => panic!("Backend service disconnected, leaving out client service")
                }
            },
            Ok(query) => {
                let backend_service = backend_service.clone();
                match parse_query(query) {
                    Query::Command(command) => {
                        match command.as_str() {
                            "requestPhone" => cb_sink.send(Box::new(move |c| request_phone(c, backend_service))).unwrap(),
                            "requestCode" => cb_sink.send(Box::new(move |c| request_code(c, backend_service))).unwrap(),
                            "passwordAuth" => cb_sink.send(Box::new(move |c| password_auth(c, backend_service))).unwrap(),
                            "loginSuccess" => {
                                cb_sink.send(Box::new(move |c| login_sucess(c, backend_service))).unwrap();
                                // break client service
                                break
                            }
                            _ => todo!()
                        }
                    },
                    Query::Error(error) => {
                        cb_sink.send(Box::new(move |s| error_dialog(s, error))).unwrap();
                        // break client service
                        break
                    }
                };
            }
        }
    }
}

fn request_phone(siv: &mut Cursive, backend_service: ClientConnection) {
    let mut view = Dialog::new()
        .title("Login")
        .content(
            LinearLayout::vertical()
                .child(
                    PaddedView::lrtb(1, 0, 2, 1, TextView::new("Enter your phone number"))
                )
                .child(
                    PaddedView::lrtb(
                        1, 0, 0, 0,
                        TextArea::new()
                            .with_name("phone_field")
                            .max_height(1)
                            .max_width(30)
                    )
                )
        );
    view.add_button("Next", move |siv| {
        if let Some(field) = siv.find_name::<TextArea>("phone_field") {
            let phone_number = field.get_content();
            if phone_number.is_empty() { return; }
            backend_service.answer(phone_number, true);
            // pop the current layer and show loading view
            siv.pop_layer();
            siv.add_layer(loading_screen());
        }
    });
    siv.pop_layer();
    siv.add_layer(view);
}

fn request_code(siv: &mut Cursive, backend_service: ClientConnection) {
    let mut view = Dialog::new()
        .title("Login")
        .content(
            LinearLayout::vertical()
                .child(
                    PaddedView::lrtb(1, 0, 2, 1, TextView::new("Enter login code"))
                )
                .child(
                    PaddedView::lrtb(
                        1, 0, 0, 0,
                        TextArea::new()
                            .with_name("code_field")
                            .max_height(1)
                            .max_width(30)
                    )
                )
        );
    view.add_button("Login", move |siv| {
        if let Some(field) = siv.find_name::<TextArea>("code_field") {
            let login_code = field.get_content();
            if login_code.is_empty() { return; }
            backend_service.answer(login_code, true);
            siv.pop_layer();
            siv.add_layer(loading_screen());
        }
    });
    siv.pop_layer();
    siv.add_layer(view);
}

fn password_auth(siv: &mut Cursive, backend_service: ClientConnection) {
    let mut view = Dialog::new()
        .title("Login")
        .content(
            LinearLayout::vertical()
                .child(
                    PaddedView::lrtb(1, 0, 2, 1, TextView::new("Enter 2FA password"))
                )
                .child(
                    PaddedView::lrtb(
                        1, 0, 0, 0,
                        TextArea::new()
                            .with_name("password_field")
                            .max_height(1)
                            .max_width(30)
                    )
                )
        );
    view.add_button("Login", move |siv| {
        if let Some(field) = siv.find_name::<TextArea>("password_field") {
            let password = field.get_content();
            if password.is_empty() { return; }
            backend_service.answer(password, true);
            siv.pop_layer();
            siv.add_layer(loading_screen());
        }
    });
    siv.pop_layer();
    siv.add_layer(view);
}

fn error_dialog(siv: &mut Cursive, error: String) {
    let mut view = Dialog::new()
        .title("Unexpected error occurred")
        .content(
            TextView::new(error)
        );
    view.add_button("Quit", |s| s.quit());
    siv.pop_layer();
    siv.add_layer(view);
}

fn login_sucess(siv: &mut Cursive, backend_service: ClientConnection) {
    let mut view = Dialog::new()
        .title("userbot.rs")
        .content(
            TextView::new("Login success!")
        );
    let backend_service_clone = backend_service.clone();
    view.add_button("Quit", move |c| { c.quit(); backend_service_clone.answer("ok", false); });
    view.add_button("Run the bot", move |c| {c.quit(); backend_service.answer("ok", true)});
    siv.pop_layer();
    siv.add_layer(view);
}

fn loading_screen() -> Dialog {
    Dialog::new()
        .title("userbot.rs")
        .content(
            TextView::new("Loading")
        )
}
