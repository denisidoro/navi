# Configuring Navi

Navi allows you to configure it with a YAML configuration.

<!-- TOC -->
* [Configuring Navi](#configuring-navi)
  * [Paths and Environment Variables](#paths-and-environment-variables)
    * [The default configuration file path](#the-default-configuration-file-path)
    * [Cheatsheets paths](#cheatsheets-paths)
      * [The default cheatsheets path](#the-default-cheatsheets-path)
      * [Defining the cheatsheets path with the environment variable](#defining-the-cheatsheets-path-with-the-environment-variable)
      * [Defining the cheatsheets path in the configuration file](#defining-the-cheatsheets-path-in-the-configuration-file)
        * [[DEPRECATED] - Using the `path` directive](#deprecated---using-the-path-directive)
  * [Customization](#customization)
    * [Changing colors](#changing-colors)
      * [fzf color scheme](#fzf-color-scheme)
      * [Navi colors](#navi-colors)
    * [Resizing columns](#resizing-columns)
    * [Overriding fzf options](#overriding-fzf-options)
      * [Overriding during cheats selection](#overriding-during-cheats-selection)
      * [Overriding during values selection](#overriding-during-values-selection)
      * [Overriding for all cases](#overriding-for-all-cases)
  * [Defining your own delimiter](#defining-your-own-delimiter)
<!-- TOC -->

## Paths and Environment Variables

On the technical side, navi uses the `directories-next` crate for rust,
which defines platform-specific locations to store the configuration files,
the cache and other types of files an application might need.

> [!TIP]
> For example, this is why cheatsheets are being stored in `~/Library/Application Support/navi` on macOS.

> [!NOTE]
> Interested on how `directories-next` works?\
> Go see their `crates.io` page: [crates.io/crates/directories-next](https://crates.io/crates/directories-next)


### The default configuration file path

During the compilation of navi, the default configuration file path is set by the `$NAVI_CONFIG` environment variable.\
If it is not set, it fallbacks to `~/.config/navi/config.yaml`.

You can check your default configuration file path with the info subcommand,
see [/docs/usage/commands/info/](/docs/usage/commands/info/README.md#default-configuration-path) for more details.

### Cheatsheets paths

Navi checks the paths in the following order until it finds a value:

1. the `$NAVI_PATH` environment variable
2. the configuration file
3. The default value of navi

#### The default cheatsheets path

By default, navi stores the cheatsheets in the `~/.local/share/navi/cheats/` directory.

You can check your default cheatsheets path with the info subcommand,
see [/docs/usage/commands/info/](/docs/usage/commands/info/README.md#default-cheatsheets-path) for more details.

#### Defining the cheatsheets path with the environment variable

The cheatsheets path can be defined using the `$NAVI_PATH` environment variable in a colon-separated list, for example:

```sh
export NAVI_PATH='/path/to/a/dir:/path/to/another/dir:/yet/another/dir'
```

#### Defining the cheatsheets path in the configuration file

You can define the cheatsheets path in the configuration file with the following syntax:

```yaml
cheats:
  paths:
    - /path/to/some/dir # on unix-like os
    - F:\\path\\to\\dir # on Windows
```

##### [DEPRECATED] - Using the `path` directive

Until `2.17.0`, you could define your cheatsheets path with the `path` directive with the following syntax:

```yaml
cheats:
  path: /path/to/some/dir
```

The directive is now deprecated and will be removed in `2.27.0`.

## Customization

### Changing colors

#### fzf color scheme

You can change the color scheme of `fzf` by overriding fzf options.

> [!NOTE]
> See [@junegunn/fzf/wiki/Color-schemes](https://github.com/junegunn/fzf/wiki/Color-schemes) and
> [#overriding-fzf-options](#overriding-fzf-options) for more details.

#### Navi colors

You can change the text color for each column of navi in the configuration file with the following syntax:

```yaml
style:
  tag:
    color: <your color for tags>
  comment:
    color: <your color for comments>
  snippet:
    color: <your color for snippets>
```

Below is an example of what to do if you'd like navi to look like the French flag:

- `config.yaml`:

  ```yaml
  style:
    tag:
      color: blue
    comment:
      color: white
    snippet:
      color: red
  ```

- The result:

  ![navi-custom-colors](https://github.com/user-attachments/assets/d80352c5-d888-43e6-927d-805a8de1a7e2)

### Resizing columns

You can change the column width of each column of navi in the configuration file with the following syntax:

```yaml
style:
  tag:
    width_percentage: <width relative to the terminal window>
    min_width: <width as number of characters>
  comment:
    width_percentage: <width relative to the terminal window>
    min_width: <width as number of characters>
  snippet:
    width_percentage: <width relative to the terminal window>
    min_width: <width as number of characters>
```

### Overriding fzf options

You can override fzf options for two different cases:

- During the cheats selection

  Navi exposes the `overrides` directive in the configuration file
  and the `NAVI_FZF_OVERRIDES` environment variable.

- During the pre-defined variable values selection

  Navi exposes the `overrides_var` directive in the configuration file
  and the `NAVI_FZF_OVERRIDES_VAR` environment variable.

For all cases, navi exposes the `FZF_DEFAULT_OPTS` environment variable.

#### Overriding during cheats selection

If you want to do the override with `--height 3`,
you can do it with the following syntax in the configuration file:

```yaml
finder:
  command: fzf
  overrides: --height 3
```

But you can also define the environment variable like this:

```bash
export NAVI_FZF_OVERRIDES='--height 3'
```

#### Overriding during values selection

If you want to do the override with `--height 3`,
you can do it with the following syntax in the configuration file:

```yaml
finder:
  command: fzf
  overrides_var: --height 3
```

But you can also define the environment variable like this:

```bash
export NAVI_FZF_OVERRIDES_VAR='--height 3'
```

#### Overriding for all cases

You can define the environment variable like this:

```bash
export FZF_DEFAULT_OPTS="--height 3"
```

> [!NOTE]
> See [@junegunn/fzf](https://github.com/junegunn/fzf#layout) for more details on `$FZF_DEFAULT_OPTS`.

## Defining your own delimiter

Navi allows you to define your own delimiter to parse the selected result for a variable in your cheats.\
It is equivalent to defining `--delimiter` used with `--column`.

You can define it as such:

```yaml
finder:
  delimiter_var: <your-regex-delimiter> ### By default the expression is \s\s+
```

> [!CAUTION]
> Defining the delimiter via the configuration file means that Navi will use this delimiter by default for
> every variable using the `--column` instruction.

You can override this configuration with the `--delimiter` instruction in the variable definition of your cheat.\
See [/docs/cheatsheet/syntax/](/docs/cheatsheet/syntax/README.md#advanced-variable-options) for more details.

