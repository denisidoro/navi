# Cheatsheet repositories

<!-- TOC -->
* [Cheatsheet repositories](#cheatsheet-repositories)
  * [About](#about)
  * [What a cheatsheet repository contains](#what-a-cheatsheet-repository-contains)
  * [Importing cheatsheet repositories](#importing-cheatsheet-repositories)
  * [Submitting cheatsheets](#submitting-cheatsheets)
  * [Auto-updating repositories](#auto-updating-repositories)
<!-- TOC -->

## About

Navi lets you work with what we call `cheatsheet repositories`, they are git repositories
and mainly consists of `.cheat` files.

This page is dedicated to the information you might need to work with `cheatsheet repositories`.

## What a cheatsheet repository contains

A cheatsheet repository is an ordinary git repository, so it can be laid out however its author likes. Navi does not read a manifest or an index file: it walks the whole tree and picks up every file whose name ends in `.cheat` or `.cheat.md`, skipping everything else.

That means a README, a LICENSE and a `.gitignore` can sit next to your cheatsheets without interfering, and that subfolders are free. The only file-level rules are:

- The extension must be `.cheat` or `.cheat.md`. Anything else is ignored.
- Inside a file, the first element of each cheat snippet is a tag line starting with `%`. Those tags are what show up in navi's search results, so the naming of the *file* has no effect on how the cheat is found.
- Anything else has to follow [the cheatsheet syntax](/docs/cheatsheet/syntax/README.md).

### The repository name matters, not the folder names

Tags are what make a cheat findable, and they come from the file contents rather than the path. Two different repositories can each ship a `git.cheat` without colliding, and a repository can reorganise its folders freely without breaking anything.

### Featured repositories

A repository is only shown by `navi repo browse` if it is listed in [`featured_repos.txt`](https://github.com/denisidoro/cheats/blob/master/featured_repos.txt) inside [denisidoro/cheats](https://github.com/denisidoro/cheats). Being a git repository with `.cheat` files is enough for `repo add` to import it; being on that list is what makes it discoverable.

### How navi flattens what it imports

`navi repo add <url>` does not keep the repository's directory structure. It clones into a temporary directory and then copies every selected file into `<cheats-path>/<user>__<repo>/`, replacing the path separator inside the repository with `__`. A repository `someuser/tools` containing `docker/container.cheat` therefore ends up as:

```
$(navi info default-cheats-path)/someuser__tools/docker__container.cheat
```

Because the flattening happens at import time, you should not rely on the folder layout of a repository for anything. This also means that renaming a folder inside a repository only changes the name of the imported file, never a cheat's tags.

### Imported files are copies

Each imported file is a copy, not a link back to the repository. Editing a cheat under `<cheats-path>` affects only your machine and is overwritten the next time that repository is imported. See [Submitting cheatsheets](#submitting-cheatsheets) for how to get a change back upstream.

## Importing cheatsheet repositories

You can import `cheatsheet repositories` with the `repo add` subcommand.\
See [/docs/usage/commands/repo](/docs/usage/commands/repo/README.md#importing-cheatsheet-repositories) for more details.

## Submitting cheatsheets

The featured repository for cheatsheets is [denisidoro/cheats](https://github.com/denisidoro/cheats),
feel free to open a PR[^1] there for me to include your contributions.

In order to add your own repository as a featured cheatsheet repo, please [edit this file](https://github.com/denisidoro/cheats/edit/master/featured_repos.txt) and open a PR[^1].

## Auto-updating repositories

Right now, **navi** doesn't have support for auto-updating out of the box.
However, you can achieve this by using `git` and `crontab`.

- First make sure you cloned your repo using `git` to the correct folder:

  ```sh
  user="<user>"
  repo="<repo>"
  git clone "https://github.com/${user}/${repo}" "$(navi info cheats-path)/${user}__${repo}"
  ```

- Then, add a cron job:

  ```sh
  crontab -e
  */0 11 * * * bash -c 'cd "$(/usr/local/bin/navi info cheats-path)/<user>__<repo>" && /usr/local/bin/git pull -q origin master'
  ```

> [!NOTE]
> Please note the cron job above is just an example **AND** you should edit it accordingly:
>
>- In this example, the cron job is triggered every day at 11am.
>  
>    You might want to check out [crontab guru](https://crontab.guru/) regarding crontab.
>
>- The full paths to `navi` and `git` may differ in your setup.
>
>    Check their actual values using `which` as `which <program>`.
>
>- Don't forget to replace `<user>__<repo>` with the actual folder name

[^1]: A *PR* is short for Pull Request
