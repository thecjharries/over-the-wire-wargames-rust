// Copyright 2023 CJ Harries
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Major portions of this file come from russh's examples
// https://github.com/warp-tech/russh/blob/main/russh/examples/remote_shell_call.rs

use std::io::Write;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use async_trait::async_trait;
use russh::*;
use russh_keys::*;

use crate::{get_level_password, load_settings};

pub struct CommandResult {
    output: Vec<u8>,
    code: Option<u32>,
}

impl CommandResult {
    pub fn output(&self) -> String {
        String::from_utf8_lossy(&self.output).into()
    }

    pub fn success(&self) -> bool {
        self.code == Some(0)
    }
}

pub struct Client {}

#[async_trait]
impl client::Handler for Client {
    type Error = anyhow::Error;

    async fn check_server_key(
        self,
        _server_public_key: &key::PublicKey,
    ) -> Result<(Self, bool), Self::Error> {
        Ok((self, true))
    }
}

pub struct Session {
    session: client::Handle<Client>,
}

impl Session {
    pub async fn connect(host: &str, port: &str, user: &str, password: &str) -> Result<Self> {
        let config = client::Config {
            inactivity_timeout: Some(Duration::from_secs(5)),
            ..<_>::default()
        };
        let config = Arc::new(config);
        let sh = Client {};
        let mut session = client::connect(config, format!("{host}:{port}"), sh).await?;
        let _auth_res = session.authenticate_password(user, password).await?;
        Ok(Self { session })
    }

    pub async fn call(&mut self, command: &str) -> Result<CommandResult> {
        let mut channel = self.session.channel_open_session().await?;
        channel.exec(true, command).await?;
        let mut output = Vec::new();
        let mut code = None;
        while let Some(msg) = channel.wait().await {
            match msg {
                russh::ChannelMsg::Data { ref data } => {
                    output.write_all(data).unwrap();
                }
                russh::ChannelMsg::ExitStatus { exit_status } => {
                    code = Some(exit_status);
                }
                _ => {}
            }
        }
        Ok(CommandResult { output, code })
    }

    pub async fn close(&mut self) -> Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}

pub async fn level1_password() -> String {
    let settings = load_settings("bandit");
    let host = settings.get_string("host").unwrap();
    let port = settings.get_string("port").unwrap();
    let user = "bandit0";
    let password = "bandit0";
    let mut session = Session::connect(&host, &port, &user, &password)
        .await
        .unwrap();
    let result = session.call("cat readme").await.unwrap();
    session.close().await.unwrap();
    result.output().trim().to_string()
}

pub async fn level2_password() -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn session_can_connect_to_bandit_host() {
        let settings = load_settings("bandit");
        let host = settings.get_string("host").unwrap();
        let port = settings.get_string("port").unwrap();
        let user = "bandit0";
        let password = "bandit0";
        let mut session = Session::connect(&host, &port, &user, &password)
            .await
            .unwrap();
        let result = session.call("echo hello").await.unwrap();
        assert_eq!("hello\n", result.output());
        assert!(result.success());
        session.close().await.unwrap();
    }

    #[tokio::test]
    async fn level1_password_returns_proper_value() {
        let settings = load_settings("bandit");
        let host = settings.get_string("host").unwrap();
        let port = settings.get_string("port").unwrap();
        let user = "bandit1";
        let password = level1_password().await;
        let mut session = Session::connect(&host, &port, &user, &password)
            .await
            .unwrap();
        let result = session.call("echo hello").await.unwrap();
        assert_eq!("hello\n", result.output());
        assert!(result.success());
        session.close().await.unwrap();
    }
}
