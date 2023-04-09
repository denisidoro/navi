use crate::prelude::*;
use crossterm::style;
use crossterm::terminal;

use std::process::Command;

const FALLBACK_WIDTH: u16 = 80;

fn width_with_shell_out() -> Result<u16> {
    let output = if cfg!(target_os = "macos") {
        Command::new("stty")
            .arg("-f")
            .arg("/dev/stderr")
            .arg("size")
            .stderr(Stdio::inherit())
            .output()?
    } else {
        Command::new("stty")
            .arg("size")
            .arg("-F")
            .arg("/dev/stderr")
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

pub fn parse_ansi(ansi: &str) -> Option<style::Color> {
    style::Color::parse_ansi(&format!("5;{ansi}"))
}

#[derive(Debug, Clone)]
pub struct Color(pub style::Color);

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(ansi: &str) -> Result<Self, Self::Err> {
        if let Some(c) = parse_ansi(ansi) {
            Ok(Color(c))
        } else {
            Err("Invalid color")
        }
    }
}
