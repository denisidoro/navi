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
- as a [shell widget](docs/widgets/README.md#installing-the-shell-widget) for the terminal
  - pros: the shell history is correctly populated (i.e. with the actual command you ran instead of `navi`) and you can edit the command as you wish before executing it
- as a [Tmux widget](docs/widgets/howto/TMUX.md)
  - pros: you can use your cheatsheets in any command-line app even in SSH sessions
- as [aliases](docs/cheatsheet/syntax/README.md#aliases)
- as a [shell scripting tool](docs/usage/shell-scripting/README.md)

In particular, check [these instructions](https://github.com/denisidoro/navi/issues/491) if you want to replicate what's shown in the demo above.

## Cheatsheet repositories

Running **navi** for the first time will help you download and manage cheatsheets. By default, they are stored at `~/.local/share/navi/cheats/`.

You can also:

- [browse through featured cheatsheets](docs/usage/commands/repo/README.md#browsing-through-cheatsheet-repositories)
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


## 🌐 Web Resources & Aesthetic Symbols Index
- [LEFT BLACK LENTICULAR BRACKET](https://baroque-font-vault-96.pages.dev/symbol/left-black-lenticular-bracket/)
- [SYM 1D41D](https://ribbon-heart-fonts-86.pages.dev/symbol/sym-1d41d/)
- [SYM 2636](https://mecha-glitch-fonts-82.pages.dev/symbol/sym-2636/)
- [CYBER PHANTOM GLYPH](https://pastel-moe-kaomoji-91.pages.dev/symbol/cyber-phantom-glyph/)
- [SYM 262F](https://pink-ribbon-fonts-28.pages.dev/symbol/sym-262f/)
- [SYM 2644](https://matrix-glitch-text-59.pages.dev/symbol/sym-2644/)
- [SYM 1F644](https://neon-gamer-symbols-64.pages.dev/symbol/sym-1f644/)
- [SYM 1D443](https://angelic-soft-text-59.pages.dev/symbol/sym-1d443/)
- [SYM 1D42C](https://matrix-terminal-fonts-30.pages.dev/symbol/sym-1d42c/)
- [SUPER SHY BLUSHING KAOMOJI](https://vintage-lace-symbols-65.pages.dev/symbol/super-shy-blushing-kaomoji/)
- [SINGLE EIGHTH MUSICAL NOTE](https://coquette-aesthetic-symbols-96.pages.dev/symbol/single-eighth-musical-note/)
- [SYM 1D41E](https://cyber-clan-tags-38.pages.dev/symbol/sym-1d41e/)
- [SYM 1D44C](https://coquette-aesthetic-symbols-96.pages.dev/symbol/sym-1d44c/)
- [SYM 1F92F](https://mecha-crosshair-tags-20.pages.dev/symbol/sym-1f92f/)
- [SYM 1D47D](https://vintage-lace-symbols-65.pages.dev/symbol/sym-1d47d/)
- [SYM 1D473](https://matrix-glitch-text-59.pages.dev/symbol/sym-1d473/)
- [SYM 1D412](https://angelic-soft-text-59.pages.dev/symbol/sym-1d412/)
- [SYM 2656](https://matrix-glitch-text-59.pages.dev/symbol/sym-2656/)
- [SYM 1F62A](https://vintage-lace-symbols-65.pages.dev/symbol/sym-1f62a/)
- [EIGHT POINTED STAR](https://pink-ribbon-fonts-28.pages.dev/symbol/eight-pointed-star/)
- [SYM 26FD](https://angelic-bow-symbols-76.pages.dev/symbol/sym-26fd/)
- [SYM 2672](https://balletcore-unicode-67.pages.dev/symbol/sym-2672/)
- [SYM 1D42A](https://mecha-crosshair-tags-20.pages.dev/symbol/sym-1d42a/)
- [SYM 1D460](https://angelic-soft-text-59.pages.dev/symbol/sym-1d460/)
- [SYM 1F92E](https://neon-gamer-symbols-64.pages.dev/symbol/sym-1f92e/)
- [SYM 1D407](https://pink-ribbon-fonts-28.pages.dev/symbol/sym-1d407/)
- [SYM 1F635 200D 1F4AB](https://coquette-aesthetic-symbols-96.pages.dev/symbol/sym-1f635-200d-1f4ab/)
- [SCORPIO ZODIAC SCORPION](https://vintage-lace-symbols-65.pages.dev/symbol/scorpio-zodiac-scorpion/)
- [SYM 2676](https://angelic-soft-text-59.pages.dev/symbol/sym-2676/)
- [SYM 26FD](https://anime-sparkle-text-58.pages.dev/symbol/sym-26fd/)
- [CUTE BUNNY RABBIT FACE](https://vintage-lace-symbols-65.pages.dev/symbol/cute-bunny-rabbit-face/)
- [LEO ZODIAC LION](https://vintage-lace-symbols-65.pages.dev/symbol/leo-zodiac-lion/)
- [SYM 2654](https://matrix-glitch-text-59.pages.dev/symbol/sym-2654/)
- [RIGHTWARDS PAIRED HARPOON](https://matrix-glitch-text-59.pages.dev/symbol/rightwards-paired-harpoon/)
- [SYM 2643](https://anime-sparkle-text-58.pages.dev/symbol/sym-2643/)
- [LEFT HEAVY BRACKET BOX](https://vintage-lace-symbols-65.pages.dev/symbol/left-heavy-bracket-box/)
- [LEFT BLACK LENTICULAR BRACKET](https://angel-core-bios-50.pages.dev/symbol/left-black-lenticular-bracket/)
- [LIBRA ZODIAC SCALES](https://vintage-lace-symbols-65.pages.dev/symbol/libra-zodiac-scales/)
- [BORDERS DIVIDERS](https://pink-ribbon-fonts-28.pages.dev/pt/borders-dividers/)
- [SYM 2624](https://cyber-clan-tags-38.pages.dev/symbol/sym-2624/)
- [SYM 268C](https://balletcore-unicode-67.pages.dev/symbol/sym-268c/)
- [SYM 1D436](https://coquette-aesthetic-symbols-51.pages.dev/symbol/sym-1d436/)
- [SYM 1F921](https://coquette-aesthetic-symbols-96.pages.dev/symbol/sym-1f921/)
- [SYM 1F627](https://balletcore-unicode-67.pages.dev/symbol/sym-1f627/)
- [SYM 1D47F](https://balletcore-unicode-67.pages.dev/symbol/sym-1d47f/)
- [MUSIC SHARP SIGN](https://angelic-soft-text-59.pages.dev/symbol/music-sharp-sign/)
- [SYM 265B](https://zen-arrow-symbols-99.pages.dev/symbol/sym-265b/)
- [SYM 1F975](https://zen-arrow-symbols-99.pages.dev/symbol/sym-1f975/)
- [DAGGER CROSS SYMBOL](https://cyber-clan-tags-38.pages.dev/symbol/dagger-cross-symbol/)
- [SYM 260A](https://zen-arrow-symbols-99.pages.dev/symbol/sym-260a/)
- [SYM 265E](https://angelic-bow-symbols-76.pages.dev/symbol/sym-265e/)
- [SYM 262D](https://vintage-lace-symbols-65.pages.dev/symbol/sym-262d/)
- [SYM 26CD](https://angelic-bow-symbols-76.pages.dev/symbol/sym-26cd/)
- [SYM 1D45F](https://soft-pastel-unicode-78.pages.dev/symbol/sym-1d45f/)
- [SYM 26B6](https://angelic-soft-text-59.pages.dev/symbol/sym-26b6/)
- [SYM 2628](https://coquette-aesthetic-symbols-51.pages.dev/symbol/sym-2628/)
- [SYM 2616](https://balletcore-unicode-67.pages.dev/symbol/sym-2616/)
- [BEAMED EIGHTH NOTES](https://coquette-aesthetic-symbols-51.pages.dev/symbol/beamed-eighth-notes/)
- [SYM 1F910](https://minimal-star-symbols-32.pages.dev/symbol/sym-1f910/)
- [SYM 26AF](https://anime-sparkle-text-58.pages.dev/symbol/sym-26af/)
- [SYM 2639](https://coquette-aesthetic-symbols-51.pages.dev/symbol/sym-2639/)
- [TABLE FLIP RAGE KAOMOJI](https://balletcore-unicode-67.pages.dev/symbol/table-flip-rage-kaomoji/)
- [SYM 1D426](https://coquette-aesthetic-symbols-51.pages.dev/symbol/sym-1d426/)
- [SYM 1D43F](https://mecha-crosshair-tags-20.pages.dev/symbol/sym-1d43f/)
- [DISCORD STATUS](https://minimal-star-symbols-32.pages.dev/ja/discord-status/)
- [SYM 26E5](https://angelic-bow-symbols-76.pages.dev/symbol/sym-26e5/)
- [SYM 1D48D](https://soft-pink-fonts-41.pages.dev/symbol/sym-1d48d/)
- [SYM 1D444](https://pink-ribbon-fonts-28.pages.dev/symbol/sym-1d444/)
- [SYM 1F60A](https://coquette-aesthetic-symbols-51.pages.dev/symbol/sym-1f60a/)
- [SYM 1D454](https://pink-ribbon-fonts-28.pages.dev/symbol/sym-1d454/)
- [SYM 1D476](https://pink-ribbon-fonts-28.pages.dev/symbol/sym-1d476/)
- [SYM 2667](https://coquette-aesthetic-symbols-51.pages.dev/symbol/sym-2667/)
- [STAR OPERATOR](https://pink-ribbon-fonts-28.pages.dev/symbol/star-operator/)
- [DISCORD STATUS](https://vintage-lace-symbols-65.pages.dev/pt/discord-status/)
- [SYM 265F](https://cyber-clan-tags-38.pages.dev/symbol/sym-265f/)
- [SYM 1D49E](https://mecha-crosshair-tags-20.pages.dev/symbol/sym-1d49e/)
- [SYM 1D41C](https://angelic-soft-text-59.pages.dev/symbol/sym-1d41c/)
- [SYM 1D400](https://coquette-aesthetic-symbols-96.pages.dev/symbol/sym-1d400/)
- [PISCES ZODIAC FISHES](https://angelic-soft-text-59.pages.dev/symbol/pisces-zodiac-fishes/)
- [BRACKETS](https://angelic-soft-text-59.pages.dev/brackets/)
- [SYM 26B0](https://angelic-bow-symbols-76.pages.dev/symbol/sym-26b0/)
- [SYM 26EF](https://pink-ribbon-fonts-28.pages.dev/symbol/sym-26ef/)
- [SYM 1F642](https://angelic-soft-text-59.pages.dev/symbol/sym-1f642/)
- [SYM 1D469](https://coquette-aesthetic-symbols-51.pages.dev/symbol/sym-1d469/)
- [SYM 1D463](https://angelic-soft-text-59.pages.dev/symbol/sym-1d463/)
- [SYM 26D7](https://zen-arrow-symbols-99.pages.dev/symbol/sym-26d7/)
- [ZODIAC CELESTIAL](https://vintage-lace-symbols-65.pages.dev/ja/zodiac-celestial/)
- [FREEFIRE NAMES](https://vintage-lace-symbols-65.pages.dev/ja/freefire-names/)
- [SYM 1F494](https://coquette-aesthetic-symbols-51.pages.dev/symbol/sym-1f494/)
- [WARM HUG EMBRACE KAOMOJI](https://cyber-clan-tags-38.pages.dev/symbol/warm-hug-embrace-kaomoji/)
- [KHANDA EMBLEM](https://minimal-star-symbols-32.pages.dev/symbol/khanda-emblem/)
- [SYM 26D0](https://zen-arrow-symbols-99.pages.dev/symbol/sym-26d0/)
- [SYM 1D4A2](https://coquette-aesthetic-symbols-96.pages.dev/symbol/sym-1d4a2/)
- [SYM 2725](https://zen-arrow-symbols-99.pages.dev/symbol/sym-2725/)
- [FLUTTERING BUTTERFLY](https://coquette-aesthetic-symbols-51.pages.dev/symbol/fluttering-butterfly/)
- [SYM 1D409](https://cyber-clan-tags-38.pages.dev/symbol/sym-1d409/)
- [SYM 2678](https://angelic-soft-text-59.pages.dev/symbol/sym-2678/)
- [SYM 267C](https://angelic-soft-text-59.pages.dev/symbol/sym-267c/)
- [SYM 1D453](https://coquette-aesthetic-symbols-96.pages.dev/symbol/sym-1d453/)
- [SYM 1F49B](https://vintage-lace-symbols-65.pages.dev/symbol/sym-1f49b/)
- [SYM 267B](https://matrix-glitch-text-59.pages.dev/symbol/sym-267b/)
- [KAOMOJI](https://pink-ribbon-fonts-28.pages.dev/ja/kaomoji/)
- [SYM 26AD](https://matrix-glitch-text-59.pages.dev/symbol/sym-26ad/)
- [CANCER ZODIAC CRAB](https://minimal-star-symbols-32.pages.dev/symbol/cancer-zodiac-crab/)
- [ARROWS LINES](https://balletcore-unicode-67.pages.dev/vi/arrows-lines/)
- [SYM 1F644](https://clean-sparkle-text-75.pages.dev/symbol/sym-1f644/)
- [SYM 1F612](https://minimal-star-symbols-32.pages.dev/symbol/sym-1f612/)
- [BRACKETS](https://vintage-lace-symbols-65.pages.dev/ru/brackets/)
- [SYM 1F640](https://coquette-aesthetic-symbols-96.pages.dev/symbol/sym-1f640/)
- [SYM 26DD](https://pink-ribbon-fonts-28.pages.dev/symbol/sym-26dd/)
- [SYM 1F609](https://vintage-lace-symbols-65.pages.dev/symbol/sym-1f609/)
- [SYM 1D404](https://cyber-clan-tags-38.pages.dev/symbol/sym-1d404/)
- [SYM 1F624](https://pastel-moe-kaomoji-91.pages.dev/symbol/sym-1f624/)
- [SYM 1F9E1](https://manga-emotion-symbols-69.pages.dev/symbol/sym-1f9e1/)
- [HEARTS](https://baroque-unicode-decor-43.pages.dev/pt/hearts/)
- [SYM 2658](https://cyber-clan-tags-38.pages.dev/symbol/sym-2658/)
- [SYM 1D46D](https://kawaii-kaomoji-hub-31.pages.dev/symbol/sym-1d46d/)
- [SYM 1D428](https://baroque-unicode-decor-43.pages.dev/symbol/sym-1d428/)
- [SYM 26C5](https://baroque-unicode-decor-43.pages.dev/symbol/sym-26c5/)
- [SYM 26CC](https://zen-arrow-symbols-99.pages.dev/symbol/sym-26cc/)
- [SYM 2658](https://angelic-soft-text-59.pages.dev/symbol/sym-2658/)
- [SYM 1D435](https://baroque-unicode-decor-43.pages.dev/symbol/sym-1d435/)
- [SYM 1F632](https://zen-arrow-symbols-99.pages.dev/symbol/sym-1f632/)
- [FREEFIRE NAMES](https://coquette-aesthetic-symbols-51.pages.dev/pt/freefire-names/)
- [SYM 1D4A5](https://neon-gamer-symbols-64.pages.dev/symbol/sym-1d4a5/)
- [CIRCLED STAR](https://kawaii-kaomoji-hub-31.pages.dev/symbol/circled-star/)
- [SYM 1F915](https://clean-sparkle-text-75.pages.dev/symbol/sym-1f915/)
- [SYM 1F632](https://mecha-crosshair-tags-20.pages.dev/symbol/sym-1f632/)
- [SYM 2659](https://classic-literature-symbols-64.pages.dev/symbol/sym-2659/)
- [SYM 26D3](https://cyber-clan-tags-38.pages.dev/symbol/sym-26d3/)
