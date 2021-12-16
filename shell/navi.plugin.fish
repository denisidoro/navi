#!/usr/bin/env fish

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

function smart_replace
  set -l current_buffer (commandline -b)
  set -l current_process (commandline -p)
  set -l best_match (navi --print --best-match --query $current_process)
  echo $best_match $current_process
  
  if [ $current_process != $best_match ]
    echo "true"
    commandline -i $best_match
  else
    echo "false"
    navi-widget
  end
end
  
  
  
  #cli_buffer = commandline -
  # split at last | left/right str
  # set $user_input
  # if query = answer : call Navi-widget
  # else smart_replace : 
  # set output (navi --print --best-match --query $user_input)
  # replace userinput string in clibuff or left string += output
  # commandline append repaint  

bind \cg smart_replace
if bind -M insert > /dev/null 2>&1
  bind -M insert \cg smart_replace
end
