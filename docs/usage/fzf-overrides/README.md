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