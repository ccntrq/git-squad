# Changelog

All notable changes to `git-squad` will be documented in this file.


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
