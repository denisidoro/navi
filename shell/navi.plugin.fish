function _navi_smart_replace
    set -l current_process (commandline -p | string trim)

    if test -z "$current_process"
        commandline -i (navi --print)
    else
        set -l best_match (navi --print --best-match --query "$current_process")

        if not test "$best_match" >/dev/null
            return
        end

        if test -z "$best_match"
            commandline -p (navi --print --query $current_process)
        else if test "$current_process" != "$best_match"
            commandline -p $best_match
        else
            commandline -p (navi --print --query $current_process)
        end
    end

    commandline -f repaint
end

if test $fish_key_bindings = fish_default_key_bindings
    bind \cg _navi_smart_replace
else
    bind -M insert \cg _navi_smart_replace
end
