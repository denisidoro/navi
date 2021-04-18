use crate::shell::{self, ShellSpawnError};
use anyhow::Result;
use std::io::{self, Read};

pub fn main() -> Result<()> {
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;
    let mut parts = text.split("NAVIEOF");
    let p1 = parts.next().unwrap();
    let p2 = parts.next().unwrap();
    let p3 = parts.next().unwrap().trim();
    super::handler::preview_var::main(p1, p2, p3)
}
