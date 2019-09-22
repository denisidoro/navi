# navi <img src="https://user-images.githubusercontent.com/3226564/65362934-b4432500-dbdf-11e9-8f75-815fbc5cbf8f.png" alt="icon" height="28px"/> [![CircleCI](https://circleci.com/gh/denisidoro/navi.svg?style=svg)](https://circleci.com/gh/denisidoro/navi) 

An interactive cheatsheet tool for the command-line so that you'll never say the following again:

>— *How to run that command again?*<br>
— *Oh, it's not in my bash history*<br>
— *Geez, it's almost what I wanted but I need to change some args*

![Demo](https://user-images.githubusercontent.com/3226564/65380182-69431380-dcac-11e9-9af8-0f7b3c869d0f.gif)

**navi** allows you to browse through cheatsheets (that you may write yourself or download from maintainers) and execute commands, prompting for argument values.

Table of Contents
-----------------

   * [Installation](#installation)
      * [Using Homebrew or Linuxbrew](#using-homebrew-or-linuxbrew)
      * [Using git](#using-git)
   * [Upgrading navi](#upgrading-fzf)
   * [Usage](#usage)
   * [Related projects](#related-projects)
   * [<a href="LICENSE">License</a>](#license)

Installation
------------

### Using Homebrew or Linuxbrew

You can use [Homebrew](http://brew.sh/) or [Linuxbrew](http://linuxbrew.sh/)
to install **navi**.

### Using git

Alternatively, you can "git clone" this repository to any directory and run `make`.

```sh
git clone --depth 1 http://github.com/denisidoro/navi /opt/navi
cd /opt/navi
sudo make install
# install fzf: https://github.com/junegunn/fzf
```

Upgrading navi
-------------

**navi** is being actively developed and you might want to upgrade it once in a while. Please follow the instruction below depending on the installation method used:

- brew: `brew update; brew reinstall navi`
- git: `cd /opt/navi && sudo make update`

Usage
-----

By simply running `navi` you will be prompted with the default cheatsheets.

### Preventing execution

If you run `navi query <cmd>`, the results will be pre-filtered.

### Pre-filtering

If you run `navi query <cmd>`, the results will be pre-filtered.

### Searching online repositories

If you run `navi search <cmd>`, **navi** will try to download cheatsheets from online repositories as well.

Please note that these cheatsheets 

### More options

Please refer to `navi --help` for more details.

Trying it out online
--------------------

If you don't have access to bash at the moment and you want to live preview **navi**, head to [this playground](https://www.katacoda.com/denisidoro/scenarios/navi). It'll start a docker container with instructions for you to install and use the tool. Note: login required.

Motivation
----------

The main objectives are:
- to increase discoverability, by finding commands given keywords or descriptions;
- to prevent you from running auxiliar commands, copying the result into the clipboard and then pasting into the original command;
- to easily share one-liners with others so that they don't need to figure out how to write the commands;
- to improve terminal usage as a whole.

Sure, you can find autocompleters out there for all your favorite commands. However, they are very specific and each one may offer a different learning curve.

Or you can launch a browser and search for instructions on Google, but that takes some time.

**navi**, on the other hand, intends to be a general purpose platform for bookmarking any command at a very low cost.

Cheatsheets
-----------

### Using your own custom cheatsheets

In this case, you need to pass a `:`-separated list of separated directories which contain `.cheat`:
```sh
navi --path "/folder/with/cheats"
```

Alternatively, you can set an environment variable in your `.bashrc`-like file:
```sh
export NAVI_PATH="/folder/with/cheats:/another/folder"
```

### Submitting cheatsheets

Feel free to fork this project and open a PR for me to include your contributions.

.cheat syntax
-------------

### Overview

- lines starting with `%` should contain tags which will be added to any command in a given file;
- lines starting with `#` should be descriptions of commands;
- lines starting with `$` should contain commands that generate suggestion values for a given argument;
- all the other non-empty lines are considered as executable commands.

For example, this is a valid `.cheat` file:
```sh
% git, code

# Change branch
git checkout <branch>

$ branch: git branch | awk '{print $NF}'
```

### Variables

Commands that contain variable names inside brackets (eg `<branch>`)

Related projects
----------------

https://github.com/denisidoro/navi/wiki/Related-projects

Etymology
---------

In [The Legend of Zelda Ocarina of Time](https://zelda.gamepedia.com/Ocarina_of_Time), [navi](https://zelda.gamepedia.com/Navi) is a character that provides [Link](https://zelda.gamepedia.com/Link) with a variety of clues to help him solve puzzles and progress in his quest.
