# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0-alpha.4](https://github.com/curlz-rs/curlz/compare/v0.1.0-alpha.3...v0.1.0-alpha.4) - 2023-01-16

### Added
- *(ci)* migrate to `release-plz`
- *(http-language)* basics of the http language file format (#14)
- *(functions)* implement `jwt` template function (#8)
- *(essentials)* implement interactive prompt with a label (#3)
- *(funding)* add the github sponsoring button
- *(essentials)* `prompt_password()` special placeholder (#1)
- *(essentials)* completing example 1 in the readme
- *(essentials)* little refactoring
- *(essentials)* switch template language to minijinja
- *(essentials)* some more progress on basics
- *(doc)* update the readme on features
- *(essentials)* add first essential features
- *(command:bookmark-as)* introduce insta testing
- *(command:bookmark-as)* implement first `BookmarkAsCommand`
- *(ci)* first build pipeline
- remove dimensions from gif
- add demo.gif
- add first version of README.md
- add first version of Cargo.toml

### Fixed
- *(#10)* RUSTSEC-2020-0071: avoid full time featured time dependency (#11)
- *(ci)* disable brew deployment for now
- *(doc)* fix badges and repo links

### Other
- *(ci)* release-please use the patch version bump strategy
- *(ci)* release-please use the prerelease flag
- *(ci)* fix release-please token variable
- *(ci)* fix release-please add debug flag
- *(ci)* fix release-please token issue again
- *(ci)* fix release-please token issue
- *(ci)* add release-please workflow
- 0.1.0-alpha.3
- add docs for placeholders at the beginning of urls (#9)
- fix readme formatting issue
- *(v0.1.0-alpha.2)* CHANGELOG + version bump + cargo update (#4)
- `v0.1.0-alpha.1` (#2)
- *(docs)* fix typos
- *(doc)* fix cargo doc lint
- *(fmt)* reformat
- *(deps)* cargo update some deps
- add todos for the next iteration
- Initial commit
- Initial commit
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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