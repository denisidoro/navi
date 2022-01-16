function _navi_smart_replace
  set -l current_process (commandline -p)

  if test $current_process = ""
    commandline -p (navi --print)
    commandline -f repaint
  else
    set -l best_match (navi --print --best-match --query $current_process)

    if not test $best_match > /dev/null
      commandline -p $current_process
      commandline -f repaint
      return
    end

    if test $best_match = ""
        commandline -p (navi --print --query $current_process)
        commandline -f repaint
    else if test $current_process != $best_match
      commandline -p $best_match
      commandline -f repaint
    else
      commandline -p (navi --print --query $current_process)
      commandline -f repaint
    end
  end
end

bind \cg smart_replace
bind -M insert \cg smart_replace
