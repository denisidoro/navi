# The repo subcommands of navi

<!-- TOC -->
* [The repo subcommands of navi](#the-repo-subcommands-of-navi)
  * [Commands Reference](#commands-reference)
  * [Browsing through cheatsheet repositories](#browsing-through-cheatsheet-repositories)
* [Importing cheatsheet repositories](#importing-cheatsheet-repositories)
<!-- TOC -->

## Commands Reference

| Command | Description                                                       |
|---------|-------------------------------------------------------------------|
| add     | Lets you import a cheatsheet repository                           |
| browser | Lets you browse through a curated list of cheatsheet repositories |

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

> [!CAUTION]
> Despite `$NAVI_PATH` being set, it will not be used when installing cheat sheets directly via navi's own commands.\
> For example when running `navi add repo <repo>`, the default paths will still be used.
> 
> To avoid this, you may simply clone repos via a regular `git clone` command, directly into `$NAVI_PATH`.
