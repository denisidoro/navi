use crate::handler;
use crate::structures::{error::command::BashSpawnError, option};
use anyhow::Error;
use std::process::Command;

pub fn main(func: String, args: Vec<String>) -> Result<(), Error> {
    match func.as_str() {
        "url::open" => {
            let url = args
                .into_iter()
                .next()
                .ok_or_else(|| anyhow!("No URL specified"))?;
            let code = r#"
exst() {
   type "$1" &>/dev/null
}

_open_url() { 
    local -r url="$1"
    if exst xdg-open; then
        xdg-open "$url" &disown
    elif exst open; then
        echo "$url" | xargs -I% open "%"
    else
        exit 55
    fi
}"#;
            let cmd = format!(
                r#"{}
                
read -r -d '' url <<'EOF'
{}
EOF

_open_url "$url""#,
code,
                url
            );
            Command::new("bash")
                .arg("-c")
                .arg(cmd.as_str())
                .spawn()
                .map_err(|e| BashSpawnError::new(cmd, e))?
                .wait()?;
            Ok(())
        }

        "welcome" => handler::handle_config(option::config_from_iter(
            "navi --path /tmp/navi/irrelevant".split(' ').collect(),
        )),

        _ => Err(anyhow!("Unrecognized function")),
    }
}
