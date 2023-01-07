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

- ☑️.env files
- ☑️.yaml env files
- ☑️placeholder evaluation, with the [minijinja](https://docs.rs/minijinja/latest/minijinja/) template engine
  - in urls
  - in http headers (`-H | --header` arguments)
  - in every other passed curl parameter
- ☑️save request as a bookmark, containing
  - curl arguments
  - http headers
  - http method
  - placeholders
- ☑️pass all arguments after `--` to curl, that makes drop-in-replacement possible
- ☑️execute a bookmarked request
- ☑️special placeholder variables that would interact with the user
  - ☑️prompt for a password as `{{ prompt_password() }}` 
    `curlz r https://api.github.com/user -- -u "{{ username }}:{{ prompt_password() }}"`
  - ☑️prompt for interactive input with a label as `{{ prompt_for("Username") }}` or `{{ prompt_for("Birthdate") }}`
    `curlz -- -u "{{ prompt_for("Username") }}:{{ prompt_password() }}" https://api.github.com/user`
- ☑️evaluate placeholders at the beginning of an url like:
  ```sh
  curlz r --define 'host=https://httpbin.org' '{{host}}/get?show_env={{ prompt_for("show_env") }}'
  ```
- ☑️special placeholder for developers, like for Json Web Tokens (JWT)
  - example: `{{ jwt(claims, signing_key) }}`, where `claims` and `signing_key` are looked up at the environment file or can be directly provided map and string
    ```sh
    curlz r -H 'Authorization: Bearer {{ jwt({"uid": "1234"}, "000") }}' https://httpbin.org/bearer -- -vvv
    ```
- [ ]: send json payload and json headers with the `--json` argument
  - example: 
    ```sh
    curlz r --json -d '{ "foo": "bar" }' -X POST 'https://httpbin.org/anything'`
    ```

## TODOs
- [ ] support rest client template language [see #5](https://github.com/curlz-rs/curlz/issues/5)

## Example #1

In this example we're going to download a pre-configured `.gitignore` for a given language from GitHub via curl

- `curl https://api.github.com/gitignore/templates/Rust`
- the same with curlz: `curlz r https://api.github.com/gitignore/templates/Rust`
- Add a placeholder that is interactively requested 
  `curlz r 'https://api.github.com/gitignore/templates/{{ prompt_for("Language") | title }}'`
- Now let's bookmark this request:
  ```sh
  curlz r --bookmark 'https://api.github.com/gitignore/templates/{{ prompt_for("Language") | title }}'
  Language: rust
  Please enter a bookmark name: gitignore
  ```
- Finally, we can keep using the bookmark from now on: `curlz r gitignore`

## Template functions

### Json Web Token - `jwt(claims: map, [signing_key: string])`
- arguments:
  - `claims`: to be a map of key value pairs like `{"uid": "1234"}` that are the payload of the JWT
  - `signing_key`: to be a string, this is optional and can be provided at the environment file with a variable named `jwt_signing_key`
- output: string is a Json Web Token (JWT)
- notes: 
  - the hash algorithm is `HS256` and the JWT header is `{"alg": "HS256", "typ": "JWT"}`
  - the claim `exp` expiry time is set to in 15min by default, but can be overwritten
  - the claim `iat` issued at timestamp is set automatically and cannot be overwritten
