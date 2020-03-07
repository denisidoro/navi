#!/usr/bin/env fish

function __call_navi
    navi --print
end 

function navi-widget -d "Show cheat sheets"
  begin
    stty sane
    __call_navi | perl -pe 'chomp if eof' | read -lz result
    and commandline -- $result
  end
  commandline -f repaint
end

bind \cg navi-widget
if bind -M insert > /dev/null 2>&1
  bind -M insert \cg navi-widget
end
