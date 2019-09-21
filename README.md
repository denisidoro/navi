# navi <img src="https://user-images.githubusercontent.com/3226564/65362934-b4432500-dbdf-11e9-8f75-815fbc5cbf8f.png" alt="icon" height="28px"/> [![CircleCI](https://circleci.com/gh/denisidoro/navi.svg?style=svg)](https://circleci.com/gh/denisidoro/navi) 



An interactive cheatsheet tool for the command-line so that you'll never say the following again:

>— *How to run that command again?*<br>
— *Oh, it's not in my bash history*<br>
— *Geez, it's almost what I wanted but I need to change some args*

<img src="https://user-images.githubusercontent.com/3226564/65347073-8dbbc480-dbb4-11e9-886f-0f10d56def74.gif" alt="Demo" width="60%" />

**navi** allows you to browse through cheatsheets (that you may write yourself or download from maintainers) and execute commands, prompting for argument values.

## Installation

**Using [brew](https://brew.sh/):**
```sh
brew install denisidoro/tools/navi
```

**Without brew:**
```sh
git clone http://github.com/denisidoro/navi /opt/navi
cd /opt/navi
sudo make install
# install fzf: https://github.com/junegunn/fzf
```

## Usage

Simply call: 
```sh
navi
```

## Trying it out online

Head to [this playground](https://www.katacoda.com/denisidoro/scenarios/navi) for previewing **navi**.

## Motivation

The main objectives are:
- to increase discoverability, by finding commands given keywords or descriptions;
- to prevent you from running auxiliar commands, copying the result into the clipboard and then pasting into the original command;
- to easily share one-liners with others so that they don't need to figure out how to write the commands;
- to improve terminal usage as a whole.

Sure, you can find autocompleters out there for all your favorite commands. However, they are very specific and each one may offer a different learning curve.

Or you can launch a browser and search for instructions on Google, but that takes some time.

**navi**, on the other hand, intends to be a general purpose platform for bookmarking any command at a very low cost.

## Using your own cheatsheets

In this case, you need to pass the directory which contains `.cheat` files as in:
```sh
navi --dir "/folder/with/cheats"
```

Alternatively, you can set an environment variable in your `.bashrc`-like file:
```sh
export NAVI_DIR="/folder/with/cheats"
```

## Submitting cheatsheets

Feel free to fork this project and open a PR for me to include your contributions.

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

- [cmdmenu](https://github.com/amacfie/cmdmenu);
- [denisidoro/beavr](https://github.com/denisidoro/beavr);
- [how2](https://github.com/santinic/how2);
- [howdoi](https://github.com/gleitz/howdoi);
- [cheat](https://github.com/cheat/cheat).

## Etymology

In [The Legend of Zelda Ocarina of Time](https://zelda.gamepedia.com/Ocarina_of_Time), [navi](https://zelda.gamepedia.com/Navi) is a character that provides [Link](https://zelda.gamepedia.com/Link) with a variety of clues to help him solve puzzles and progress in his quest.
