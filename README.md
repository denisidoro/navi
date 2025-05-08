# navi <img src="https://raw.githubusercontent.com/denisidoro/navi/master/assets/icon.png" alt="icon" height="28px"/> [![Actions Status](https://github.com/denisidoro/navi/workflows/CI/badge.svg)](https://github.com/denisidoro/navi/actions) ![GitHub release](https://img.shields.io/github/v/release/denisidoro/navi?include_prereleases)

An interactive cheatsheet tool for the command-line.

[![Demo](https://asciinema.org/a/406461.svg)](https://asciinema.org/a/406461)

**navi** allows you to browse through cheatsheets (that you may write yourself or download from maintainers) and execute commands. Suggested values for arguments are dynamically displayed in a list.

## Pros

- it will spare you from knowing CLIs by heart
- it will spare you from copy-pasting output from intermediate commands
- it will make you type less
- it will teach you new one-liners

It uses [fzf](https://github.com/junegunn/fzf) or [skim](https://github.com/lotabout/skim) under the hood and it can be either used as a command or as a shell widget (_à la_ Ctrl-R).

## Table of contents

- [Installation](#installation)
- [Usage](#usage)
- [Cheatsheet repositories](#cheatsheet-repositories)
- [Cheatsheet syntax](#cheatsheet-syntax)
- [Customization](#customization)
- [More info](#more-info)
- [Trying out online](#trying-out-online)
- [Similar tools](#similar-tools)
- [Etymology](#etymology)

## Installation

The recommended way to install **navi** is by running:

```sh
brew install navi
```

> [!NOTE]
> For more details on how to install Navi, see [docs/installation](docs/installation/README.md)

**navi** can be installed with the following package managers:

[![Packaging status](https://repology.org/badge/vertical-allrepos/navi.svg)](https://repology.org/project/navi/versions)

## Usage

There are multiple ways to use **navi**:

- by typing `navi` in the terminal
  - pros: you have access to all possible subcommands and flags
- as a [shell widget](docs/installation/README.md#installing-the-shell-widget) for the terminal
  - pros: the shell history is correctly populated (i.e. with the actual command you ran instead of `navi`) and you can edit the command as you wish before executing it
- as a [Tmux widget](docs/widgets/howto/TMUX.md)
  - pros: you can use your cheatsheets in any command-line app even in SSH sessions
- as [aliases](docs/cheatsheet/syntax/README.md#aliases)
- as a [shell scripting tool](docs/usage/shell-scripting/README.md)

In particular, check [these instructions](https://github.com/denisidoro/navi/issues/491) if you want to replicate what's shown in the demo above.

## Cheatsheet repositories

Running **navi** for the first time will help you download and manage cheatsheets. By default, they are stored at `~/.local/share/navi/cheats/`.

You can also:

- [browse through featured cheatsheets](docs/usage/commands/repo/README.md#browsing-through-cheatsheet-repositorieea)
- [import cheatsheets from git repositories](docs/cheatsheet/repositories/README.md#importing-cheatsheet-repositories)
- [write your own cheatsheets](#cheatsheet-syntax) (and [share them](docs/cheatsheet/repositories/README.md#submitting-cheatsheets), if you want)
- [use cheatsheets from other tools](docs/cheatsheet/README.md#using-cheatsheets-from-other-tools), such as [tldr](https://github.com/tldr-pages/tldr) and [cheat.sh](https://github.com/chubin/cheat.sh)
- [auto-update repositories](docs/cheatsheet/repositories/README.md#auto-updating-repositories)
- auto-export cheatsheets from your [TiddlyWiki](https://tiddlywiki.com/) notes using a [TiddlyWiki plugin](https://bimlas.github.io/tw5-navi-cheatsheet/)

## Cheatsheet syntax

Cheatsheets are described in `.cheat` files that look like this:

```sh
% git, code

# Change branch
git checkout <branch>

$ branch: git branch | awk '{print $NF}'
```

The full syntax and examples can be found [here](docs/cheatsheet/syntax/README.md).

## Customization

You can:

- [setup your own config file](docs/configuration/README.md)
- [set custom paths for your config file and cheat sheets](docs/configuration/README.md#paths-and-environment-variables)
- [change colors](docs/configuration/README.md#changing-colors)
- [resize columns](docs/configuration/README.md#resizing-columns)
- [change how search is performed](docs/configuration/README.md#overriding-fzf-options)

## More info

Please run the following command to read more about all possible options:

```sh
navi --help
```

In addition, please check the [/docs](docs) folder or the website.
