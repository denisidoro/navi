use std::io::Write;
use std::process::{Command, Stdio};
use std::error::Error;
use ansi_term::Colour;
use std::fs::File;
use std::io::{self, BufRead};
use std::fs;
use std::path::Path;
use std::env;
use ansi_term::Style;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn gen_snippet(snippet: &String, line: &String) -> String {
    if snippet.len() < 1 {
        line.clone() 
    } else {
        format!("{}{}", &snippet[..snippet.len() - 2], line)
    }
}

fn parse_file(path: &str, stdin: &mut std::process::ChildStdin) {

    let mut tags = String::from("");
    let mut comment = String::from("");
    let mut snippet = String::from("");

    let style = Style::new().hidden();

    if let Ok(lines) = read_lines(path) {
        for l in lines {
            let line = l.unwrap();
            if line.starts_with('%') { tags = line; }
            else if line.starts_with('#') { comment = line; } 
            else if line.starts_with('$') { } // TODO
            else if line.ends_with('\\') {
                snippet = if snippet.len() > 0 {
                    format!("{}{}", &snippet[..snippet.len() - 2], line)
                } else {
                    line
                }
            }
            else if line.len() < 1 { }
            else { 
                match stdin.write(format!("{} {} {}\n", Colour::Red.paint(&tags[2..]), Colour::Blue.paint(&comment[2..]), Colour::Green.paint(gen_snippet(&snippet, &line))).as_bytes()) {
                    Ok(_) => snippet = String::from(""),
                    Err(_) => break,
                } 
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "preview" {
        println!("preview");
        println!("{}", args[2]);
        panic!("hi")
    }
    
    let mut child = Command::new("fzf")
        .args(&["--preview", "./navi preview {}", 
                "--height", "100%", 
                "--preview-window", "up:3",
                "--ansi"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;

    let stdin = child.stdin
        .as_mut()
        .ok_or("Child process stdin has not been captured!")?;

    let paths = fs::read_dir("./cheats").unwrap();
    for path in paths {
        parse_file(path.unwrap().path().into_os_string().to_str().unwrap(), stdin);
    }

    /*for i in 0..10 {
        match stdin.write(simulated_expensive_calculation(i).as_bytes()) {
            Ok(_) => (),
            Err(_) => break,
        };
    }*/

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