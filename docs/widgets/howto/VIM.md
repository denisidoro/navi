# Vim widget

<!-- TOC -->
* [Vim widget](#vim-widget)
  * [Syntax Highlighting](#syntax-highlighting)
<!-- TOC -->

## Syntax Highlighting

If you want syntax highlighting support for Navi in Vim, you need to
add those syntax rules to your syntax files such as at `$VIMRUNTIME/syntax/navi.vim`.

The rules are defined based on the [Cheatsheet syntax](/docs/cheatsheet/syntax/README.md).

Here is an example:

```vim
syntax match Comment "\v^;.*$"
syntax match Statement "\v^\%.*$"
syntax match Operator "\v^\#.*$"
syntax match String "\v\<.{-}\>"
syntax match String "\v^\$.*$"
```
