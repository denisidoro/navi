# navi <img src="https://user-images.githubusercontent.com/3226564/65362934-b4432500-dbdf-11e9-8f75-815fbc5cbf8f.png" alt="icon" height="28px"/> [![Actions Status](https://github.com/denisidoro/navi/workflows/Tests/badge.svg)](https://github.com/denisidoro/navi/actions) ![GitHub release](https://img.shields.io/github/v/release/denisidoro/navi?include_prereleases)
  
> :information_source: If you're here because you upgraded **navi** and are having some issues, please check [this thread](https://github.com/denisidoro/navi/issues/201).

An interactive cheatsheet tool for the command-line.

![Demo](https://user-images.githubusercontent.com/3226564/76437136-ddc35900-6397-11ea-823c-d2da7615fe60.gif)

**navi** allows you to browse through cheatsheets (that you may write yourself or download from maintainers) and execute commands, with argument values prompted to you. It uses [fzf](https://github.com/junegunn/fzf) under the hood.

It can be either used as a command or as a shell widget (*à la* Ctrl-R).

Table of contents
-----------------

   * [Installation](#installation)
      * [Using Homebrew or Linuxbrew](#using-homebrew-or-linuxbrew)
      * [Using one-liner script](#using-one-liner-script)
      * [Downloading pre-compiled binaries](#downloading-pre-compiled-binaries)
      * [Building from source](#building-from-source)
   * [Usage](#usage)
      * [Preventing execution](#preventing-execution)
      * [Pre-filtering](#pre-filtering)
      * [Shell widget](#shell-widget)
      * [More options](#more-options)
   * [Trying out online](#trying-out-online)
   * [Cheatsheets](#cheatsheets)
      * [Using your own custom cheatsheets](#using-your-own-custom-cheatsheets)
      * [Submitting cheatsheets](#submitting-cheatsheets)
   * [Cheatsheet syntax](#cheatsheet-syntax)
      * [Syntax overview](#syntax-overview)
      * [Variables](#variables)
      * [Variable dependency](#variable-dependency)
      * [Variable options](#variable-options)
         * [Table formatting](#table-formatting)
         * [Multiple choice](#multiple-choice)
   * [List customization](#list-customization)
   * [Motivation](#motivation)
   * [Related projects](#related-projects)
   * [Etymology](#etymology)

Installation
------------

### Using [Homebrew](http://brew.sh/) or [Linuxbrew](http://linuxbrew.sh/)

```sh
brew install denisidoro/tools/navi
```

Alternatively, you can use the official formula (but it will install a very old version of **navi**):
```sh
brew install navi
```

### Using one-liner script

```bash
bash <(curl -sL https://raw.githubusercontent.com/denisidoro/navi/master/scripts/install)
```

### Downloading pre-compiled binaries

You can download binaries [here](https://github.com/denisidoro/navi/releases/latest).

### Building from source

```bash
git clone https://github.com/denisidoro/navi ~/.navi
cd ~/.navi
make install # or make SOURCE_DIR=/opt/navi BIN_DIR=/usr/local/bin install
```

Usage
-----

By simply running `navi` you will be prompted with the default cheatsheets.

### Preventing execution

If you run `navi --print`, the selected snippet won't be executed. It will be printed to stdout instead.

### Pre-filtering

If you run `navi query <cmd>`, the results will be pre-filtered.

### Shell widget

You can use **navi** as a widget to your shell. This way, your history is correctly populated and you can edit the command as you wish before executing it.

In order to use it, add this line to your `.bashrc`-like file:
```sh
# bash
source <(navi widget bash)

# zsh
source <(navi widget zsh)

# fish
navi widget fish | source
```

By default, `Ctrl+G` is assigned to launching **navi**. If you want to change the keybinding, replace the argument of `bind` or `bindkey` in [the widget file](https://github.com/denisidoro/navi/search?q=filename%3Anavi.plugin.*&unscoped_q=filename%3Anavi.plugin.*).

If you want a widget for other shells, please upvote [this issue](https://github.com/denisidoro/navi/issues/37).

### More options

Please refer to `navi --help` for more details.

Trying out online
--------------------

If you don't have access to bash at the moment and you want to live preview **navi**, head to [this playground](https://www.katacoda.com/denisidoro/scenarios/navi). It'll start a docker container with instructions for you to install and use the tool. Note: login required.

Cheatsheets
-----------

### Using your own custom cheatsheets

In this case, you need to pass a `:`-separated list of separated directories which contain `.cheat` files:
```sh
navi --path "/folder/with/cheats"
```

Alternatively, you can set an environment variable in your `.bashrc`-like file:
```sh
export NAVI_PATH="/folder/with/cheats:/another/folder"
```

### Submitting cheatsheets

Feel free to fork this project and open a PR for me to include your contributions.

Cheatsheet syntax
-----------------

Cheatsheets are described in `.cheat` files.

### Syntax overview

- lines starting with `%` determine the start of a new cheatsheet. They should contain tags which will be added to any command in a given file;
- lines starting with `#` should be descriptions of commands;
- lines starting with `;` are ignored. You can use them for metacomments;
- lines starting with `$` should contain commands that generate a list of possible values for a given argument;
- all the other non-empty lines are considered as executable commands.

For example, this is a valid `.cheat` file:
```sh
% git, code

# Change branch
git checkout <branch>

$ branch: git branch | awk '{print $NF}'
```

It's irrelevant how many files are used to store cheatsheets. They can be all in a single file if you wish, as long as you split them accordingly with lines starting with `%`.

Commands may be multiline:
```sh
# This will output foo\nyes
echo foo
true \
   && echo yes \
   || echo no
```

### Variables

The interface prompts for variable names inside brackets (eg `<branch>`).

Variable names should only include alphanumeric characters and `_`.

### Variable dependency

The command for generating possible inputs can refer other variables:
```sh
# If you select 2 for x, the possible values of y will be 12 and 22
echo <x> <y>

$ x: echo -e '1\n2\n3'
$ y: echo -e "$((x+10))\n$((x+20))"
```

### Variable options

For lines starting with `$` you can add use`---` to parse parameters to `fzf`.
* `--allow-extra` *(experimental)*: handles `fzf` option `--print-query`. `enter` will prefer a selection,
    `tab` will prefer the query typed. 
* `--multi` : forwarded option to `fzf`.
* `--header-lines` : forwarded option to `fzf`
* `--column` : forwarded option to `fzf`.
* `--delimiter` : forwarded option to `fzf`.

#### Table formatting

You can pick a specific column of a selection and set the number of lines considered as headers via `--column`, `--delimiter` and `--header-lines`:

```sh
# This will pick the 3rd column and use the first line as header
docker rmi <image_id>

$ image_id: docker images --- --column 3 --header-lines 1 --delimiter '\s\s+'
```

#### Multiple choice

You can select multiple values via `--multi` and hitting `<TAB>`:

```sh
# The resulting command will be something like: cat "a.txt" "b.txt"
cat <files>

$ files: ls --- --multi
```

List customization
------------------

Lists can be stylized with the [$FZF_DEFAULT_OPTS](https://github.com/junegunn/fzf) environment variable or `--fzf-overrides`. This way, you can change the [color scheme](https://github.com/junegunn/fzf/wiki/Color-schemes), for example.

Motivation
----------

The main objectives are:
- to increase discoverability, by finding snippets given keywords or descriptions;
- to prevent you from running auxiliar commands, copying the result into the clipboard and then pasting into the original command;
- to easily share one-liners with others so that they don't need to figure out how to write the commands;
- to improve terminal usage as a whole.

Sure, you can find autocompleters out there for all your favorite commands. However, they are very specific and each one may offer a different learning curve.

Or you can launch a browser and search for instructions on Google, but that takes some time.

**navi**, on the other hand, intends to be a general purpose platform for bookmarking any snippet at a very low cost.

Related projects
----------------

There are many similar projects out there ([bro](https://github.com/hubsmoke/bro), [eg](https://github.com/srsudar/eg), [cheat.sh](https://github.com/chubin/cheat.sh), [tldr](https://github.com/tldr-pages/tldr), [cmdmenu](https://github.com/amacfie/cmdmenu), [cheat](https://github.com/cheat/cheat), [beavr](https://github.com/denisidoro/beavr), [how2](https://github.com/santinic/how2) and [howdoi](https://github.com/gleitz/howdoi), to name a few).

Most of them provide excellent cheatsheet repositories, but lack a nice UI and argument suggestions.

Etymology
---------

In [The Legend of Zelda Ocarina of Time](https://zelda.gamepedia.com/Ocarina_of_Time), [navi](https://zelda.gamepedia.com/Navi) is a character that provides [Link](https://zelda.gamepedia.com/Link) with a variety of clues to help him solve puzzles and progress in his quest.
