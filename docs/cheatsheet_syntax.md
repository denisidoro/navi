Cheatsheet syntax
-----------------

* [Syntax overview](#syntax-overview)
* [Folder structure](#folder-structure)
* [Variables](#variables)
* [Advanced variable options](#advanced-variable-options)
* [Variable dependency](#variable-dependency)
* [Multiline snippets](#multiline-snippets)
* [Variable as multiple arguments](#variable-as-multiple-arguments)

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

### Folder structure

It's irrelevant how many files are used to store cheatsheets. They can be all in a single file if you wish, as long as you split them accordingly with lines starting with `%`.

### Variables

The interface prompts for variable names inside brackets (eg `<branch>`). 

Variable names should only include alphanumeric characters and `_`.

If there's a corresponding line starting with `$` for a variable, suggestions will be displayed. Otherwise, the user will be able to type any value for it.

If you hit `<tab>` the query typed will be prefered. If you hit `<enter>` the selection will be prefered.

### Advanced variable options

For lines starting with `$` you can use `---` to customize the behavior of `fzf` or how the value is going to be used:

```sh
# This will pick the 3rd column and use the first line as header
docker rmi <image_id>

# Even though "false/true" is displayed, this will print "0/1"
echo <mapped>

$ image_id: docker images --- --column 3 --header-lines 1 --delimiter '\s\s+'
$ mapped: echo 'false true' | tr ' ' '\n' --- --map "[[ $0 == t* ]] && echo 1 || echo 0"
```

The supported parameters are:
- `--prevent-extra` *(experimental)*: limits the user to select one of the suggestions;
- `--column <number>`: extracts a single column from the selected result;
- `--map <bash_code>` *(experimental)*: applies a map function to the selected variable value;

In addition, it's possible to forward the following parameters to `fzf`:
- `--multi`;
- `--header-lines <number>`;
- `--delimiter <regex>`;
- `--query <text>`;
- `--filter <text>`;
- `--header <text>`;
- `--preview <bash_code>`;
- `--preview-window <text>`.

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
jsons=($(echo "<jsons>"))
cat "${jsons[@]}"

$ jsons: find . -iname '*.json' -type f -print --- --multi
```