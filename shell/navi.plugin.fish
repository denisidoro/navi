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
  # set -l current_buffer (commandline -b)
  set -l current_process (commandline -p)
  # set -l current_job (commandline -j)
  set -l best_match (navi --print --best-match --query $current_process)
  
  if [ $current_process = $best_match ]
  # if [ $current_process != $best_match ]
    # echo "true"
    set -l new_process (navi --print --query $current_process)
    # echo $new_process
    commandline -p $new_process
    commandline -f repaint
  else
    # echo "false"
    commandline -p $best_match
    commandline -f repaint
    # commandline -p ' '$best_match
    # navi-widget
  end
end

bind \cg smart_replace
if bind -M insert > /dev/null 2>&1
  bind -M insert \cg smart_replace
end
