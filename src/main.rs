use std::io::Write;
use std::process::{Command, Stdio};
use std::error::Error;
use ansi_term::Colour;
use std::fs::File;
use std::io::{self, BufRead};
use std::fs;
use std::path::Path;
use std::env;
use std::process;
use regex::Regex;

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

fn limit_str(text: &str, length: usize) -> String {
    if text.len() > length {
        format!("{}…", &text[..length-1])
    } else {
        format!("{:width$}", text, width = length) 
    }
}

fn parse_file(path: &str, stdin: &mut std::process::ChildStdin) {

    let mut tags = String::from("");
    let mut comment = String::from("");
    let mut snippet = String::from("");

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
                let full_snippet = gen_snippet(&snippet, &line);
                match stdin.write(format!("{col0}\t{col1}\t{col2}\t{tags}\t{comment}\t{snippet}\t\n", 
                                          col0 = Colour::Red.paint(limit_str(&tags[2..], 16)), 
                                          col1 = Colour::Blue.paint(limit_str(&comment[2..], 26)), 
                                          col2 = Colour::Green.paint(&full_snippet),
                                          tags = &tags[2..],
                                          comment = &comment[2..],
                                          snippet = &full_snippet)
                                          .as_bytes()) {
                    Ok(_) => snippet = String::from(""),
                    Err(_) => break,
                } 
            }
        }
    }
}

fn extract_elements(argstr: &String) -> (&str, &str, &str) {
        let mut parts = argstr.split('\t').skip(3);
        let tags = parts.next().unwrap();
        let comment = parts.next().unwrap();
        let snippet = parts.next().unwrap();
        (tags, comment, snippet)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "preview" {
        let (tags, comment, snippet) = extract_elements(&args[2]);
        println!("{comment} {tags} \n{snippet}", 
                                          comment = Colour::Blue.paint(comment), 
                                          tags = Colour::Red.paint(format!("[{}]", tags)), 
                                          snippet = Colour::Green.paint(snippet));

        process::exit(0x0100)
    }
    
    let mut child = Command::new("fzf")
        .args(&["--preview", "./navi preview {}", 
                "--height", "100%", 
                "--preview-window", "up:2",
                "--with-nth", "1,2,3",
                "--delimiter", "\t",
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

    let output = child.wait_with_output()?;
    

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        let snippet = raw_output.split('\t').nth(5).unwrap();

        let re = Regex::new(r"<(.*?)>").unwrap();
        for cap in re.captures_iter(snippet) {
            let varname = &cap[0];
            println!("{}", &varname[1..varname.len()-1]);
        }

        Command::new("bash")
           .arg("-c")
           .arg(&snippet[..])
           .spawn()?;

        Ok(())
    } else {
        let err = String::from_utf8(output.stderr)?;
        panic!("External command failed:\n {}", err)
    }
}