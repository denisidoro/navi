## Cheatsheet syntax

- [Syntax overview](#syntax-overview)
- [Folder structure](#folder-structure)
- [Variables](#variables)
- [Advanced variable options](#advanced-variable-options)
- [Variable dependency](#variable-dependency)
- [Multiline snippets](#multiline-snippets)
- [Variable as multiple arguments](#variable-as-multiple-arguments)
- [Aliases](#aliases)

### Syntax overview

Cheatsheets are described in `.cheat` files that look like this:

```sh
% git, code

# Change branch
git checkout <branch>

$ branch: git branch | awk '{print $NF}'
```

Lines starting with:

- `%`: determine the start of a new cheatsheet and should contain tags
- `#`: should be descriptions of commands
- `;`: are ignored. You can use them for metacomments
- `$`: should contain commands that generate a list of possible values for a given argument [:information_source:](#variables)
- `@`: should contain tags whose associated cheatsheet you want to base on [:information_source:](#extending-cheatsheets)

All the other non-empty lines are considered as executable commands.

Tip: if you are editing cheatsheets in Visual Studio Code, you could enable syntax highlighting
by installing [this extension](https://marketplace.visualstudio.com/items?itemName=yanivmo.navi-cheatsheet-language).

### Variables

The interface prompts for variable names inside brackets (eg `<branch>`).

Variable names should only include alphanumeric characters and `_`.

If there's a corresponding line starting with `$` for a variable, suggestions will be displayed. Otherwise, the user will be able to type any value for it.

If you hit `<tab>` the query typed will be preferred. If you hit `<enter>` the selection will be preferred.

### Advanced variable options

For lines starting with `$` you can use `---` to customize the behavior of `fzf` or how the value is going to be used:

```sh
# This will pick the 3rd column and use the first line as header
docker rmi <image_id>

# Even though "false/true" is displayed, this will print "0/1"
echo <mapped>

$ image_id: docker images --- --column 3 --header-lines 1 --delimiter '\s\s+'
$ mapped: echo 'false true' | tr ' ' '\n' --- --map "grep -q t && echo 1 || echo 0"
```

The supported parameters are:

- `--column <number>`: extracts a single column from the selected result
- `--map <bash_code>`: _(experimental)_ applies a map function to the selected variable value
- `--prevent-extra`: _(experimental)_ limits the user to select one of the suggestions
- `--fzf-overrides <arg>`: _(experimental)_ applies arbitrary `fzf` overrides
- `--expand`: _(experimental)_ converts each line into a separate argument

In addition, it's possible to forward the following parameters to `fzf`:

- `--multi`
- `--header-lines <number>`
- `--delimiter <regex>`
- `--query <text>`
- `--filter <text>`
- `--header <text>`
- `--preview <bash_code>`
- `--preview-window <text>`

### Variable dependency

The command for generating possible inputs can implicitly refer other variables by using the `<varname>` syntax:

```sh
# Should print /my/pictures/wallpapers
echo "<wallpaper_folder>"

$ pictures_folder: echo "/my/pictures"
$ wallpaper_folder: echo "<pictures_folder>/wallpapers"
```

If you want to make dependencies explicit, you can use the `$varname` syntax:

```sh
# If you select "hello" for <x>, the possible values of <y> will be "hello foo" and "hello bar"
echo <x> <y>

# If you want to ignore the contents of <x> and only print <y>
: <x>; echo <y>

$ x: echo "hello hi" | tr ' ' '\n'
$ y: echo "$x foo;$x bar" | tr ';' '\n'
```

### Extending cheatsheets

With the `@ same tags from other cheatsheet` syntax you can reuse the same variable in multiple cheatsheets.

```sh
% dirs, common

$ pictures_folder: echo "/my/pictures"

% wallpapers
@ dirs, common

# Should print /my/pictures/wallpapers
echo "<pictures_folder>/wallpapers"

% screenshots
@ dirs, common

# Should print /my/pictures/screenshots
echo "<pictures_folder>/screenshots"
```

### Multiline snippets

Commands may be multiline:

```sh
# This will output "foo\nyes"
echo foo
true \
   && echo yes \
   || echo no
```

### Variable as multiple arguments

```sh
# This will result into: cat "file1.json" "file2.json"
cat <jsons>

$ jsons: find . -iname '*.json' -type f -print --- --multi --expand
```
### Aliases

**navi** doesn't have support for aliases as first-class citizens at the moment.

However, it is trivial to create aliases using **navi** + a few conventions.

For example, suppose you decide to end some of your commands with `:: <some_alias>`:

```bash
% aliases

# This is one command :: el
echo lorem ipsum

# This is another command :: ef
echo foo bar
```

Then, if you use **navi** as a [shell scripting tool](shell_scripting.md), you could add something similar to this in your `.bashrc`-like file:

```bash
navialias() {
    navi --query ":: $1" --best-match
}

alias el="navialias el"
alias ef="navialias ef"
```

If you don't want to use these conventions, you can even add full comments in your aliases:

```bash
navibestmatch() {
    navi --query "$1" --best-match
}

alias el="navibestmatch 'This is one command'"
alias ef="navibestmatch 'This is another command'"
```
