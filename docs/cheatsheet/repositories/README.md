# Cheatsheet repositories

<!-- TOC -->
* [Cheatsheet repositories](#cheatsheet-repositories)
  * [About](#about)
  * [Importing cheatsheet repositories](#importing-cheatsheet-repositories)
  * [Submitting cheatsheets](#submitting-cheatsheets)
  * [Auto-updating repositories](#auto-updating-repositories)
<!-- TOC -->

## About

Navi lets you work with what we call `cheatsheet repositories`, they are git repositories
and mainly consists of `.cheat` files.

This page is dedicated to the information you might need to work with `cheatsheet repositories`.

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
