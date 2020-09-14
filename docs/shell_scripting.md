Using it for shell scripting
----------------------------

Let's say you want to write a bash script that, among other things, asks the user to write the name of a git branch that should be checked out. 

If you already have the [cheatsheet above](#cheatsheet-syntax), then you could write the following in your script:
```sh
navi --query "change branch" --best-match
```

**navi** will ask the user to fill all arguments needed. 

If you want to set the `<branch>` beforehand in your script, you could then write:
```sh
branch="master" navi --query "change branch" --best-match
```
