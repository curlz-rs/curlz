<div align="center">
 <img src="https://github.com/sassman/curlx-rs/blob/main/resources/demo.gif?raw=true">
 <h1><strong>curlx</strong></h1>

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Build Status](https://github.com/curlx-rs/curlx/workflows/Build/badge.svg)](https://github.com/curlx-rs/curlx/actions?query=branch%3Amain+workflow%3ABuild+)
[![crates.io](https://img.shields.io/crates/v/curlx.svg)](https://crates.io/crates/curlx)
[![dependency status](https://deps.rs/repo/github/sassman/curlx-rs/status.svg)](https://deps.rs/repo/github/curlx-rs/curlx)

</div>

> curl wrapper with placeholder, bookmark and environment powers just like postman
 
## Features

- ☑️ .env files
- ☑️ .yaml env files
- ☑️ placeholders evaluation, via liquid templates
  - in urls
  - in http headers (`-H | --header` arguments)
  - in all other passed curl parameters
- ☑️ save request as bookmark, containing
  - curl arguments
  - http headers
  - http method
  - placeholders
- ☑️ pass all arguments after `-- ` to curl, that makes drop-in-replacement possible

## TODOs
- [] execute a bookmarked request
- [] evaluate placeholders at the beginning of an url
- [] special placeholder variables like `mfa_token` etc. that would interact on usage with the user
- [] test other template engines