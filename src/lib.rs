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

use config::Config;

pub mod bandit;

pub fn load_settings(wargame: &str) -> Config {
    Config::builder()
        .add_source(config::File::with_name(
            format!("settings/{}.yaml", wargame).as_str(),
        ))
        .build()
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
}
