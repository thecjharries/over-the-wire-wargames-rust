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

use paste::paste;

use crate::macros::ssh_single_command_level;

macro_rules! leviathan_single_command_level {
    ($level:literal, $command:literal) => {
        ssh_single_command_level!("leviathan", $level, $command);
    };
}

leviathan_single_command_level!(1, "todo");

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use crate::macros::test_ssh_level;

    #[allow(unused_macros)]
    macro_rules! test_leviathan_level {
        ($level:literal) => {
            test_ssh_level!("leviathan", $level);
        };
    }

    test_leviathan_level!(1);
}
