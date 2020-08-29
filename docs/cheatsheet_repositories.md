Cheatsheet repositories
-----------------------

* [Browsing through cheatsheet repositories](#browsing-through-cheatsheet-repositories)
* [Importing cheatsheets](#importing-cheatsheets)
* [Adding your own cheatsheets](#adding-your-own-cheatsheets)
* [Submitting cheatsheets](#submitting-cheatsheets)

### Browsing through cheatsheet repositories

You can find cheatsheet repositories with:
```sh
navi repo browse
```

### Importing cheatsheets

You can import cheatsheets from any git repository that includes `.cheat` files:
```sh
navi repo add https://github.com/denisidoro/cheats
```

### Adding your own cheatsheets

You can either start a git repo with cheatsheets and import it as described above or you can add them directly to [data_dir](https://github.com/soc/dirs-rs#Features)`/navi`.

### Submitting cheatsheets

The main repository for cheatsheets is [denisidoro/cheats](https://github.com/denisidoro/cheats). Feel free to open a PR there for me to include your contributions.

In order to add your own repository as a featured cheatsheet repo, please [edit this file](https://github.com/denisidoro/cheats/edit/master/featured_repos.txt). This list will be displayed when `navi repo browse` is run.
