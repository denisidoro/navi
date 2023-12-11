# Paths and Environment Variables

Navi uses the [`directories-next`](https://crates.io/crates/directories-next) package, which 
defines platform-specific standard locations of directories for config, cache and other data.

Mac users, this is why your files are being stored in `~/Library/Application Support/navi`.

To set custom paths for your config and cheat sheets, you can set the following
environment variables:

```zsh
export NAVI_CONFIG="~/.config/navi/config.yaml"
export NAVI_PATH="~/.local/share/navi"
```
Despite `$NAVI_PATH` being set, it will not be used when installing cheat
sheets directly via navi's own commands. 

For example when running `navi add repo <repo>`, the default paths as per the `directories-next` 
package will still be used.

To avoid this, you may simply clone repos via a regular `git clone` command,
directly into `$NAVI_PATH`.

Note! `navi info cheats-path` and `navi info config-path` display the *default* path, not 
the path set by the user. [It is known that this is a little misleading!](https://github.com/denisidoro/navi/issues/664#issuecomment-1004721178).