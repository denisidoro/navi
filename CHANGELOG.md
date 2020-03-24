## [v2.3.1](https://github.com/denisidoro/navi/releases/tag/v2.3.1) - 2020-03-22

### :bug: Fixes
- [`8c374`](https://github.com/denisidoro/navi/commit/8c374) Panic on `navi repo ` commands because of invalid `--preview-window` syntax ([#302](https://github.com/denisidoro/navi/issues/302))
- [`c12c2`](https://github.com/denisidoro/navi/commit/c12c2) Issue of terminal programs like fzf not recognizing stdin as terminal ([#267](https://github.com/denisidoro/navi/issues/267))
- [`1986d`](https://github.com/denisidoro/navi/commit/1986d) Fish widget changing tty settings ([#296](https://github.com/denisidoro/navi/issues/296))


## [v2.3.0](https://github.com/denisidoro/navi/releases/tag/v2.3.0) - 2020-03-22

### :zap: New features
- [`10f02`](https://github.com/denisidoro/navi/commit/10f02) Accept more fzf parameters after `---` ([#294](https://github.com/denisidoro/navi/issues/294))

### :computer: Code quality
- [`64045`](https://github.com/denisidoro/navi/commit/64045) Remove need for lifetime on `FzfOpt` ([#293](https://github.com/denisidoro/navi/issues/293))


## [v2.2.0](https://github.com/denisidoro/navi/releases/tag/v2.2.0) - 2020-03-22

### :zap: New features
- [`0b91a`](https://github.com/denisidoro/navi/commit/0b91a) Add `navi repo browse` ([#280](https://github.com/denisidoro/navi/issues/280))
- [`a51a5`](https://github.com/denisidoro/navi/commit/a51a5) De-duplicate cheatsheets
- [`b49ee`](https://github.com/denisidoro/navi/commit/b49ee) Make `--allow-extra` default and add `--prevent-extra` ([#281](https://github.com/denisidoro/navi/issues/281)`

### :bug: Fixes
- [`2ca1d`](https://github.com/denisidoro/navi/commit/2ca1d) `--column` with `--multi` selecting first line only ([#290](https://github.com/denisidoro/navi/issues/290))
- [`a0bba`](https://github.com/denisidoro/navi/commit/a0bba) Minor fixes ([#285](https://github.com/denisidoro/navi/issues/285))

### :computer: Code quality
- [`2ca1d`](https://github.com/denisidoro/navi/commit/2ca1d) Code refactors ([#290](https://github.com/denisidoro/navi/issues/290))


## [v2.1.3](https://github.com/denisidoro/navi/releases/tag/v2.1.3) - 2020-03-22

### :bug: Fixes
- [`98658`](https://github.com/denisidoro/navi/commit/98658) Fix OpenSSL dependency


## [v2.1.1](https://github.com/denisidoro/navi/releases/tag/v2.1.1) - 2020-03-22

### :zap: New features
- [`92cf1`](https://github.com/denisidoro/navi/commit/92cf1) Allow cheatsheet repos with `@` ([#272](https://github.com/denisidoro/navi/issues/272))

### :computer: Code quality
- [`672fc`](https://github.com/denisidoro/navi/commit/672fc) Improve README ([#271](https://github.com/denisidoro/navi/issues/271))


## [v2.1.0](https://github.com/denisidoro/navi/releases/tag/v2.1.0) - 2020-03-22

### :bangbang: Breaking changes
- [`de168`](https://github.com/denisidoro/navi/commit/de168) `.cheat` files aren't bundled with the release anymore. They should be downloaded when running `navi` for the first time ([#258](https://github.com/denisidoro/navi/issues/258)
- [`de168`](https://github.com/denisidoro/navi/commit/de168) Sourcing shell widgets is different now. Please check README.md for more info. ([#258](https://github.com/denisidoro/navi/issues/258)))

### :zap: New features
- [`de168`](https://github.com/denisidoro/navi/commit/de168) Initial cheat repo support ([#258](https://github.com/denisidoro/navi/issues/258))
- [`3fa14`](https://github.com/denisidoro/navi/commit/3fa14) Allow multiline commands ([#262](https://github.com/denisidoro/navi/issues/262))
- [`b8097`](https://github.com/denisidoro/navi/commit/b8097) Add `--allow-extra` option
- [`e99e1`](https://github.com/denisidoro/navi/commit/e99e1) Add variable name to prompt with suggestions

### :computer: Code quality
- [`bbbd5`](https://github.com/denisidoro/navi/commit/bbbd5) Remove TODOs


## [v2.0.11](https://github.com/denisidoro/navi/releases/tag/v2.0.11) - 2020-03-22

### :zap: New features
- [`f8e8d`](https://github.com/denisidoro/navi/commit/f8e8d) Minor theme update ([#246](https://github.com/denisidoro/navi/issues/246))
- [`c5b3b`](https://github.com/denisidoro/navi/commit/c5b3b) Add `navi fn url::open` ([#241](https://github.com/denisidoro/navi/issues/241))

### :computer: Code quality
- [`5af27`](https://github.com/denisidoro/navi/commit/5af27) Mention fzf in README.md ([#240](https://github.com/denisidoro/navi/issues/240))


## [v2.0.8](https://github.com/denisidoro/navi/releases/tag/v2.0.8) - 2020-03-22

### :bug: Fixes
- [`e32e0`](https://github.com/denisidoro/navi/commit/e32e0) Delimiter ([#239](https://github.com/denisidoro/navi/issues/239))
- [`8d0c8`](https://github.com/denisidoro/navi/commit/8d0c8) Crash when user enters a command that doesn't match any cheats


## [v2.0.6](https://github.com/denisidoro/navi/releases/tag/v2.0.6) - 2020-03-22

### :zap: New features
- [`ce9c2`](https://github.com/denisidoro/navi/commit/ce9c2) Allow overriding FZF variables ([#235](https://github.com/denisidoro/navi/issues/235))


## [v2.0.5](https://github.com/denisidoro/navi/releases/tag/v2.0.5) - 2020-03-22

### :bug: Fixes
- [`a1dea`](https://github.com/denisidoro/navi/commit/a1dea) Path issues ([#231](https://github.com/denisidoro/navi/issues/231))


## [v2.0.4](https://github.com/denisidoro/navi/releases/tag/v2.0.4) - 2020-03-22

### :bug: Fixes
- [`77079`](https://github.com/denisidoro/navi/commit/77079) Handle relative symlinks ([#225](https://github.com/denisidoro/navi/issues/225))


## [v2.0.3](https://github.com/denisidoro/navi/releases/tag/v2.0.3) - 2020-03-22

### :bug: Fixes
- [`06eae`](https://github.com/denisidoro/navi/commit/06eae) Minor fixes ([#221](https://github.com/denisidoro/navi/issues/221))


## [v2.0.2](https://github.com/denisidoro/navi/releases/tag/v2.0.2) - 2020-03-22

### :zap: New features
- [`4a1bb`](https://github.com/denisidoro/navi/commit/4a1bb) Add support for metacomment ([#216](https://github.com/denisidoro/navi/issues/216))

### :bug: Fixes
- [`75139`](https://github.com/denisidoro/navi/commit/75139) Problems retrieving cheats folder ([#220](https://github.com/denisidoro/navi/issues/220))
- [`1cbb1`](https://github.com/denisidoro/navi/commit/1cbb1) Allow custom dirs when running make ([#214](https://github.com/denisidoro/navi/issues/214))

### :computer: Code quality
- [`aefc1`](https://github.com/denisidoro/navi/commit/aefc1) Rename CI pipeline ([#218](https://github.com/denisidoro/navi/issues/218))


## [v2.0.0](https://github.com/denisidoro/navi/releases/tag/v2.0.0) - 2020-03-22

### :sparkles: Context
- This is a complete rewrite of the project in [Rust](https://www.rust-lang.org)

### :bangbang: Breaking changes
- Removed support for `--col-widths` and `--col-colors`
- Removed support for  variable dependency in variable suggestions
- Removed support for `navi search <cmd>` #204
- Removed support for copying snippet to clipboard
- Removed support for `--no-interpolation`
- Removed support for `navi best <cmd> <args>...`
- Removed support for `navi fn url::open <args>`


## [v1.0.0](https://github.com/denisidoro/navi/releases/tag/v1.0.0) - 2020-03-22

### :sparkles: Context
- This is the same as [v0.18.3](https://github.com/denisidoro/navi/releases/tag/v0.18.3). It's tagged as `v1.0.0` simply to indicate stability. 


## [v0.18.3](https://github.com/denisidoro/navi/releases/tag/v0.18.3) - 2020-03-22

### :sparkles: Context
- Last release for the bash implementation
