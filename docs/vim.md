## vim

If you want syntax highlighting support for Navi in Vim, add those syntax rules
to your syntax files such as at `$VIMRUNTIME/syntax/navi.vim`.
The rules are defined based on the [Cheatsheet syntax](cheatsheet_syntax.md).

```vim
syntax match Comment "\v^;.*$"
syntax match Statement "\v^\%.*$"
syntax match Operator "\v^\#.*$"
syntax match String "\v\<.*\>"
syntax match String "\v^\$.*$"
```
