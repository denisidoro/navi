# Cheatsheet repositories

## Table of contents

<!-- TOC -->
* [Cheatsheet repositories](#cheatsheet-repositories)
  * [Table of contents](#table-of-contents)
  * [About](#about)
  * [Browsing through cheatsheet repositories](#browsing-through-cheatsheet-repositories)
  * [Importing cheatsheet repositories](#importing-cheatsheet-repositories)
  * [Submitting cheatsheets](#submitting-cheatsheets)
  * [Auto-updating repositories](#auto-updating-repositories)
<!-- TOC -->

## About

Navi lets you work with what we call `cheatsheet repositories`, they are git repositories
and mainly consists of `.cheat` files.

This page is dedicated to the information you might need to work with `cheatsheet repositories`.

## Browsing through cheatsheet repositories

Navi lets you browse featured [GitHub](https://github.com) repositories registered in [@denisidoro/cheats/featured_repos.txt](https://github.com/denisidoro/cheats/blob/master/featured_repos.txt).

You can find them within navi with the following command:

```sh
navi repo browse
```

## Importing cheatsheet repositories

You can import `cheatsheet repositories` using a working git-clone format.\
This includes using an HTTPS URL or an SSH URI.

- Import using HTTPS

    ```sh
    navi repo add https://github.com/denisidoro/cheats
    ```

- Import using SSH

    ```shell
    navi repo add git@github.com:denisidoro/cheats
    ```

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
