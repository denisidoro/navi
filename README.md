# navi <img src="https://user-images.githubusercontent.com/3226564/65362934-b4432500-dbdf-11e9-8f75-815fbc5cbf8f.png" alt="icon" height="28px"/> [![Actions Status](https://github.com/denisidoro/navi/workflows/Tests/badge.svg)](https://github.com/denisidoro/navi/actions) ![GitHub release](https://img.shields.io/github/v/release/denisidoro/navi?include_prereleases)
  
An interactive cheatsheet tool for the command-line and application launchers.

![Terminal demo](https://user-images.githubusercontent.com/3226564/76437136-ddc35900-6397-11ea-823c-d2da7615fe60.gif)

**navi** allows you to browse through cheatsheets (that you may write yourself or download from maintainers) and execute commands. Suggested values for arguments are dinamically displayed in a list.

#### Pros
- it will make you type less
- it will spare you from knowing CLIs by heart
- it will teach you new one-liners

It uses [fzf](https://github.com/junegunn/fzf), [skim](https://github.com/lotabout/skim), or [Alfred](https://www.alfredapp.com/) under the hood and it can be either used as a command or as a shell widget (*à la* Ctrl-R).

Table of contents
-----------------

   * [Installation](#installation)
   * [Usage](#usage)
   * [Cheatsheet repositories](#cheatsheet-repositories)
   * [Cheatsheet syntax](#cheatsheet-syntax)
   * [Customization](#customization)
   * [Using it for shell scripting](#using-it-for-shell-scripting)
   * [More info](#more-info)
   * [Trying out online](#trying-out-online)
   * [Similar tools](#similar-tools)
   * [Etymology](#etymology)

Installation
------------

**navi** can be installed with the following package managers:

[![Packaging status](https://repology.org/badge/vertical-allrepos/navi.svg)](https://repology.org/project/navi/versions)

The recommended way to install **navi** is by running: 
```sh
brew install navi
```

If `brew` isn't available, you can check [alternative install instructions](docs/installation.md).

Usage
-----

There are 3 ways to use **navi**:

- by typing `navi` in the terminal and hitting \<Enter>
- by invoking it via a [shell widget](docs/installation.md#installing-the-shell-widget) in the terminal
   - this way, the shell history is correctly populated (e.g. `docker run alpine` instead of `navi`) and you can edit the command as you wish before executing it
- as an [Alfred workflow](docs/alfred.md)

Cheatsheet repositories
-----------------------

Running **navi** for the first time will help you download and manage cheatsheets.

You can also:
- [browse through featured cheatsheets](docs/cheatsheet_repositories.md#browsing-through-cheatsheet-repositories)
- [import cheatsheets from git repositories](docs/cheatsheet_repositories.md#importing-cheatsheets)
- [write your own cheatsheets](#cheatsheet-syntax) (and [share them](docs/cheatsheet_repositories.md#submitting-cheatsheets), if you want)
- [use cheatsheets from other tools](docs/cheatsheet_repositories.md#using-cheatsheets-from-other-tools), such as [tldr](https://github.com/tldr-pages/tldr) and [cheat.sh](https://github.com/chubin/cheat.sh)

Cheatsheet syntax
-----------------

Cheatsheets are described in `.cheat` files that look like this:

```sh
% git, code

# Change branch
git checkout <branch>

$ branch: git branch | awk '{print $NF}'
```

The full syntax and examples can be found [here](docs/cheatsheet_syntax.md).

Customization
-------------

You can:
- [change colors](docs/customization.md#changing-colors)
- [resize columns](docs/customization.md#changing-colors)
- [change how search is performed](docs/customization.md#overriding-fzf-options)

Using it for shell scripting
----------------------------

Let's say you want to write a bash script that, among other things, asks the user to write the name of a git branch that should be checked out. 

If you already have the [cheatsheet above](#cheatsheet-syntax), then you write the following in your script:
```sh
navi --query "git checkout" --best-match
```

If you want to set the `<branch>` beforehand in your script, you could then write:
```sh
branch="master" navi --query "git checkout" --best-match
```

More info
---------

Please run the following command to read more about all possible options:
```sh
navi --help
```

In addition, the [/docs](docs) folder includes more information.

Trying out online
-----------------

If you don't have access to a Unix shell at the moment and you want to live preview **navi**, head to [this playground](https://www.katacoda.com/denisidoro/scenarios/navi). It'll start a docker container with instructions for you to install and use the tool. Note: login required.

Similar tools
-------------

There are many similar projects out there ([beavr](https://github.com/denisidoro/beavr), [bro](https://github.com/hubsmoke/bro), [cheat](https://github.com/cheat/cheat), [cheat.sh](https://github.com/chubin/cheat.sh), [cmdmenu](https://github.com/amacfie/cmdmenu), [eg](https://github.com/srsudar/eg), [how2](https://github.com/santinic/how2), [howdoi](https://github.com/gleitz/howdoi) and [tldr](https://github.com/tldr-pages/tldr), to name a few).

They are excellent projects, but **navi** remains unique in the following ways:
- it's natural to write cheatsheets tailored to your needs
- arguments are neither hardcoded nor a simple template

Etymology
---------

In [The Legend of Zelda Ocarina of Time](https://zelda.gamepedia.com/Ocarina_of_Time), [navi](https://zelda.gamepedia.com/Navi) is a character that provides [Link](https://zelda.gamepedia.com/Link) with a variety of clues to help him solve puzzles and progress in his quest.
