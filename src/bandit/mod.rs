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

use crate::get_ssh_client_from_settings;

#[cfg(not(tarpaulin_include))]
pub async fn level1_password() -> String {
    let client = get_ssh_client_from_settings("bandit", 0).await;
    let result = client.execute("cat readme").await.unwrap();
    result.stdout.trim().to_string()
}

#[cfg(not(tarpaulin_include))]
pub async fn level2_password() -> String {
    let client = get_ssh_client_from_settings("bandit", 1).await;
    let result = client.execute("cat ./-").await.unwrap();
    result.stdout.trim().to_string()
}

#[cfg(not(tarpaulin_include))]
pub async fn level3_password() -> String {
    let client = get_ssh_client_from_settings("bandit", 2).await;
    let result = client
        .execute("cat \"./spaces in this filename\"")
        .await
        .unwrap();
    result.stdout.trim().to_string()
}

#[cfg(not(tarpaulin_include))]
pub async fn level4_password() -> String {
    let client = get_ssh_client_from_settings("bandit", 3).await;
    let result = client.execute("cat \"./inhere/.hidden\"").await.unwrap();
    result.stdout.trim().to_string()
}

#[cfg(not(tarpaulin_include))]
pub async fn level5_password() -> String {
    let client = get_ssh_client_from_settings("bandit", 4).await;
    let result = client
        .execute(
            "find ./inhere -type f -exec file {} + | awk -F: '/ ASCII text/{print $1}' | xargs cat",
        )
        .await
        .unwrap();
    result.stdout.trim().to_string()
}

#[cfg(not(tarpaulin_include))]
pub async fn level6_password() -> String {
    let client = get_ssh_client_from_settings("bandit", 5).await;
    let result = client
        .execute("find ./inhere -type f -size 1033c ! -perm /0111 | xargs cat")
        .await
        .unwrap();
    result.stdout.trim().to_string()
}

#[cfg(not(tarpaulin_include))]
pub async fn level7_password() -> String {
    let client = get_ssh_client_from_settings("bandit", 6).await;
    let result = client
        .execute("find / -size 33c -group bandit6 -user bandit7 -print 2>/dev/null | xargs cat")
        .await
        .unwrap();
    result.stdout.trim().to_string()
}

#[cfg(not(tarpaulin_include))]
pub async fn level8_password() -> String {
    let client = get_ssh_client_from_settings("bandit", 7).await;
    let result = client
        .execute("grep 'millionth' ./data.txt | awk '{print $2}'")
        .await
        .unwrap();
    result.stdout.trim().to_string()
}

pub async fn level9_password() -> String {
    todo!()
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use crate::get_ssh_client_from_settings_with_password;

    #[tokio::test]
    async fn level8_password_returns_proper_value() {
        let client =
            get_ssh_client_from_settings_with_password("bandit", 9, level9_password().await).await;
        let result = client.execute("echo hello").await.unwrap();
        assert_eq!("hello\n", result.stdout);
        assert_eq!(0, result.exit_status);
    }

    // #[tokio::test]
    // async fn level8_password_returns_proper_value() {
    //     let client =
    //         get_ssh_client_from_settings_with_password("bandit", 8, level8_password().await).await;
    //     let result = client.execute("echo hello").await.unwrap();
    //     assert_eq!("hello\n", result.stdout);
    //     assert_eq!(0, result.exit_status);
    // }

    // #[tokio::test]
    // async fn level7_password_returns_proper_value() {
    //     let client =
    //         get_ssh_client_from_settings_with_password("bandit", 7, level7_password().await).await;
    //     let result = client.execute("echo hello").await.unwrap();
    //     assert_eq!("hello\n", result.stdout);
    //     assert_eq!(0, result.exit_status);
    // }

    // #[tokio::test]
    // async fn level6_password_returns_proper_value() {
    //     let client =
    //         get_ssh_client_from_settings_with_password("bandit", 6, level6_password().await).await;
    //     let result = client.execute("echo hello").await.unwrap();
    //     assert_eq!("hello\n", result.stdout);
    //     assert_eq!(0, result.exit_status);
    // }

    // #[tokio::test]
    // async fn level5_password_returns_proper_value() {
    //     let client =
    //         get_ssh_client_from_settings_with_password("bandit", 5, level5_password().await).await;
    //     let result = client.execute("echo hello").await.unwrap();
    //     assert_eq!("hello\n", result.stdout);
    //     assert_eq!(0, result.exit_status);
    // }

    // #[tokio::test]
    // async fn level4_password_returns_proper_value() {
    //     let client =
    //         get_ssh_client_from_settings_with_password("bandit", 4, level4_password().await).await;
    //     let result = client.execute("echo hello").await.unwrap();
    //     assert_eq!("hello\n", result.stdout);
    //     assert_eq!(0, result.exit_status);
    // }

    // #[tokio::test]
    // async fn level3_password_returns_proper_value() {
    //     let client =
    //         get_ssh_client_from_settings_with_password("bandit", 3, level3_password().await).await;
    //     let result = client.execute("echo hello").await.unwrap();
    //     assert_eq!("hello\n", result.stdout);
    //     assert_eq!(0, result.exit_status);
    // }

    // #[tokio::test]
    // async fn level2_password_returns_proper_value() {
    //     let client =
    //         get_ssh_client_from_settings_with_password("bandit", 2, level2_password().await).await;
    //     let result = client.execute("echo hello").await.unwrap();
    //     assert_eq!("hello\n", result.stdout);
    //     assert_eq!(0, result.exit_status);
    // }

    // #[tokio::test]
    // async fn level1_password_returns_proper_value() {
    //     let client =
    //         get_ssh_client_from_settings_with_password("bandit", 1, level1_password().await).await;
    //     let result = client.execute("echo hello").await.unwrap();
    //     assert_eq!("hello\n", result.stdout);
    //     assert_eq!(0, result.exit_status);
    // }
}
