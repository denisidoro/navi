function __call_navi
    navi --print
end 

function navi-widget -d "Show cheat sheets"
  begin
    set ttysettings (stty -g)
    stty sane
    __call_navi | perl -pe 'chomp if eof' | read -lz result
    and commandline -- $result

    stty $ttysettings
  end
  commandline -f repaint
end

# set -g navi_last_cmd ""

function smart_replace
  set -l current_process (commandline -p)

  if [ $current_process = "" ]
    commandline -p (navi --print)
    commandline -f repaint
  else
    set -l best_match (navi --print --best-match --query $current_process)

    if not [ $best_match > /dev/null ];
      commandline -p $current_process
      commandline -f repaint
      return
    end

    if [ $best_match = "" ]
        commandline -p (navi --print --query $current_process)
        commandline -f repaint
    else if [ $current_process != $best_match ]
      commandline -p $best_match
      commandline -f repaint
    else if [ $current_process = $best_match ]
      commandline -p (navi --print --query $current_process)
      commandline -f repaint
    end
  end
end

bind \cg smart_replace
if bind -M insert > /dev/null 2>&1
  bind -M insert \cg smart_replace
end
