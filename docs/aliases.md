## Aliases

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
