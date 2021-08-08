Config file
-----------------

* [Example](#example)
* [Location](#location)
* [Creating the file](#creating-the-file)

### Example

An example config can be found by running:
```sh
navi info config-example
```

You can also read it online by clicking [here](./config_file_example.yaml).

### Location

Run the following command to check where the config file is/should be located:
```sh
navi info config-path
```

### Creating the file

Run the following command to generate a config file with the default parameters:
```sh
navi info config-example > "$(navi info config-path)"
```
