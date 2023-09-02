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
pub async fn level1_password() -> String {
    let client = get_client_from_settings("bandit", 0).await;
    let result = client.execute("cat readme").await.unwrap();
    result.stdout.trim().to_string()
}

#[cfg(not(tarpaulin_include))]
pub async fn level2_password() -> String {
    let client = get_client_from_settings("bandit", 1).await;
    let result = client.execute("cat ./-").await.unwrap();
    result.stdout.trim().to_string()
}

#[cfg(not(tarpaulin_include))]
pub async fn level3_password() -> String {
    let client = get_client_from_settings("bandit", 2).await;
    let result = client
        .execute("cat \"./spaces in this filename\"")
        .await
        .unwrap();
    result.stdout.trim().to_string()
}

#[cfg(not(tarpaulin_include))]
pub async fn level4_password() -> String {
    let client = get_client_from_settings("bandit", 3).await;
    let result = client.execute("cat \"./inhere/.hidden\"").await.unwrap();
    result.stdout.trim().to_string()
}

#[cfg(not(tarpaulin_include))]
pub async fn level5_password() -> String {
    let client = get_client_from_settings("bandit", 4).await;
    let result = client
        .execute(
            "find ./inhere -type f -exec file {} + | awk -F: '/ ASCII text/{print $1}' | xargs cat",
        )
        .await
        .unwrap();
    result.stdout.trim().to_string()
}

pub async fn level6_password() -> String {
    let client = get_client_from_settings("bandit", 5).await;
    let result = client
        .execute("find ./inhere -type f -size 1033c ! -perm /0111 | xargs cat")
        .await
        .unwrap();
    result.stdout.trim().to_string()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn level1_password_returns_proper_value() {
        let client =
            get_client_from_settings_with_password("bandit", 1, level1_password().await).await;
        let result = client.execute("echo hello").await.unwrap();
        assert_eq!("hello\n", result.stdout);
        assert_eq!(0, result.exit_status);
    }

    #[tokio::test]
    async fn level2_password_returns_proper_value() {
        let client =
            get_client_from_settings_with_password("bandit", 2, level2_password().await).await;
        let result = client.execute("echo hello").await.unwrap();
        assert_eq!("hello\n", result.stdout);
        assert_eq!(0, result.exit_status);
    }

    #[tokio::test]
    async fn level3_password_returns_proper_value() {
        let client =
            get_client_from_settings_with_password("bandit", 3, level3_password().await).await;
        let result = client.execute("echo hello").await.unwrap();
        assert_eq!("hello\n", result.stdout);
        assert_eq!(0, result.exit_status);
    }

    #[tokio::test]
    async fn level4_password_returns_proper_value() {
        let client =
            get_client_from_settings_with_password("bandit", 4, level4_password().await).await;
        let result = client.execute("echo hello").await.unwrap();
        assert_eq!("hello\n", result.stdout);
        assert_eq!(0, result.exit_status);
    }

    #[tokio::test]
    async fn level5_password_returns_proper_value() {
        let client =
            get_client_from_settings_with_password("bandit", 5, level5_password().await).await;
        let result = client.execute("echo hello").await.unwrap();
        assert_eq!("hello\n", result.stdout);
        assert_eq!(0, result.exit_status);
    }

    #[tokio::test]
    async fn level6_password_returns_proper_value() {
        let client =
            get_client_from_settings_with_password("bandit", 6, level6_password().await).await;
        let result = client.execute("echo hello").await.unwrap();
        assert_eq!("hello\n", result.stdout);
        assert_eq!(0, result.exit_status);
    }
}
