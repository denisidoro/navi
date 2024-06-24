## Installation

- [Installing the main binary](#installing-the-main-binary)
  - [Using Homebrew](#using-homebrew)
  - [Using Gentoo](#using-gentoo)
  - [Using nix](#using-nix)
  - [Using cargo](#using-cargo)
  - [Using install script](#using-install-script)
  - [Downloading pre-compiled binaries](#downloading-pre-compiled-binaries)
  - [Building from source](#building-from-source)
  - [Other package managers](#other-package-managers)
- [Installing the shell widget](#installing-the-shell-widget)

### Installing the main binary

#### Using [Homebrew](http://brew.sh/)

```sh
brew install navi
```

#### Using [Gentoo](https://gentoo.org/)

Be sure to [enable](https://wiki.gentoo.org/wiki/Ebuild_repository) the [GURU overlay](https://gpo.zugaina.org/Overlays/guru/app-misc/navi).

```sh
emerge -a app-misc/navi
```

#### Using [pacman](https://wiki.archlinux.org/title/Pacman)

```sh
pacman -S navi
```

#### Using [nix](https://nixos.org/)

```sh
nix-env -iA nixpkgs.navi
```

#### Using [cargo](https://github.com/rust-lang/cargo)

```bash
cargo install --locked navi
```

#### Using [choco](https://community.chocolatey.org/packages/navi)

For Windows user, using powershell

1. Install package via choco
   ```bash
   choco install navi
   ```
2. Create `$env:USERPROFILE\AppData\Roaming\navi\config.yaml` and override `shell.command` as per [config_file_example.yaml](./config_file_example.yaml)

   ```
   style:
     tag:
       color: cyan
     comment:
       color: grey
     snippet:
       color: white

   shell:
     command: powershell
   ```

   Remark: Above example also adds custom colors for better readability in case you use standard blue for your Powershell

#### Using install script

```bash
bash <(curl -sL https://raw.githubusercontent.com/denisidoro/navi/master/scripts/install)

# (optional) to set directories:
# BIN_DIR=/usr/local/bin bash <(curl -sL https://raw.githubusercontent.com/denisidoro/navi/master/scripts/install)
```

#### Downloading pre-compiled binaries

- download the correct binary [here](https://github.com/denisidoro/navi/releases/latest)
- extract the content to your `$PATH`

#### Building from source

```bash
git clone https://github.com/denisidoro/navi ~/.navi
cd ~/.navi
make install

# (optional) to set the install directory:
# make BIN_DIR=/usr/local/bin install
```

##### Compile time environment variables

**navi** supports environment variables at compile time that modify the behavior of the binary at runtime:

- `NAVI_PATH` (directory path value): If the `cheats` directory in the user's directory does not exist, **navi** uses this path (if it exists), as a fallback location to look for cheat files. Use case: system-wide installed, shared used cheatsheets folder.
- `NAVI_CONFIG` (file path value): If the `config.yaml` file in the user's directory does not exist, **navi** uses this path (if it exists), as a fallback location to look for a configuration file. Use case: system-wide installed, shared used configuration file.

#### Other package managers

You can find **navi** for more package managers by clicking on the image below:

[![Packaging status](https://repology.org/badge/vertical-allrepos/navi.svg)](https://repology.org/project/navi/versions)

Feel free to be the maintainer of **navi** for any package manager you'd like!

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

There's currently no way to customize the widget behavior out-of-the-box. If you want to change the keybinding or the **navi** flags used by the widget, please:

1. run, e.g., `navi widget bash` in your terminal
2. copy the output
3. paste the output in your `.bashrc`-like file
4. edit the contents accordingly
