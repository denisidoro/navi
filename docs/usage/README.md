# The usage of Navi

Navi can be used in multiple ways

#### Defining the cheatsheets path at runtime

You can define the paths to use for cheatsheets at runtime using the `--path` parameter and a colon-separated paths list

For example, if we want to search for cheatsheets in `/some/dir` and in `/other/dir`:

```sh
navi --path '/some/dir:/other/dir'
```

## Logging

The log file will be created under the same directory where the configuration file is located.\
You can use the `RUST_LOG` environment variable to set the log level.

For example, to have the logging in debug mode when running navi:

```bash
RUST_LOG=debug navi
```

> [!NOTE]
> If the directory of the configuration file doesn't exist, no log file
> is going to be created.
