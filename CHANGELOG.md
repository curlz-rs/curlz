# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0-alpha.3](https://github.com/curlz-rs/curlz/compare/v0.1.0-alpha.3...v0.2.0-alpha.3) (2023-01-08)


### Features

* **http-language:** basics of the http language file format ([#14](https://github.com/curlz-rs/curlz/issues/14)) ([5f49b45](https://github.com/curlz-rs/curlz/commit/5f49b4548e624f4a2f555fbb9d83899e74307e22))

## [Unreleased] - 2022-01-01
[Unreleased]: https://github.com/curlz-rs/curlz/compare/v0.1.0-alpha.1...HEAD

### Added
- special template function `jwt` for json web tokens [see #6](https://github.com/curlz-rs/curlz/issues/6)

### Changed
### Deprecated 
### Removed
### Fixed
### Security
### Contributors
- [@sassman](https://github.com/sassman)

## [0.1.0-alpha.2] - 2022-08-16
[0.1.0-alpha.2]: https://github.com/curlz-rs/curlz/compare/v0.1.0-alpha.1..v0.1.0-alpha.2

### Added

- special placeholder variables that would interact with the user
  - prompt for interactive input with a label as for example `{{ prompt_for("Username") }}` or `{{ prompt_for("Birthdate") }}`
    `curlz -- -u "{{ prompt_for("Username") }}:{{ prompt_password() }}" https://api.github.com/user`

### Contributors
[@sassman](https://github.com/sassman)

## [0.1.0-alpha.1] - 2022-08-07
[0.1.0-alpha.1]: https://github.com/curlz-rs/curlz/compare/v0.1.0-alpha.1

### Added
- reading of `.env` files
- reading of `.yaml` env files
- placeholder evaluation, with the [minijinja](https://docs.rs/minijinja/latest/minijinja/) template engine
  - in urls
  - in http headers (`-H | --header` arguments)
  - in every other passed curl parameter
- save request as a bookmark via `--bookmark` or `--bookmark-as`, containing:
  - curl arguments
  - http headers
  - http method
  - placeholders
- pass all arguments after `--` to curl, that makes drop-in-replacement possible
- execute a bookmarked request
- special placeholder variables that would interact with the user
  - prompting for a password as `{{ prompt_password() }}
    ```sh
    curlz -- -u "{{ username }}:{{ prompt_password() }}" https://api.github.com/user
    ```

### Contributors
[@sassman](https://github.com/sassman)
