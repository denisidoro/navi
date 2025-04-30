# The FZF Overrides of Navi

Navi allows you to override certain parts of FZF in multiple ways.

<!-- TOC -->
* [The FZF Overrides of Navi](#the-fzf-overrides-of-navi)
  * [Command line arguments](#command-line-arguments)
  * [Environment variables](#environment-variables)
<!-- TOC -->

## Command line arguments

Navi allows you to use command line arguments in order to override fzf values:

```sh
# if you want to override only when selecting snippets
navi --fzf-overrides '--height 3'

# if you want to override only when selecting argument values
navi --fzf-overrides-var '--height 3'
```

## Environment variables

Navi allows you to use environment variables in order to override fzf values.

```bash
# if you want to override for all cases
FZF_DEFAULT_OPTS="--height 3" navi
```
