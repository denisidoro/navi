version::code() {
   cd "$NAVI_HOME"
   local -r git_info=$(git log -n 1 --pretty=format:'%h%n%ad%n%s' --date=format:'%Y-%m-%d %Hh%M')
   if [ -z "$git_info" ]; then
      return 1
   else
      echo -e "$git_info"
   fi
}