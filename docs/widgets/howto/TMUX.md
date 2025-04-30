# Tmux widget

You can use **navi** as a [Tmux](https://github.com/tmux/tmux/wiki) widget to reach your Vim commands,
often used SQL queries, etc. in any command-line app even in SSH sessions.

<!-- TOC -->
* [Tmux widget](#tmux-widget)
  * [Keybinding navi](#keybinding-navi)
  * [Example cheatsheet](#example-cheatsheet)
<!-- TOC -->

## Keybinding navi

To be able to open navi via <kbd> prefix + C-g </kbd>, you need to add the following lines
to your Tmux configuration file.

```sh
bind-key -N "Open Navi (cheat sheets)" -T prefix C-g split-window \
  "$SHELL --login -i -c 'navi --print | head -n 1 | tmux load-buffer -b tmp - ; tmux paste-buffer -p -t {last} -b tmp -d'"
```

## Example cheatsheet

Here is an example cheatsheet to use inside Tmux:

```sh
% vim 

# Quit without save
qa!

# Delete a paragraph
normal dap

# Generate sequence of numbers
put =range(<start>, <stop>)

% postgresql

# Describe table columns in `psql` or `pgcli`
select 
   table_name, 
   column_name, 
   data_type 
from 
   information_schema.columns
where 
   table_name = '<table>';
```
