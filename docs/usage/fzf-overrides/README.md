```sh
# if you want to override only when selecting snippets
navi --fzf-overrides '--height 3'

# if you want to override only when selecting argument values
navi --fzf-overrides-var '--height 3'

# if you want to override for all cases
FZF_DEFAULT_OPTS="--height 3" navi
```