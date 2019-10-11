function navi-widget -d 'Call navi'
  set -q NAVI_USE_FZF_ALL_INPUTS; or set -l NAVI_USE_FZF_ALL_INPUTS "true"
  begin
    navi --print | while read -l r; set result $result $r; end

    if [ -n "$result" ]
      echo $result

      # Remove last token from commandline.
      commandline -t ""
    end
  end

  # commandline -f repaint
end

bind \cg navi-widget

if bind -M insert > /dev/null 2>&1
  bind -M insert \cr navi-widget
end
