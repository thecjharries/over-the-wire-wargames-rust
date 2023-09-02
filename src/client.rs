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

use async_ssh2_tokio::client::{AuthMethod, Client, ServerCheckMethod};

use crate::{get_level_password, load_settings};

pub async fn get_client_from_settings(wargame: &str, level: u8) -> Client {
    get_client_from_settings_with_password(
        wargame,
        level,
        get_level_password(load_settings(wargame), level),
    )
    .await
}

pub async fn get_client_from_settings_with_password(
    wargame: &str,
    level: u8,
    password: String,
) -> Client {
    let settings = load_settings(wargame);
    let host = settings.get_string("host").unwrap();
    let port = settings.get_int("port").unwrap();
    let user = format!("bandit{}", level);
    Client::connect(
        (host, port as u16),
        &user,
        AuthMethod::Password(password),
        ServerCheckMethod::NoCheck,
    )
    .await
    .unwrap()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn session_can_connect_to_bandit_host() {
        let client = get_client_from_settings("bandit", 0).await;
        let result = client.execute("echo hello").await.unwrap();
        assert_eq!("hello\n", result.stdout);
        assert_eq!(0, result.exit_status);
    }
}
