use crate::prelude::*;
use crossterm::terminal;
use std::process::Command;

const FALLBACK_WIDTH: u16 = 80;

fn width_with_shell_out() -> Result<u16> {
    let output = if cfg!(target_os = "windows") {
        Command::new("stty")
            .args(["size", "-F", "/dev/stderr"])
            .stderr(Stdio::inherit())
            .output()?
    } else {
        Command::new("stty")
            .arg("-f")
            .arg("/dev/stderr")
            .arg("size")
            .stderr(Stdio::inherit())
            .output()?
    };

    if let Some(0) = output.status.code() {
        let stdout = String::from_utf8(output.stdout).expect("Invalid utf8 output from stty");
        let mut data = stdout.split_whitespace();
        data.next();
        return data
            .next()
            .expect("Not enough data")
            .parse::<u16>()
            .map_err(|_| anyhow!("Invalid width"));
    }

    Err(anyhow!("Invalid status code"))
}

pub fn width() -> u16 {
    if let Ok((w, _)) = terminal::size() {
        w
    } else {
        width_with_shell_out().unwrap_or(FALLBACK_WIDTH)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_width_with_shell_out() {
        let result = width_with_shell_out().expect("Shell error");
        let is_ok = if result == 0 { false } else { true };

        assert!(is_ok);
    }
}
