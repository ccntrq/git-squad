# git-squad
## Manage co-authors in git commit messages with ease

`git-squad` is a command-line tool that makes it easy to manage co-authors in
your Git commit messages. Perfect for pair programming or collaborative coding
sessions, it lets you quickly add or remove co-authors without having to
manually edit commit templates.

## Features

- Maintain a list of collaborators (buddies) with their names and emails
- Add and remove co-authors in the current git session
- Automatically updates your git commit template
- Simple command-line interface
- Shell completions for commands and buddies

## Usage 

```
Manage co-authors in git commit messages with ease

Usage: git-squad [OPTIONS] [COMMAND]

Commands:
  with         Add buddies to the current session
  without      Remove buddies from the current session
  alone        Remove all buddies from the current session
  create       Create a new buddy
  forget       Delete a buddy from the list of available buddies
  info         List both active and available buddies
  list         List all available buddies
  active       List active buddies in the current session
  completions  Generate completions for your shell
  help         Print this message or the help of the given subcommand(s)

Options:
      --buddies-file <BUDDIES_FILE>  Use a custom buddy file instead of ~/.config/git-squad/buddies.yaml
  -h, --help                         Print help
```

## Installation

### Prerequisites

- Git must be installed and configured
- Rust and Cargo must be installed

### Install published version from crates.io

```bash
cargo install git-squad
```

### Building the latest development version

1. Clone the repository:

```bash
git clone git@github.com:ccntrq/git-squad.git
cd git-squad
```

2. Build and install with Cargo:

```bash
cargo install --path .
```


## Setup

Before using `git-squad`, you need to set up a git commit template. You can
either setup a global one or do this per repo.

### Global

```bash
# Create a template file
touch ~/.gitmessage

# Configure git to use it
git config --global commit.template ~/.gitmessage
```

### Per repo

```bash
# Inside your repo dir
# Create a template file
touch .git/gitmessage

# Configure git to use it
git config  commit.template .git/gitmessage
```

# Related work

There is a similar tool written in typescript called
[git-mob](https://github.com/rkotze/git-mob) which I discovered while name
hunting for this tool.
