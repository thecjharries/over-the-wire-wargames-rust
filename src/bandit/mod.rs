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

macro_rules! bandit_single_command_level {
    ($level:literal, $command:literal) => {
        ssh_single_command_level!("bandit", $level, $command);
    };
}

bandit_single_command_level!(1, "cat readme");
bandit_single_command_level!(2, "cat ./-");
bandit_single_command_level!(3, "cat \"./spaces in this filename\"");
bandit_single_command_level!(4, "cat \"./inhere/.hidden\"");
bandit_single_command_level!(
    5,
    "find ./inhere -type f -exec file {} + | awk -F: '/ ASCII text/{print $1}' | xargs cat"
);
bandit_single_command_level!(
    6,
    "find ./inhere -type f -size 1033c ! -perm /0111 | xargs cat"
);
bandit_single_command_level!(
    7,
    "find / -size 33c -group bandit6 -user bandit7 -print 2>/dev/null | xargs cat"
);
bandit_single_command_level!(8, "grep 'millionth' ./data.txt | awk '{print $2}'");
bandit_single_command_level!(9, "cat data.txt | sort | uniq -u");
bandit_single_command_level!(
    10,
    "strings data.txt | awk '/==+/{print $2;}' | tac | head -n 1"
);
bandit_single_command_level!(11, "cat data.txt | base64 --decode | awk '{print $4}'");
bandit_single_command_level!(
    12,
    "cat data.txt | tr 'A-Za-z' 'N-ZA-Mn-za-m' | awk '{print $4}'"
);
bandit_single_command_level!(
    13,
    "cat data.txt |
        xxd -r |
        gzip --decompress |
        bzip2 --decompress |
        gzip --decompress |
        tar --extract --to-stdout |
        tar --extract --to-stdout |
        bzip2 --decompress |
        tar --extract --to-stdout |
        gzip --decompress |
        awk '{print $4}'"
);
bandit_single_command_level!(14, "ssh -o StrictHostKeyChecking=no -i sshkey.private bandit14@localhost -p 2220 'cat /etc/bandit_pass/bandit14' 2>/dev/null");

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use crate::macros::test_ssh_level;

    #[allow(unused_macros)]
    macro_rules! test_bandit_level {
        ($level:literal) => {
            test_ssh_level!("bandit", $level);
        };
    }

    test_bandit_level!(14);
    // test_bandit_level!(13);
    // test_bandit_level!(12);
    // test_bandit_level!(11);
    // test_bandit_level!(10);
    // test_bandit_level!(9);
    // test_bandit_level!(8);
    // test_bandit_level!(7);
    // test_bandit_level!(6);
    // test_bandit_level!(5);
    // test_bandit_level!(4);
    // test_bandit_level!(3);
    // test_bandit_level!(2);
    // test_bandit_level!(1);
}
