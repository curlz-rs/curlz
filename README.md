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

- variables from `.env` and `.yaml` environment files
- ️placeholder evaluation using the [minijinja](https://docs.rs/minijinja/latest/minijinja/) template engine, which can be used in URLs, HTTP headers, the HTTP body, and other passed curl parameters
- ability to save and execute requests as bookmarks with a shortname
- support any curl argument after a `--`, that makes a drop-in-replacement for curl
- special placeholders to interact with the user
  - prompt for a password as `{{ prompt_password() }}` 
  `curlz r https://api.github.com/user -- -u "{{ username }}:{{ prompt_password() }}"`
  - prompt for interactive input with a label as `{{ prompt_for("Username") }}` or `{{ prompt_for("Birthdate") }}`
  `curlz -- -u "{{ prompt_for("Username") }}:{{ prompt_password() }}" https://api.github.com/user`
- ️evaluate placeholders at the beginning of an url like:
`curlz r --define 'host=https://httpbin.org' '{{host}}/get'`
- ️special placeholder for developers, like for Json Web Tokens (JWT)
`{{ jwt(claims, signing_key) }}`, where `claims` and `signing_key` are looked up at the environment file or can be directly provided map and string
`curlz r -H 'Authorization: Bearer {{ jwt({"uid": "1234"}, "000") }}' https://httpbin.org/bearer -- -vvv`
- send a http body via `-d | --data` 
`curlz r -d 'Hello World' -X POST https://httpbin.org/anything`
- send a json payload and headers with the `--json` argument
`curlz r --json '{ "foo": "bar" }' -X POST 'https://httpbin.org/anything'`

## WIP
- [⏳] support rest client template language [see #5](https://github.com/curlz-rs/curlz/issues/5)
       [check out the examples folder for more infos](./examples/http-file)

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

## Template function documentation

please read the book to learn more about the template functions
