# The info subcommands of navi

Navi exposes information about its default values or examples for you to use.

<!-- TOC -->
* [The info subcommands of navi](#the-info-subcommands-of-navi)
  * [Commands Reference](#commands-reference)
  * [Default configuration information](#default-configuration-information)
    * [Default configuration path](#default-configuration-path)
    * [Example configuration file](#example-configuration-file)
  * [Default cheatsheets path](#default-cheatsheets-path)
<!-- TOC -->

## Commands Reference

| Command             | Description |
|---------------------|-------------|
| config-path         |             |
| cheats-path         |             |
| default-config-path |             |
| default-cheats-path |             |
| config-example      |             |
| cheats-example      |             |

## Default configuration information

### Default configuration path

Navi exposes its default configuration path with:

```sh
navi info config-path
```

> [!NOTE]
> See [/docs/configuration/](/docs/configuration/README.md#the-default-configuration-file-path) for more details on how the default configuration path is defined.

### Example configuration file

Navi lets you get an example configuration file with:

```sh
navi info config-example
```

> [!NOTE]
> You can retrieve this file at the following address: [/docs/examples/configuration/config-example.yaml](/docs/examples/configuration/config-example.yaml)

For example, you can use this command to create the default configuration file,
if not already present:

```sh
navi info config-example > "$(navi info config-path)"
```

## Default cheatsheets path

Navi exposes its default cheatsheets path with:

```sh
navi info cheats-path
```

> [!NOTE]
> See [/docs/configuration/](/docs/configuration/README.md#the-default-cheatsheets-path) for more details on how the cheatsheets path is defined.

