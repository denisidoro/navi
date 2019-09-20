# navi

An interactive cheatsheet tool for the command-line so that you'll never say the following again:

*- How to run that command again?*
*- Oh, it's not in my bash history*
*- Geez, it's almost what I wanted but I need to change some args*

![Demo](https://user-images.githubusercontent.com/3226564/65347073-8dbbc480-dbb4-11e9-886f-0f10d56def74.gif)

**navi** allows you to browse through cheatsheets (that you may write yourself or downloaded from maintainers) and execute commands, prompting for argument values.

## Installation

**Using [brew](https://brew.sh/):**
```
brew install denisidoro/tools/navi
```

**Without brew:**
```
git clone http://github.com/denisidoro/navi /opt/navi
cd /opt/navi
sudo make install
```

## Usage

Simply call `navi`

## Motivation

The main objectives are:
- to increase discoverability, by finding commands given keywords or descriptions;
- to prevent you from running auxiliar commands, copying the result into the clipboard and then pasting into the original command;
- to improve terminal usage as a whole.

Sure, you can find autocompleters out there for all your favorite commands. However, they are very specific and each one may offer a different learning curve.

Or you can launch and browser and search for instructions on Google, but that takes some time.

**navi**, on the other hand, intends to be a general purpose platform for bookmarking any command at a very low cost.

## .cheat syntax

- lines starting with `%` should contain tags which will be added to any command in a given file;
- lines starting with `#` should be descriptions of commands;
- lines starting with `$` should contain commands that generate suggestion values for a given argument;
- all the other non-empty lines are considered as executable commands.

For example, this is a valid `.cheat` file:
```sh
% git, code

# Change branch
git checkout <branch>

$ branch: git branch --format='%(refname:short)'
```

For advanced usage, please refer to the files in [/cheats](https://github.com/denisidoro/navi/tree/master/cheats).

## Alternatives

- [denisidoro/beavr](https://github.com/denisidoro/beavr);
- [how2](https://github.com/santinic/how2);
- [howdoi](https://github.com/gleitz/howdoi);
- [cheat](https://github.com/cheat/cheat).

## Etymology

In [The Legend of Zelda Ocarina of Time](https://zelda.gamepedia.com/Ocarina_of_Time), [navi](https://zelda.gamepedia.com/Navi) is a character that provides [Link](https://zelda.gamepedia.com/Link) with a variety of clues to help him solve puzzles and progress in his quest.
