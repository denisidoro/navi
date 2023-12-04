## Config file path

The default config file path is set by the `$NAVI_CONFIG` environment variable. If it is not set, it fallbacks to `~/.config/navi/config.yaml`. The command
```sh
navi info config-path
```
prints which config file path is being used. You can get an config file example by running
```sh
navi info config-example
```
or by clicking [here](./config_file_example.yaml). To this this example your config file, run

```sh
navi info config-example > "$(navi info config-path)"
```
## Cheat sheet paths

The default `.cheat` files paths are defined in the `$NAVI_PATH` environment variable in a colon-separated list, e.g.,
```
export NAVI_PATH='/path/to/a/dir:/path/to/another/dir:/yet/another/dir'
```
If this environment variable is unset or if all directories do not exist, `navi` uses that paths defined in its config files. Finally, if there is no config file or if there is no `.cheat` file paths settings there, the default `.cheat` file path fallbacks to `~/.local/share/navi/cheats/`. The command
```sh
navi info cheats-path
```
prints to you all paths used to search for `.cheat` files. 

You can also add other paths at runtime by running `navi` with the `--path` option and a colon-separed paths list, e.g.,
```sh
navi --path '/some/dir:/other/dir'
```
It's irrelevant the directory structure within each path. They can even be all in a single file if you wish, as long as you split them accordingly with lines starting with `%`.
