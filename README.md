<div align="center">
 <img src="https://github.com/curlz-rs/curlz/blob/main/resources/demo.gif?raw=true">
 <h1><strong>curlz</strong></h1>

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Build Status](https://github.com/curlz-rs/curlz/workflows/Build/badge.svg)](https://github.com/curlz-rs/curlz/actions?query=branch%3Amain+workflow%3ABuild+)
[![crates.io](https://img.shields.io/crates/v/curlz.svg)](https://crates.io/crates/curlz)
[![dependency status](https://deps.rs/repo/github/curlz-rs/curlz/status.svg)](https://deps.rs/repo/github/curlz-rs/curlz)

</div>

> a curl wrapper with placeholder, bookmark and environment powers just like postman but for the terminal
 
## Features

- ☑️ .env files
- ☑️ .yaml env files
- ☑️ placeholder evaluation, with the [minijinja](https://docs.rs/minijinja/latest/minijinja/) template engine
  - in urls
  - in http headers (`-H | --header` arguments)
  - in every other passed curl parameter
- ☑️ save request as a bookmark, containing
  - curl arguments
  - http headers
  - http method
  - placeholders
- ☑️ pass all arguments after `--` to curl, that makes drop-in-replacement possible
- ☑️ execute a bookmarked request
- ☑️ special placeholder variables that would interact with the user
  - ☑️ prompt for a password as `{{ prompt_password() }}
    `curlz r https://api.github.com/user -- -u "{{ username }}:{{ prompt_password() }}"`
  - ☑️ prompt for interactive input with a label as `{{ prompt_for("Username") }}` or `{{ prompt_for("Birthdate") }}`
    `curlz -- -u "{{ prompt_for("Username") }}:{{ prompt_password() }}" https://api.github.com/user`

## TODOs
- [] evaluate placeholders at the beginning of an url
- [] special placeholder for developers, like `jwt_token` or `mfa_token` 
  - example:  `{{ jwt_token(signin_key, signin_secret) }}`, where `signin_key` and `signin_secret` are first looked up 
    at the environment file as variable or else taken then as given.
    `curlz -H "Authorization: Bearer {{ jwt_token(signin_key, signin_secret) }}" -X POST https://api.github.com/user/repos -d '{ "name": "{{ repo_name }}" }'`

## Example #1

In this example we're going to download a pre-configured `.gitignore` for a given language from GitHub via curl

- `curl https://api.github.com/gitignore/templates/Rust`
- the same with curlz: `curlz r https://api.github.com/gitignore/templates/Rust`
- Add a placeholder that is interactively requested 
  `curlz r 'https://api.github.com/gitignore/templates/{{ prompt_for("Language") | title }}'`
- Now lets bookmark this request:
  ```sh
  curlz r --bookmark 'https://api.github.com/gitignore/templates/{{ prompt_for("Language") | title }}'
  Language: rust
  Please enter a bookmark name: gitignore
  ```
- Finally, we can keep using the bookmark from now on
  `curlz r gitignore`
