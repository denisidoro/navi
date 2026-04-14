# Navi widgets

You want to launch Navi with a shortcut?\
Widgets are here for you!

Widgets are 3rd-party contributions and integrates Navi with 3rd-party software such as shells.

## List of shell widgets

| Shell      | Navi support       |
|------------|--------------------|
| Bash       | :white_check_mark: |
| Fish       |                    |
| Zsh        |                    |
| NuShell    | :white_check_mark: |
| PowerShell | :white_check_mark: |

## PowerShell Widget

- Removal

```powershell
Remove-Module navi.plugin
```

## Other widgets

- Tmux
- Vim


### Installing the shell widget

If you want to install it, add this line to your `.bashrc`-like file:

```sh
# bash
eval "$(navi widget bash)"

# zsh
eval "$(navi widget zsh)"

# fish
navi widget fish | source

# elvish
eval (navi widget elvish | slurp)

# xonsh
# xpip install xontrib-navi # ← run in your xonsh session to install xontrib
xontrib load navi # ← add to your xonsh run control file
```

#### Nushell

Due to Nushell's [unique design](https://www.nushell.sh/book/thinking_in_nu.html#think-of-nushell-as-a-compiled-language), it is not possible to `eval` a piece of code dynamically like in other shells therefore the integration process is a bit more involved. Here is an example:
1. run `^navi widget nushell | save ($nu.default-config-dir | path join "navi-integration.nu")`
2. add the following lines to `config.nu`:
    ```nushell
    source ($nu.default-config-dir | path join "navi-integration.nu")
    ```


By default, `Ctrl+G` is assigned to launching **navi** (in xonsh can be customized with `$X_NAVI_KEY`, see [xontrib-navi](https://github.com/eugenesvk/xontrib-navi) for details).

To customize the keybinding, use the `--key` flag with your shell's native keybinding syntax:

```sh
# zsh — bind to Ctrl+Space
eval "$(navi widget zsh --key '^ ')"

# bash — bind to Ctrl+Space
eval "$(navi widget bash --key '\C- ')"

# fish — bind to Ctrl+Space
navi widget fish --key '\c ' | source
```

If you want to further customize the widget behavior (e.g., change the **navi** flags), you can:

1. run, e.g., `navi widget bash` in your terminal
2. copy the output
3. paste the output in your `.bashrc`-like file
4. edit the contents accordingly
