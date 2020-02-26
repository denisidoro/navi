use std::process;
use std::process::{Command, Stdio};

pub fn call<F>(f: F) -> process::Output
where
    F: Fn(&mut process::ChildStdin) -> (),
{
    let mut child = Command::new("fzf")
        .args(&[
            "--height",
            "100%",
            "--preview-window",
            "up:2",
            "--with-nth",
            "1,2,3",
            "--delimiter",
            "\t",
            "--ansi",
        ])
        .args(&["--preview", "./navi preview {}"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    let stdin = child
        .stdin
        .as_mut()
        .ok_or("Child process stdin has not been captured!")
        .unwrap();

    f(stdin);

    child.wait_with_output().unwrap()
}
