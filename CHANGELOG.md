# Changelog

All notable changes to `git-squad` will be documented in this file.


## [0.3.3](https://github.com/ccntrq/git-squad/compare/v0.3.2...v0.3.3) - 2025-05-22

### ⛰️ Features

- add multiselect menu for `with` and `without` command ([#35](https://github.com/ccntrq/git-squad/issues/35))
- reimplement integration test suite ([#38](https://github.com/ccntrq/git-squad/issues/38))
- improved prompts for create command ([#39](https://github.com/ccntrq/git-squad/issues/39))

### 🐛 Bug Fixes

- place Co-Authors in footer section ([#23](https://github.com/ccntrq/git-squad/issues/23))

## [0.3.2](https://github.com/ccntrq/git-squad/compare/v0.3.1...v0.3.2) - 2025-04-17

### 📚 Documentation

- really make demo tape show on crates.io now ([#21](https://github.com/ccntrq/git-squad/issues/21))

## [0.3.1](https://github.com/ccntrq/git-squad/compare/v0.3.0...v0.3.1) - 2025-04-17

### 📚 Documentation

- make demo tape gif show on crates.io ([#20](https://github.com/ccntrq/git-squad/issues/20))

## [0.3.0](https://github.com/ccntrq/git-squad/compare/v0.2.0...v0.3.0) - 2025-04-15

### ⛰️ Features

- allow passing multiple buddies to `with` and `without` ([#1](https://github.com/ccntrq/git-squad/issues/1))
- add autocomplete for buddies aliases ([#2](https://github.com/ccntrq/git-squad/issues/2))
- [**breaking**] new toml format for buddies file ([#17](https://github.com/ccntrq/git-squad/issues/17))
  **BREAKING CHANGE**: `yaml` buddies configs have been deprecated in favor of
  `toml`. For users using the default config locations the old config will
  be automatically migrated to the new format. For users using custom
  `--buddies-file` locations there is a new command `git-squad
  migrate-buddies` that can be used to perform the migration

### 🐛 Bug Fixes

- always place co-authors into the footer section ([#8](https://github.com/ccntrq/git-squad/issues/8))
- make create command fail early on duplicate alias ([#9](https://github.com/ccntrq/git-squad/issues/9))

### 📚 Documentation

- add demo tape ([#10](https://github.com/ccntrq/git-squad/issues/10))

## 0.2.0 -- 2025-04-04

### Features

- add `completions` subcommand that generates shell completions

## 0.1.0 -- 2025-04-04

First version. Released on an unsuspecting world.

### Features

- Maintain a list of collaborators (buddies) with their names and emails
- Add and remove co-authors in the current git session
- Automatically updates your git commit template
- Simple command-line interface
