# Navi and shell scripting

You can use Navi with shell scripting.

The following blog post gives you an example of a real world scenario: [denisidoro.github.io/posts/cli-templates/](https://denisidoro.github.io/posts/cli-templates/)

## Simply calling a cheat

Below is an example on how to call a cheat from within navi:

```sh
navi --query "change branch" --best-match
```

> [!NOTE]
> Navi will ask the user to fill all arguments/variables needed.

## Defining variables while calling

If you want to set the `<branch>` beforehand in your script, you can do as follows:

```sh
branch="master" navi --query "change branch" --best-match
```

Navi will not show any interactive input and `<branch>` will be exactly the one defined while calling.

## Filtering results for a variable

If you want to filter some results for `<branch>`, you can do as follows:

```sh
branch__query="master" navi --query "change branch" --best-match
```

Navi will show any interactive input, unless a single entry is automatically selected and
the value for `<branch>` will be the one selected by the user.

## Selecting the best match for a variable

If you want to select the best match for `<branch>`, you can do as follows:

```sh
branch__best="master" navi --query "change branch" --best-match
```

Navi will not show any interactive input, and the value for `<branch>` will be the one that
best matches the value passed as argument.
