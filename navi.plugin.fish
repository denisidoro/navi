function navi_key_bindings
  function navi-widget -d "Show cheat sheet"
    set -q FZF_TMUX_HEIGHT; or set FZF_TMUX_HEIGHT 40%
    begin
      set -lx FZF_DEFAULT_OPTS "--height $FZF_TMUX_HEIGHT $FZF_DEFAULT_OPTS --tiebreak=index --bind=ctrl-r:toggle-sort $FZF_CTRL_R_OPTS +m"
      stty sane
      env NAVI_USE_FZF_ALL_INPUTS=true navi --print query (commandline)  | perl -pe 'chomp if eof' | read -lz result
      and commandline -- $result
    end
    commandline -f repaint
  end

  bind \cg navi-widget
  if bind -M insert > /dev/null 2>&1
    bind -M insert \cg navi-widget
  end
end
