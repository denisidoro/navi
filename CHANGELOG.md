## [v2.3.1](releases/tag/v2.3.1) 2020-03-22
- Fix --preview-window for repo browse (#302)
- fixed issue of terminal programs like fzf not recognizing stdin as terminal (#267)
- Fix fish widget changing tty settings (#296)


## [v2.3.0](releases/tag/v2.3.0) 2020-03-22
- Forward many more parameters to fzf after `---` (#294)
- Remove need for lifetime on FzfOpts (#293)


## [v2.2.0](releases/tag/v2.2.0) 2020-03-22
- Fix --multi with --column + refactors (#290)
- Small refactors (#289)
- Minor fixes (#285)
- Uses a hashset to de-duplicate results
- Remove travis (#284)
- Make --allow-extra default (#281)
- Mention navi repo browse (#282)
- Add `navi repo browse` (#280)


## [v2.1.3](releases/tag/v2.1.3) 2020-03-22
- fix openssl dependency and introduce travis
- Add version to assets (#276)
- Remove OpenSSL dependency (#275)


## [v2.1.1](releases/tag/v2.1.1) 2020-03-22
- Allow repos with @ (#272)
- Improve README (#271)


## [v2.1.0](releases/tag/v2.1.0) 2020-03-22
- Initial cheat repo support (#258)
- Add a sheet for HTTPie
- Remove TODO
- Static newline regex (#264)
- Allow multiline commands (#262)
- wip (#261)
- add `--allow-extra` option
- add variable name to prompt with suggestions


## [v2.0.11](releases/tag/v2.0.11) 2020-03-22
- Update javascript cheats
- Reuse variables (#252)
- Minor theme change (#246)
- Prevent panic when there are no cheats (#244)
- :sparkles: Add basic firebase cheatsheet
- Add apt cheat
- Add files via upload
- Add support for variable dependency (#242)
- Add support for navi fn (#241)
- Mention fzf (#240)


## [v2.0.8](releases/tag/v2.0.8) 2020-03-22
- Fix delimiter (#239)
- Fix crash when user enters a command that doesn't match any cheats


## [v2.0.6](releases/tag/v2.0.6) 2020-03-22
- Allow overriding FZF variables (#235)


## [v2.0.5](releases/tag/v2.0.5) 2020-03-22
- Fix path issues (#231)


## [v2.0.4](releases/tag/v2.0.4) 2020-03-22
- Handle relative symlinks (#225)


## [v2.0.3](releases/tag/v2.0.3) 2020-03-22
- Minor fixes (#221)


## [v2.0.2](releases/tag/v2.0.2) 2020-03-22
- Defensive cheats folder (#220)
- Be more defensive with /dev/tty (#219)
- Rename CI pipeline (#218)
- Update README (#217)
- Allow custom dirs when running make (#214)
- Add support for metacomment (#216)


## [v2.0.0](releases/tag/v2.0.0) 2020-03-22
- Fixed OOB panic in replace_variables_from_snippet()
- Fix use in subshell (#213)
- Fix best command (#210)
- Follow symlinks for exe (#209)
- Rust implementation (#197)


## [v1.0.0](releases/tag/v1.0.0) 2020-03-22
- Added a Command line(bash) cheat sheet


## [v0.18.3](releases/tag/v0.18.3) 2020-03-22
- Limit dep order filter
- Hello this PR supports multiple selection for files in `git diff` and `git add`


## [v0.18.2](releases/tag/v0.18.2) 2020-03-22
- Revert "Correctly escape backslashes" (#181)
- Add experimental funding button (#178)


## [v0.18.1](releases/tag/v0.18.1) 2020-03-22
- Correctly escape backslashes (#177)


## [v0.18.0](releases/tag/v0.18.0) 2020-03-22
- Add support for multiple choice (#175)


## [v0.17.1](releases/tag/v0.17.1) 2020-03-22
- Fish: fix widget (#174)


## [v0.17.0](releases/tag/v0.17.0) 2020-03-22
- Fix preview in fish shell
- Create cf.cheat (#166)


## [v0.16.0](releases/tag/v0.16.0) 2020-03-22
- Add bindings for C-j and C-k (#165)


## [v0.15.5](releases/tag/v0.15.5) 2020-03-22
- Remove dep order from outputs (#163)
- Fix "navi best" command (#162)
- Meta: Add issue templates and minor fix in readme


## [v0.15.3](releases/tag/v0.15.3) 2020-03-22
- Expand tildes in paths (#160)
- Add cheats for Concourse FLY
- Sync with master
- Add option to change colors (#147)
- Use 2+ spaces as column separator (#156)
- Add keybinding to copy snippet (#155)
- Update-git-cheats
- Update demo gif (#143)


## [v0.15.2](releases/tag/v0.15.2) 2020-03-22


## [v0.15.1](releases/tag/v0.15.1) 2020-03-22


## [v0.15.0](releases/tag/v0.15.0) 2020-03-22
- Change license to Apache 2.0 (#141)
- Additional cheat files (#139)
- Refactor list visualization (#138)


## [v0.14.3](releases/tag/v0.14.3) 2020-03-22
- Add new cheats (#136)
- ## What does this do


## [v0.14.2](releases/tag/v0.14.2) 2020-03-22
- Correctly escape backslashes (#132)


## [v0.14.1](releases/tag/v0.14.1) 2020-03-22
- fix bug about variable suggestion
- Create cheats yum


## [v0.14.0](releases/tag/v0.14.0) 2020-03-22
- 0.14.0
- Add more cheatsheets
- navi fish plugin with ability to edit commandline
- Add kubectl config get-contexts
- doc(#121): Clarify that oh-my-zsh does not add navi to PATH.
- Cheatsheet for gpg
- Minor changes (#122)


## [v0.12.0](releases/tag/v0.12.0) 2020-03-22
- Refactor string escaping code (#114)
- Fix read only error
- Add possibility to pass argument values (#110)
- [src]: Add condition to check regex for arg/escaped_arg (#109)


## [v0.11.1](releases/tag/v0.11.1) 2020-03-22
- Add fallback to tac (#108)
- Install instruction updates (#106)


## [v0.11.0](releases/tag/v0.11.0) 2020-03-22
- Limit cheat for selection (#102)
- Minor README update (#103)
- Add widget for bash (#105)
- Thanks for making this project, it's really cool! I'd been looking for something like this for a while.
- Interpolation: only quote multi-word values (#100)


## [v0.10.3](releases/tag/v0.10.3) 2020-03-22
- Show comment for command in preview window (#98)


## [v0.10.2](releases/tag/v0.10.2) 2020-03-22
- #60 names can include dashes or spaces
- Allow variable names to include dashes and spaces (#97)
- Minor fixes (#96)
- #60 fix space/dash regex


## [v0.10.1](releases/tag/v0.10.1) 2020-03-22
- Correctly escape % in cheatsheets (#94)


## [v0.10.0](releases/tag/v0.10.0) 2020-03-22
- Improve zsh widget (#92)
- Add suppot for multiline snippets (#91)
- Escape subshells in cheatsheets (#90)
- Store cheatsheets in memory (#86)
- Improve test coverage (#84)
- Add option for calling best match directly (#83)
- Improve lib for collections (#82)
- Refactor option parsing (#79)
- The commands added are as under:-
- Add warning about version on new issues (#81)
- Refer to FZF_DEFAULT_OPTS (#80)
- Update command name in usage and examples


## [v0.9.4](releases/tag/v0.9.4) 2020-03-22
- Fix escaping issues (#73)


## [v0.9.3](releases/tag/v0.9.3) 2020-03-22
- Add Github badges (#71)
- Bump to 0.9.3 (#70)
- Suppress tput errors (#69)
- Autoselect on single option (#68)
- Fix snippets with multiple arguments (#67)
- Setup CODEOWNERS (#59)


## [v0.9.2](releases/tag/v0.9.2) 2020-03-22
- Minor refactor (#57)
- Add workaround for --help (#54)


## [v0.9.0](releases/tag/v0.9.0) 2020-03-22
- Add MVP for zsh widget (#50)
- Refactor code (#49)
- Revert "Use dictionaries (#47)" (#48)
- Use dictionaries (#47)
- Added cheat for brew
- Update kubernetes cheat
- Prevent wait_for error (#46)
- Fix local readonly (#42)
- Improve test runner (#41)


## [v0.8.1](releases/tag/v0.8.1) 2020-03-22


## [v0.8.0](releases/tag/v0.8.0) 2020-03-22
- Add option for searching online repositories (#38)


## [v0.7.0](releases/tag/v0.7.0) 2020-03-22
- Bump to 0.7.0 (#32)
- Add update script (#31)
- Fix linting (#30)
- Validate variable names (#29)
- Add preview window (#28)
- Warn about fzf if not installed (#27)
- This PR adds additional cheats to the git cheatsheet.
- Cheatsheet for handling systemctl services.
- navi is probably better but cmdmenu is similar so maybe it could be listed as an alternative.
- Fix $database typo in mysql cheatsheet (#26)
- Prevent printing result in some cases with --no-interpolation (#25)
- Add double-quotes to arguments (#24)
- Remove dependency on brew in scripts/release (#23)
- Fix indentation (#8)
- Configure welcome probot (#7)
- Add icon (#6)
- Request for contributions (#5)
- License under AGPL-3 (#4)


## [v0.6.0](releases/tag/v0.6.0) 2020-03-22
- Add tests (#3)


## [v0.5.0](releases/tag/v0.5.0) 2020-03-22
- Improve README (#1)


## [v0.4.0](releases/tag/v0.4.0) 2020-03-22


## [v0.2.0](releases/tag/v0.2.0) 2020-03-22
