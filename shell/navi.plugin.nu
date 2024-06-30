export def navi_widget [] {
    let current_input = (commandline)
    let last_command = ($current_input | navi fn widget::last_command | str trim)

    match ($last_command | is-empty) {
        true => {^navi --print | complete | get "stdout"}
        false => {
            let find = $"($last_command)_NAVIEND"
            let replacement = (^navi --print --query $'($last_command)' | complete | get "stdout")

            match ($replacement | str trim | is-empty) {
                false => {$"($current_input)_NAVIEND" | str replace $find $replacement}
                true => $current_input
            }
        }
    } 
    | str trim
    | commandline edit --replace $in
    
    commandline set-cursor --end
}

let nav_keybinding = {
    name: "navi",
    modifier: control,
    keycode: char_g,
    mode: [emacs, vi_normal, vi_insert],
    event: {
        send: executehostcommand,
        cmd: navi_widget,
    }
}

$env.config.keybindings = ($env.config.keybindings | append $nav_keybinding)
