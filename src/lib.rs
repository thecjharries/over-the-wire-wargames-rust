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

use async_ssh2_tokio::client::{AuthMethod, Client, ServerCheckMethod};
use config::Config;

pub mod bandit;

/// Load the settings for a given wargame
///
/// # Arguments
/// * `wargame` - The name of the wargame to load settings for
///
/// # Returns
/// A Config object with the settings for the given wargame
///
/// # Panics
/// If the wargame does not exist
///
/// # Examples
/// ```
/// let settings = load_settings("bandit");
/// assert_eq!("bandit.labs.overthewire.org", settings.get_string("host").unwrap());
/// ```
pub fn load_settings(wargame: &str) -> Config {
    Config::builder()
        .add_source(config::File::with_name(
            format!("settings/{}.yaml", wargame).as_str(),
        ))
        .build()
        .unwrap()
}

/// Get the password for a given level of a wargame
///
/// # Arguments
/// * `config` - The Config object for the wargame
/// * `level` - The level to get the password for
///
/// # Returns
/// The password for the given level
///
/// # Panics
/// If the level does not exist
/// If the level does not have a password
///
/// # Examples
/// ```
/// let settings = load_settings("bandit");
/// assert_eq!("bandit0", get_level_password(settings, 0));
/// ```
pub fn get_level_password(config: Config, level: u8) -> String {
    config
        .get_array("passwords")
        .unwrap()
        .get(level as usize)
        .unwrap()
        .to_string()
}

/// Get an SSH client for a given wargame and level
///
/// Test coverage is not calculated because this wraps external code
///
/// # Arguments
/// * `wargame` - The name of the wargame for the client
/// * `level` - The level for the client
///
/// # Returns
/// An SSH client for the given wargame and level
///
/// # Examples
/// ```
/// let client = get_ssh_client_from_settings("bandit", 0).await;
/// let result = client.execute("echo hello").await.unwrap();
/// assert_eq!("hello\n", result.stdout);
/// assert_eq!(0, result.exit_status);
/// ```
#[cfg(not(tarpaulin_include))]
pub async fn get_ssh_client_from_settings(wargame: &str, level: u8) -> Client {
    get_ssh_client_from_settings_with_password(
        wargame,
        level,
        get_level_password(load_settings(wargame), level),
    )
    .await
}

/// Get an SSH client for a given wargame and level with a password
///
/// Test coverage is not calculated because this wraps external code
/// This is a separate function from get_ssh_client_from_settings so we can test the password
///
/// # Arguments
/// * `wargame` - The name of the wargame for the client
/// * `level` - The level for the client
/// * `password` - The password for the client
///
/// # Returns
/// An SSH client for the given wargame and level
///
/// # Examples
/// ```
/// let client = get_ssh_client_from_settings_with_password("bandit", 0, "bandit0".to_string()).await;
/// let result = client.execute("echo hello").await.unwrap();
/// assert_eq!("hello\n", result.stdout);
/// assert_eq!(0, result.exit_status);
/// ```
#[cfg(not(tarpaulin_include))]
pub async fn get_ssh_client_from_settings_with_password(
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

    #[test]
    fn load_settings_can_load_existing_wargame() {
        let settings = load_settings("bandit");
        assert_eq!(
            "bandit.labs.overthewire.org",
            settings.get_string("host").unwrap()
        );
    }

    #[test]
    #[should_panic]
    fn load_settings_panics_on_nonexistent_wargame() {
        load_settings("nonexistent");
    }

    #[test]
    fn get_level_password_returns_correct_password() {
        let settings = load_settings("bandit");
        assert_eq!("bandit0", get_level_password(settings, 0));
    }

    #[test]
    #[should_panic]
    fn get_level_password_panics_on_nonexistent_level() {
        let settings = load_settings("bandit");
        get_level_password(settings, 200);
    }
}
