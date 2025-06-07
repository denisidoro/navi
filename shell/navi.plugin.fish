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
    if test $version_parts[1] -ge 4
        set force_repaint true
    end

    if test -n "$query"
        set --local best_match (navi --print --query "$query" --best-match)
        if test -n "$best_match"
            commandline --current-process $best_match
            if test "$force_repaint" = true
                commandline --function repaint
            end
            return
        end
    end

    set --local candidate (navi --print --query "$query")
    if test -n "$candidate"
        commandline --current-process $candidate
        if test "$force_repaint" = true
            commandline --function repaint
        end
    end
end

bind \cg _navi_smart_replace
bind --mode insert \cg _navi_smart_replace
