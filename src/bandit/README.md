# Bandit

This module covers [the Bandit wargame](https://overthewire.org/wargames/bandit/).

## Setup

You'll need to expose several environment variables. I recommend using [direnv](https://direnv.net/) to have hidden local files. While all of these "secrets" are exposed in the wargame, I'm trying to follow decent security practice and keep "secrets" out of the repository.

```bash
$ cat .envrc
#!/usr/bin/env bash

# General
export SSH_HOST=...
export SSH_PORT=...
export SSH_USER=...
export SSH_PASS=...
```
