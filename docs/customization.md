Customization
-------------

* [Changing colors](#changing-colors)
* [Resizing columns](#resizing-columns)
* [Overriding fzf options](#overriding-fzf-options)

### Changing colors

You can change the [color scheme](https://github.com/junegunn/fzf/wiki/Color-schemes) by [overriding fzf options](#overriding-fzf-options).

In addition, you can change the text color for each column by setting the following environment variables:
- `$NAVI_TAG_COLOR`
- `$NAVI_COMMENT_COLOR`
- `$NAVI_SNIPPET_COLOR`

The values go [from 0 to 15](https://github.com/redox-os/termion/blob/189222555ef92a29de366f96d2a067b3a920fc24/src/color.rs#L62-L77).

For example, if you add the following your .bashrc-like file, the comment color will be yellow:
```sh
export NAVI_COMMENT_COLOR=3
```

### Resizing columns

You can change the column widths by setting the following environment variables:
- `$NAVI_TAG_WIDTH`
- `$NAVI_COMMENT_WIDTH`

The values go from 0 to 100 and represent the percentage the column will occupy.

For example, if you add the following your .bashrc-like file, the comment column will be very small:
```sh
export NAVI_COMMENT_WIDTH=5
```

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