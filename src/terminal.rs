use terminal_size::{terminal_size, terminal_size_using_fd, Height, Width};

fn width_with_shell_out() -> u16 {
    use std::process::Command;
    use std::process::Stdio;

    let output = if cfg!(target_os = "macos") {
        Command::new("stty")
            .arg("-f")
            .arg("/dev/stderr")
            .arg("size")
            .stderr(Stdio::inherit())
            .output()
            .unwrap()
    } else {
        Command::new("stty")
            .arg("size")
            .arg("-F")
            .arg("/dev/stderr")
            .stderr(Stdio::inherit())
            .output()
            .unwrap()
    };

    match output.status.code() {
        Some(0) => {
            let stdout = String::from_utf8(output.stdout).unwrap();
            let mut data = stdout.split_whitespace();
            data.next();
            u16::from_str_radix(data.next().unwrap(), 10).unwrap()
        }
        _ => 40,
    }
}

fn width_with_fd() -> u16 {
    use std::fs;
    use std::os::unix::io::AsRawFd;

    let file = fs::File::open("/dev/tty");

    if let Ok(f) = file {
        let size = terminal_size_using_fd(f.as_raw_fd());

        if let Some((Width(w), Height(_))) = size {
            w
        } else {
            width_with_shell_out()
        }
    } else {
        width_with_shell_out()
    }
}

pub fn width() -> u16 {
    let size = terminal_size();
    if let Some((Width(w), Height(_))) = size {
        w
    } else {
        width_with_fd()
    }
}
