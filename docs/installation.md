Installation
------------

* [Installing the main binary](#installing-the-main-binary)
    * [Using Homebrew or Linuxbrew](#using-homebrew-or-linuxbrew)
    * [Using Gentoo](#using-gentoo)
    * [Using nix](#using-nix)
    * [Using cargo](#using-cargo)
    * [Using install script](#using-install-script)
    * [Downloading pre-compiled binaries](#downloading-pre-compiled-binaries)
    * [Building from source](#building-from-source)
    * [Other package managers](#other-package-managers)
* [Installing the shell widget](#installing-the-shell-widget)

### Installing the main binary

#### Using [Homebrew](http://brew.sh/) or [Linuxbrew](http://linuxbrew.sh/)

```sh
brew install navi
```

#### Using [Gentoo](https://gentoo.org/)

Be sure to [enable](https://wiki.gentoo.org/wiki/Ebuild_repository) the [GURU overlay](https://gpo.zugaina.org/Overlays/guru/app-misc/navi).

```sh
emerge -a app-misc/navi
```

#### Using [nix](https://nixos.org/)

```sh
nix-env -iA nixpkgs.navi
```

#### Using [cargo](https://github.com/rust-lang/cargo)

```bash
cargo install navi
```

#### Using install script

```bash
bash <(curl -sL https://raw.githubusercontent.com/denisidoro/navi/master/scripts/install)

# (optional) to set directories:
# SOURCE_DIR=/opt/navi BIN_DIR=/usr/local/bin bash <(curl -sL https://raw.githubusercontent.com/denisidoro/navi/master/scripts/install)
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

#### Other package managers

You can find **navi** for more package managers by clicking on the image below: 

[![Packaging status](https://repology.org/badge/vertical-allrepos/navi.svg)](https://repology.org/project/navi/versions)

Feel free to be the maintainer of **navi** for any package manager you'd like!

### Installing the shell widget

If you want to use the shell widget, add this line to your `.bashrc`-like file:
```sh
# bash
eval "$(navi widget bash)"

# zsh
eval "$(navi widget zsh)"

# fish
navi widget fish | source
```

By default, `Ctrl+G` is assigned to launching **navi**.
