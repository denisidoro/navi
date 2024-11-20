- [Paths and Environment Variables](#paths-and-environment-variables)
  - [Config file path](#config-file-path)
  - [Cheat sheet paths](#cheat-sheet-paths)
- [Logging](#logging)
- [Customization](#customization)
  - [Changing colors](#changing-colors)
  - [Resizing columns](#resizing-columns)
  - [Overriding fzf options](#overriding-fzf-options)

# Paths and Environment Variables

Navi uses the [`directories-next`](https://crates.io/crates/directories-next) package, which 
defines platform-specific standard locations of directories for config, cache and other data (Mac users, this is why your files are being stored in `~/Library/Application Support/navi`).

## Config file path

The default config file path is set by the `$NAVI_CONFIG` environment variable. If it is not set, it fallbacks to `~/.config/navi/config.yaml`. The command
```sh
navi info config-path
```
prints which config file path is being used. You can get a config file example by running
```sh
navi info config-example
```
or by clicking [here](./config_file_example.yaml). To turn this example your config file, run

```sh
navi info config-example > "$(navi info config-path)"
```
## Cheat sheet paths

The default `.cheat` files paths are defined in the `$NAVI_PATH` environment variable in a colon-separated list, e.g.,
```sh
export NAVI_PATH='/path/to/a/dir:/path/to/another/dir:/yet/another/dir'
```
If this environment variable is unset or if all directories do not exist, `navi` uses the [paths defined in the config file](https://github.com/denisidoro/navi/blob/master/docs/config_file_example.yaml#L21-L24). Finally, if there is no config file or if the `.cheat` file paths was not set, the default `.cheat` file paths fallbacks to `~/.local/share/navi/cheats/`. The command
```sh
navi info cheats-path
```
prints to you all paths used to search for `.cheat` files. 

You can also add other paths at runtime by running `navi` with the `--path` option and a colon-separated paths list, e.g.,
```sh
navi --path '/some/dir:/other/dir'
```
It's irrelevant the directory structure within each path. They can even be all in a single file if you wish, as long as you split them accordingly with lines starting with `%`.

Despite `$NAVI_PATH` being set, it will not be used when installing cheat sheets directly via navi's own commands.  For example when running `navi add repo <repo>`, the default paths as per the `directories-next` package will still be used. To avoid this, you may simply clone repos via a regular `git clone` command, directly into `$NAVI_PATH`.

Note! `navi info cheats-path` and `navi info config-path` display the *default* path, not 
the path set by the user. [It is known that this is a little misleading!](https://github.com/denisidoro/navi/issues/664#issuecomment-1004721178).

# Logging

The log file will be created under the same directory where the config locates.

And you can use the `RUST_LOG` env to set the log level, e.g. `RUST_LOG=debug navi`.

# Customization

## Changing colors

You can change the [color scheme](https://github.com/junegunn/fzf/wiki/Color-schemes) by [overriding fzf options](#overriding-fzf-options).

In addition, you can change the text color for each column by properly configuring _navi_'s `config.yaml`. Please check `navi --help` for more instructions.

## Resizing columns

You can change the column widths by properly configuring _navi_'s `config.yaml`. Please check `navi --help` for more instructions.

## Overriding fzf options

Let's say you want to override [$FZF_DEFAULT_OPTS](https://github.com/junegunn/fzf#layout) with `--height 3`.

This can be overridden in the following ways:

```sh
# if you want to override only when selecting snippets
navi --fzf-overrides '--height 3'

# alternatively, using an environment variable in your .bashrc-like file:
export NAVI_FZF_OVERRIDES='--height 3'

# if you want to override only when selecting argument values
navi --fzf-overrides-var '--height 3'

# alternatively, using an environment variable in your .bashrc-like file:
export NAVI_FZF_OVERRIDES_VAR='--height 3'

# if you want to override for all cases
FZF_DEFAULT_OPTS="--height 3" navi
```

In addition, this can be set by properly configuring _navi_'s `config.yaml`. Please check `navi --help` for more instructions.
