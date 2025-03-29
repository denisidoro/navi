# Installation of navi

Navi can be installed by multiple ways, here is a list of referenced ways to do so.

> [!CAUTION]
> Navi, as of now, has only two official builds, the released binaries on GitHub
> and the published package on brew.
> 
> All the other packages are community-maintained.

## Using package managers

### Homebrew

```sh
brew install navi
```

> [!NOTE]
> See [brew.sh](https://brew.sh/) for more details.

### Using Gentoo

> [!WARNING]
> You need to enable the GURU overlay for the instructions below to work correctly.
> 
> For more details see:
> 
> - [wiki.gentoo.org/wiki/Ebuild_repository](https://wiki.gentoo.org/wiki/Ebuild_repository)
> - [gpo.zugaina.org/Overlays/guru/app-misc/navi](https://gpo.zugaina.org/Overlays/guru/app-misc/navi).

```sh
emerge -a app-misc/navi
```

> [!NOTE]
> See [Gentoo.org](https://gentoo.org/) for more details.

### Using Pacman

```sh
pacman -S navi
```

> [!NOTE]
> See [wiki.archlinux.org/title/Pacman](https://wiki.archlinux.org/title/Pacman) for more details.

### Using nix

```sh
nix-env -iA nixpkgs.navi
```

> [!NOTE]
> See [nixos.org](https://nixos.org/) for more details

### Using Cargo

```bash
cargo install --locked navi
```

> [!NOTE]
> See [@rust-lang/cargo](https://github.com/rust-lang/cargo) for more details.

### Using Chocolatey

```bash
choco install navi
```

> [!CAUTION]
> You currently need to create the config file `$env:USERPROFILE\AppData\Roaming\navi\config.yaml`
> and define the `shell.command` directive as `powershell`.
> 
> ```yaml
> shell:
>   command: powershell
> ```

> [!NOTE]
> See [community.chocolatey.org](https://community.chocolatey.org) for more details.

## Using the installation script

Navi has an installation script ready for you to use, you can call it like this:

```bash
bash <(curl -sL https://raw.githubusercontent.com/denisidoro/navi/master/scripts/install)
```

If you need to define the directory for the binary, you can call it like this:

```bash
BIN_DIR=/usr/local/bin bash <(curl -sL https://raw.githubusercontent.com/denisidoro/navi/master/scripts/install)
```

## Downloading pre-compiled binaries

With each release, we try our best to build and publish a binary for each
supported platform, you can find them here:
[@denisidoro/navi/releases/latest](https://github.com/denisidoro/navi/releases/latest)

What you need to do is:

- to download the binary corresponding to the version you want to install
- to extract the content of the archive to your `$PATH`

## Building from source

You can also build navi from source, it's mainly used by contributors to
test their modifications can be used by end users who want to build their own version.

- You need to clone the repository:

    ```bash
    git clone https://github.com/denisidoro/navi && cd navi
    ```

- Call `make`

    ```bash
    make install
    ```

You can specify the binary directory with:

```bash
make BIN_DIR=/usr/local/bin install
```

## Compile time environment variables

**navi** supports environment variables at compile time that will modify the behavior of navi at runtime, they are:

| Environment variable | Description                                                 |
|----------------------|-------------------------------------------------------------|
| `NAVI_PATH`          | This defines the default path used by navi for cheatsheets. |
| `NAVI_CONFIG`        | This defines the default configuration file used by navi.   |

## Other package managers

You can find **navi** for more package managers by clicking on the image below:

[![Packaging status](https://repology.org/badge/vertical-allrepos/navi.svg)](https://repology.org/project/navi/versions)

Feel free to be the maintainer of **navi** for any package manager you'd like!
