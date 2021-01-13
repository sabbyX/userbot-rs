/*
 * This file is part of the userbot-rs (github.com/sabbyX/userbot-rs).
 * Copyright (c) 2020 Sabby.
 * Copyright (c) Bi Channel Project (github.com/SOF3/bi_channel.rs)
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

use std::time::Duration;
use grammers_client::{SignInError, Client};
use log::{debug, warn, info};
use grammers_client::types::User;
use std::{borrow::Cow, future::Future, pin::Pin};
use crossbeam_channel::{unbounded, Sender, Receiver, SendError, TryRecvError, RecvTimeoutError};
use std::process::exit;

type ExternalChannel = BiChannel<<ClientBiMessage as BiMessage>::In, <ClientBiMessage as BiMessage>::Out>;
type InternalChannel = BiChannel<<ClientBiMessage as BiMessage>::Out, <ClientBiMessage as BiMessage>::In>;
type Out<'a> = (Cow<'a, str>, Cow<'a, str>);

const MAX_CLIENT_TIMEOUT_PERIOD: u64 = 60;

#[derive(Clone)]
pub struct BiChannel<Sent, Received> {
    pub sender: Sender<Sent>,
    pub receiver: Receiver<Received>,
}

impl<Sent, Received> BiChannel<Sent, Received> {
    pub fn send(&self, send: Sent) -> Result<(), SendError<Sent>> { self.sender.send(send) }

    // pub fn recv(&self) -> Result<Received, RecvError> { self.receiver.recv() }
    pub fn recv_timeout(&self, timeout: Duration) -> Result<Received, RecvTimeoutError> { self.receiver.recv_timeout(timeout) }
    pub fn try_recv(&self) -> Result<Received, TryRecvError> { self.receiver.try_recv() }
}

pub trait BiMessage {
    type In;
    type Out;
}

#[derive(Clone)]
pub struct BiChannelPair<M: BiMessage> {
    pub internal: BiChannel<M::Out, M::In>,
    pub external: BiChannel<M::In, M::Out>,
}

pub fn bi_channel<M: BiMessage>() -> BiChannelPair<M> {
    let (in_send, in_recv) = unbounded::<M::In>();
    let (out_send, out_recv) = unbounded::<M::Out>();
    BiChannelPair {
        internal: BiChannel { sender: out_send, receiver: in_recv },
        external: BiChannel { sender: in_send, receiver: out_recv },
    }
}

#[derive(Clone)]
pub struct BackendConnection {
    internal: InternalChannel,
}

impl BackendConnection {
    pub fn new(internal: InternalChannel) -> Self {
        Self {
            internal
        }
    }

    pub fn request(&self, msg: &str) -> Out {
        self.internal.send(("command".into(), msg.into())).unwrap();
        let (r0, r1) = self.internal.recv_timeout(Duration::from_secs(MAX_CLIENT_TIMEOUT_PERIOD)).unwrap();
        (r0.into(), r1.into())
    }

    pub fn error(&self, msg: &str) -> Out {
        self.internal.send(("error".into(), msg.into())).unwrap();
        let (r0, r1) = self.internal.recv_timeout(Duration::from_secs(MAX_CLIENT_TIMEOUT_PERIOD)).unwrap();
        (r0.into(), r1.into())
    }
}

#[derive(Clone)]
pub struct ClientConnection {
    pub external: ExternalChannel,
}

impl ClientConnection {
    pub fn new(external: ExternalChannel) -> Self {
        Self {
            external,
        }
    }

    pub fn answer(&self, value: &str, success: bool) {
        self.external.send(({ if success { "ok" } else { "error" }}.into(), value.into())).unwrap();
    }
}

pub fn create_client_backend_connection() -> (ClientConnection, BackendConnection) {
    let BiChannelPair { external, internal } = bi_channel::<ClientBiMessage>();
    (ClientConnection::new(external), BackendConnection::new(internal))
}

pub struct ClientBiMessage;

impl BiMessage for ClientBiMessage {
    type In = (String, String);
    type Out = (String, String);
}

pub async fn handle_signin_result<SessionType: grammers_session::Session>(result: Result<User, SignInError>, client_service: &BackendConnection, gram_client: &mut Client<SessionType>) {
    match result {
        Err(e) => {
            match e {
                SignInError::InvalidCode => {
                    warn!("Got invalid login code");
                    client_service.error(format!("{}", e).as_str());
                    exit(1)
                }
                SignInError::SignUpRequired { .. } => todo!(),
                SignInError::PasswordRequired(token) => {
                    debug!("Signin requires 2FA authentication");
                    let (_, password) = client_service.request("passwordAuth");
                    let result = gram_client.check_password(token, &*password).await;
                    let f: Pin<Box<dyn Future<Output=_>>> = Box::pin(handle_signin_result(result, client_service, gram_client));
                    f.await;
                    // handle_signin_result(result, client_service, gram_client).await;
                }
                SignInError::InvalidPassword => {
                    client_service.error(format!("{}", e).as_str());
                    exit(1)
                }
                SignInError::Other(e) => {
                    let error_desc = format!("{}", e);
                    client_service.error(error_desc.as_str());
                    exit(1)
                }
            }
        }
        Ok(user) => {
            info!("successfully logined as {}", user.id());
            let result = client_service.request("loginSuccess");
            match (result.0.as_ref(), result.1.as_ref()) {
                ("error", _) => {
                    warn!("Client refused to start bot, exiting");
                    exit(1);
                },
                _ => {
                    info!("Starting the bot..!");
                }
            };
        }
    };
}

pub(crate) enum Query {
    Command(String),
    Error(String)
}

pub(crate) fn parse_query(query: (String, String)) -> Query {
    match (query.0.as_str(), query.1) {
        ("command", command) => Query::Command(command),
        ("error", error) => Query::Error(error),
        (_, _) => panic!("Got unexpected query")
    }

}
