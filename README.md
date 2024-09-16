# navi <img src="https://raw.githubusercontent.com/denisidoro/navi/master/assets/icon.png" alt="icon" height="28px"/> [![Actions Status](https://github.com/denisidoro/navi/workflows/CI/badge.svg)](https://github.com/denisidoro/navi/actions) ![GitHub release](https://img.shields.io/github/v/release/denisidoro/navi?include_prereleases)

An interactive cheatsheet tool for the command-line.

[![Demo](https://asciinema.org/a/406461.svg)](https://asciinema.org/a/406461)

**navi** allows you to browse through cheatsheets (that you may write yourself or download from maintainers) and execute commands. Suggested values for arguments are dynamically displayed in a list.

## Pros

- it will spare you from knowing CLIs by heart
- it will spare you from copy-pasting output from intermediate commands
- it will make you type less
- it will teach you new one-liners

It uses [fzf](https://github.com/junegunn/fzf), [skim](https://github.com/lotabout/skim), or [Alfred](https://www.alfredapp.com/) under the hood and it can be either used as a command or as a shell widget (_à la_ Ctrl-R).

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

**navi** can be installed with the following package managers:

[![Packaging status](https://repology.org/badge/vertical-allrepos/navi.svg)](https://repology.org/project/navi/versions)

The recommended way to install **navi** is by running:

```sh
brew install navi
```

If `brew` isn't available, you can check [alternative install instructions](docs/installation.md).

## Usage

There are multiple ways to use **navi**:

- by typing `navi` in the terminal
  - pros: you have access to all possible subcommands and flags
- as a [shell widget](docs/installation.md#installing-the-shell-widget) for the terminal
  - pros: the shell history is correctly populated (i.e. with the actual command you ran instead of `navi`) and you can edit the command as you wish before executing it
- as a [Tmux widget](docs/tmux.md)
  - pros: you can use your cheatsheets in any command-line app even in SSH sessions
- as [aliases](docs/cheatsheet_syntax.md#aliases)
- as a [shell scripting tool](docs/shell_scripting.md)
- as an [Alfred workflow](docs/alfred.md)

In particular, check [these instructions](https://github.com/denisidoro/navi/issues/491) if you want to replicate what's shown in the demo above.

## Cheatsheet repositories

Running **navi** for the first time will help you download and manage cheatsheets. By default, they are stored at `~/.local/share/navi/cheats/`.

You can also:

- [browse through featured cheatsheets](docs/cheatsheet_repositories.md#browsing-through-cheatsheet-repositories)
- [import cheatsheets from git repositories](docs/cheatsheet_repositories.md#importing-cheatsheets)
- [write your own cheatsheets](#cheatsheet-syntax) (and [share them](docs/cheatsheet_repositories.md#submitting-cheatsheets), if you want)
- [use cheatsheets from other tools](docs/cheatsheet_repositories.md#using-cheatsheets-from-other-tools), such as [tldr](https://github.com/tldr-pages/tldr) and [cheat.sh](https://github.com/chubin/cheat.sh)
- [auto-update repositories](docs/cheatsheet_repositories.md#auto-updating-repositories)
- auto-export cheatsheets from your [TiddlyWiki](https://tiddlywiki.com/) notes using a [TiddlyWiki plugin](https://bimlas.github.io/tw5-navi-cheatsheet/)

## Cheatsheet syntax

Cheatsheets are described in `.cheat` files that look like this:

```sh
% git, code

# Change branch
git checkout <branch>

$ branch: git branch | awk '{print $NF}'
```

The full syntax and examples can be found [here](docs/cheatsheet_syntax.md).

## Customization

You can:

- [setup your own config file](docs/navi_config.md)
- [set custom paths for your config file and cheat sheets](docs/navi_config.md#paths-and-environment-variables)
- [change colors](docs/navi_config.md#changing-colors)
- [resize columns](docs/navi_config.md#resizing-columns)
- [change how search is performed](docs/navi_config.md#overriding-fzf-options)

## More info

Please run the following command to read more about all possible options:

```sh
navi --help
```

In addition, please check the [/docs](docs) folder.

## Similar tools

There are many similar projects out there ([beavr](https://github.com/denisidoro/beavr), [bro](https://github.com/hubsmoke/bro), [cheat](https://github.com/cheat/cheat), [cheat.sh](https://github.com/chubin/cheat.sh), [cmdmenu](https://github.com/amacfie/cmdmenu), [eg](https://github.com/srsudar/eg), [how2](https://github.com/santinic/how2), [howdoi](https://github.com/gleitz/howdoi), [Command Line Interface Pages](https://github.com/command-line-interface-pages) and [tldr](https://github.com/tldr-pages/tldr), to name a few).

They are excellent projects, but **navi** remains unique in the following ways:

- it's natural to write cheatsheets tailored to your needs
- arguments are neither hardcoded nor a simple template

## Etymology

[Navi](https://zelda.gamepedia.com/Navi) is a character from [The Legend of Zelda Ocarina of Time](https://zelda.gamepedia.com/Ocarina_of_Time) that provides [Link](https://zelda.gamepedia.com/Link) with a variety of clues to help him solve puzzles and make progress in his quest.
