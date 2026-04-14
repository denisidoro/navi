function _navi_smart_replace
    set --local query (commandline --current-process | string trim)
    set --local version_parts ""
    if test -n "$version"
        set version_parts (string split '.' $version)
    else
        set version_parts (string split '.' (string match -r '\d+\.\d+\.\d+' (fish --version)))
    end

    set --local force_repaint false
    # https://github.com/fish-shell/fish-shell/blob/d663f553dffba460d6d0bcdf93df21bda9ec6f3f/doc_src/interactive.rst?plain=1#L440
    #  > Bindings that change the mode are supposed to call the repaint-mode bind function
    #
    # Related issues
    #  - https://github.com/fish-shell/fish-shell/issues/5033
    #  - https://github.com/fish-shell/fish-shell/issues/5860
    #  - https://github.com/fish-shell/fish-shell/blob/d663f553dffba460d6d0bcdf93df21bda9ec6f3f/src/screen.rs#L531
    #
    # Introduced with: https://github.com/denisidoro/navi/pull/982
    if test $version_parts[1] -ge 4
        set force_repaint true
    end

    if test -n "$query"
        set --local best_match (navi --print --query "$query" --best-match)
        if test -n "$best_match"
            # --replace without --current-process: --current-process treats newlines as process
            # boundaries and flattens multi-line snippets into a single line
            commandline --replace -- "$best_match"
            commandline --function end-of-line
        end
    end

    if test -z "$best_match"
        set --local candidate (navi --print --query "$query")
        if test -n "$candidate"
            commandline --replace -- "$candidate"
            commandline --function end-of-line
        end
    end

    # always repaint to restore the prompt after fzf clobbers the terminal
    if test "$force_repaint" = true
        commandline --function repaint
    end
end

bind __NAVI_KEY__ _navi_smart_replace
bind --mode insert __NAVI_KEY__ _navi_smart_replace
