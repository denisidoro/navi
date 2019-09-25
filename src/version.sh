version::code() {
   cd "$SCRIPT_DIR"
   local -r git_info=$(gsit log -n 1 --pretty=format:'%h%n%ad%n%s' --date=format:'%Y-%m-%d %Hh%M')
   if [ -z "$git_info" ]; then
      return 1
   else
      echo -e "$git_info"
   fi
}