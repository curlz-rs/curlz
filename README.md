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
- [] special placeholder variables that would interact with the user
  - ☑️ example: prompting for a password `{{ prompt_password() }}
    ```
    curlz -- -u "{{ username }}:{{ prompt_password() }}" https://api.github.com/user
    ```

## TODOs
- [] evaluate placeholders at the beginning of an url
- [] special placeholder variables that would interact with the user
  - example:  `{{ jwt_token(signin_key, signin_secret) }}`
    ```
    curlz -H "Authorization: Bearer {{ mfa_token }}" -X POST https://api.github.com/user/repos -d '{ "name": "{{ repo_name }}" }'
    ```

## Example #1

In this example we're going to download a pre-configured `.gitignore` for a given language from GitHub via curl

- `curl https://api.github.com/gitignore/templates/Rust`
- the same with curlz: `curlz r https://api.github.com/gitignore/templates/Rust`
- parametrization: `curlz r 'https://api.github.com/gitignore/templates/{{ lang | title }}'`
- bookmarking:
  ```sh
  curlz r --define lang=Rust --bookmark 'https://api.github.com/gitignore/templates/{{ lang | title }}'
  Saving this request as a bookmark:
    Please enter a bookmark name: /gitignore
  Request bookmarked as: /gitignore`
  ```
- Finally, we can keep using the bookmark from now on: `curlz r /gitignore`