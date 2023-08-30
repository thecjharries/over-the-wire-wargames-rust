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

use crate::client::Session;
use crate::{get_level_password, load_settings};

#[cfg(not(tarpaulin_include))]
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

#[cfg(not(tarpaulin_include))]
pub async fn level2_password() -> String {
    let settings = load_settings("bandit");
    let host = settings.get_string("host").unwrap();
    let port = settings.get_string("port").unwrap();
    let user = "bandit1";
    let password = get_level_password(settings, 1);
    let mut session = Session::connect(&host, &port, &user, &password)
        .await
        .unwrap();
    let result = session.call("cat ./-").await.unwrap();
    session.close().await.unwrap();
    result.output().trim().to_string()
}

#[cfg(not(tarpaulin_include))]
pub async fn level3_password() -> String {
    let settings = load_settings("bandit");
    let host = settings.get_string("host").unwrap();
    let port = settings.get_string("port").unwrap();
    let user = "bandit2";
    let password = get_level_password(settings, 2);
    let mut session = Session::connect(&host, &port, &user, &password)
        .await
        .unwrap();
    let result = session
        .call("cat \"./spaces in this filename\"")
        .await
        .unwrap();
    session.close().await.unwrap();
    result.output().trim().to_string()
}

pub async fn level4_password() -> String {
    let settings = load_settings("bandit");
    let host = settings.get_string("host").unwrap();
    let port = settings.get_string("port").unwrap();
    let user = "bandit2";
    let password = get_level_password(settings, 2);
    let mut session = Session::connect(&host, &port, &user, &password)
        .await
        .unwrap();
    let result = session
        .call("cat \"./spaces in this filename\"")
        .await
        .unwrap();
    session.close().await.unwrap();
    result.output().trim().to_string()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;

    // #[tokio::test]
    // async fn level1_password_returns_proper_value() {
    //     let settings = load_settings("bandit");
    //     let host = settings.get_string("host").unwrap();
    //     let port = settings.get_string("port").unwrap();
    //     let user = "bandit1";
    //     let password = level1_password().await;
    //     let mut session = Session::connect(&host, &port, &user, &password)
    //         .await
    //         .unwrap();
    //     let result = session.call("echo hello").await.unwrap();
    //     assert_eq!("hello\n", result.output());
    //     assert!(result.success());
    //     session.close().await.unwrap();
    // }

    // #[tokio::test]
    // async fn level2_password_returns_proper_value() {
    //     let settings = load_settings("bandit");
    //     let host = settings.get_string("host").unwrap();
    //     let port = settings.get_string("port").unwrap();
    //     let user = "bandit2";
    //     let password = level2_password().await;
    //     let mut session = Session::connect(&host, &port, &user, &password)
    //         .await
    //         .unwrap();
    //     let result = session.call("echo hello").await.unwrap();
    //     assert_eq!("hello\n", result.output());
    //     assert!(result.success());
    //     session.close().await.unwrap();
    // }

    // #[tokio::test]
    // async fn level3_password_returns_proper_value() {
    //     let settings = load_settings("bandit");
    //     let host = settings.get_string("host").unwrap();
    //     let port = settings.get_string("port").unwrap();
    //     let user = "bandit3";
    //     let password = level3_password().await;
    //     let mut session = Session::connect(&host, &port, &user, &password)
    //         .await
    //         .unwrap();
    //     let result = session.call("echo hello").await.unwrap();
    //     assert_eq!("hello\n", result.output());
    //     assert!(result.success());
    //     session.close().await.unwrap();
    // }

    #[tokio::test]
    async fn level4_password_returns_proper_value() {
        let settings = load_settings("bandit");
        let host = settings.get_string("host").unwrap();
        let port = settings.get_string("port").unwrap();
        let user = "bandit4";
        let password = level4_password().await;
        let mut session = Session::connect(&host, &port, &user, &password)
            .await
            .unwrap();
        let result = session.call("echo hello").await.unwrap();
        assert_eq!("hello\n", result.output());
        assert!(result.success());
        session.close().await.unwrap();
    }
}
