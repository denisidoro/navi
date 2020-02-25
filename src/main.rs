use std::io::Write;
use std::process::{Command, Stdio};
use std::error::Error;
use std::thread;
use std::time::Duration;
use ansi_term::Colour;

fn simulated_expensive_calculation(intensity: u32) -> String {
    thread::sleep(Duration::from_secs(1));
    format!("{} - line {}\n", Colour::Red.paint("a red string"), intensity)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut child = Command::new("fzf")
        .args(&["--preview", "echo {}", 
                "--height", "100%", 
                "--ansi"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let stdin = child.stdin
        .as_mut()
        .ok_or("Child process stdin has not been captured!")?;

    for i in 0..10 {
        match stdin.write(simulated_expensive_calculation(i).as_bytes()) {
            Ok(_) => (),
            Err(_) => break,
        };
    }

    let output = child.wait_with_output()?;

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        println!("result: {:#?}", raw_output);
        Ok(())
    } else {
        let err = String::from_utf8(output.stderr)?;
        panic!("External command failed:\n {}", err)
    }
}