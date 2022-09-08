## Customization

- [Changing colors](#changing-colors)
- [Resizing columns](#resizing-columns)
- [Overriding fzf options](#overriding-fzf-options)

### Changing colors

You can change the [color scheme](https://github.com/junegunn/fzf/wiki/Color-schemes) by [overriding fzf options](#overriding-fzf-options).

In addition, you can change the text color for each column by properly configuring _navi_'s `config.yaml`. Please check `navi --help` for more instructions.

### Resizing columns

You can change the column widths by properly configuring _navi_'s `config.yaml`. Please check `navi --help` for more instructions.

### Overriding fzf options

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
