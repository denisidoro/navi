## Using it for shell scripting

For a real world scenario example, please check this [blog post](https://denisidoro.github.io/posts/cli-templates/).

Let's say you want to write a bash script that, among other things, asks the user to write the name of a git branch that should be checked out.

If you already have the [cheatsheet above](#cheatsheet-syntax), then you could write the following in your script:

```sh
navi --query "change branch" --best-match
```

**navi** will ask the user to fill all arguments needed.

If you want to set the `<branch>` beforehand in your script:

```sh
branch="master" navi --query "change branch" --best-match
```

- no interactive input will be shown
- the value for `<branch>` will be exactly the one passed as argument

If you want to filter some results for `<branch>`:

```sh
branch__query="master" navi --query "change branch" --best-match
```

- an interactive input will be shown, unless a single entry is autoselected
- the value for `<branch>` will be the one selected

If you want to select the best match for `<branch>`:

```sh
branch__best="master" navi --query "change branch" --best-match
```

- no interactive input will be shown
- the value for `<branch>` will be the one that best matches the one passed as argument
