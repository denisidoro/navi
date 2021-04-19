use crate::shell::{self, ShellSpawnError};
use anyhow::Result;
use shell::EOF;

pub fn open(args: Vec<String>) -> Result<()> {
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
        r#"{code}
                
read -r -d '' url <<'{eof}'
{url}
{eof}

_open_url "$url""#,
        code = code,
        url = url,
        eof = EOF,
    );
    shell::command()
        .arg("-c")
        .arg(cmd.as_str())
        .spawn()
        .map_err(|e| ShellSpawnError::new(cmd, e))?
        .wait()?;
    Ok(())
}
