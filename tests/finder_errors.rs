#![cfg(unix)]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_ID: AtomicUsize = AtomicUsize::new(0);

struct Fixture(PathBuf);

impl Fixture {
    fn new(script: &str) -> Self {
        let dir = loop {
            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            let dir = std::env::temp_dir().join(format!("navi-finder-{}-{id}", std::process::id()));
            match fs::create_dir(&dir) {
                Ok(()) => break dir,
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(e) => panic!("Could not create test directory: {e}"),
            }
        };
        let fixture = Self(dir);
        fs::write(
            fixture.0.join("test.cheat"),
            "% test\n\n# example\necho <value>\n",
        )
        .unwrap();
        fs::write(fixture.0.join("config.yaml"), "{}").unwrap();
        let finder = fixture.0.join("fzf");
        fs::write(
            &finder,
            format!(
                "#!/bin/sh\n\
                 if [ \"$1\" = --version ]; then echo '0.60.0'; exit 0; fi\n\
                 IFS= read -r selection\n\
                 cat >/dev/null\n\
                 printf x >>\"$NAVI_TEST_CALLS\"\n\
                 calls=$(wc -c <\"$NAVI_TEST_CALLS\")\n\
                 # Bound retries so the unfixed binary fails tests without overflowing.\n\
                 if [ \"$calls\" -gt 2 ]; then exit 130; fi\n\
                 {script}\n"
            ),
        )
        .unwrap();
        fs::set_permissions(finder, fs::Permissions::from_mode(0o700)).unwrap();
        fixture
    }

    fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_navi"))
            .env_clear()
            .env("PATH", format!("{}:/usr/bin:/bin", self.0.display()))
            .env("HOME", &self.0)
            .env("SHELL", "/bin/sh")
            .env("TERM", "xterm")
            .env("NAVI_CONFIG", self.0.join("config.yaml"))
            .env("NAVI_TEST_CALLS", self.0.join("calls"))
            .args(["--print", "--path"])
            .arg(&self.0)
            .args(args)
            .stdin(Stdio::null())
            .output()
            .unwrap()
    }

    fn calls(&self) -> usize {
        fs::read(self.0.join("calls")).unwrap().len()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn finder_error_exits_once_without_panicking() {
    for status in [2, 42] {
        let fixture = Fixture::new(&format!("echo 'finder failed' >&2\nexit {status}"));
        let out = fixture.run(&[]);
        assert_eq!(out.status.code(), Some(1), "{out:?}");
        assert_eq!(fixture.calls(), 1);
        assert!(out.stdout.is_empty());
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert!(stderr.contains("finder failed"), "{stderr}");
        assert!(stderr.contains("Finder failed"), "{stderr}");
        assert!(!stderr.contains("panicked"), "{stderr}");
    }
}

#[test]
fn no_match_exits_once_in_interactive_and_best_match_modes() {
    for args in [vec![], vec!["--best-match", "--query", "missing"]] {
        let fixture = Fixture::new("exit 1");
        let out = fixture.run(&args);
        assert_eq!(out.status.code(), Some(1), "{out:?}");
        assert_eq!(fixture.calls(), 1);
        assert!(out.stdout.is_empty());
        assert!(out.stderr.is_empty(), "{out:?}");
    }
}

#[test]
fn cancellation_preserves_exit_status() {
    let fixture = Fixture::new("exit 130");
    let out = fixture.run(&[]);
    assert_eq!(out.status.code(), Some(130), "{out:?}");
    assert_eq!(fixture.calls(), 1);
    assert!(out.stdout.is_empty());
    assert!(out.stderr.is_empty());
}

#[test]
fn malformed_successful_selection_is_not_retried() {
    let fixture = Fixture::new("printf 'enter\\n'");
    let out = fixture.run(&[]);
    assert_eq!(out.status.code(), Some(1), "{out:?}");
    assert_eq!(fixture.calls(), 1);
    assert!(out.stdout.is_empty());
    assert!(String::from_utf8_lossy(&out.stderr).contains("Invalid selection from finder"));
}

#[test]
fn valid_selection_still_prints_the_snippet() {
    let fixture = Fixture::new("printf 'enter\\n%s\\n' \"$selection\"");
    let out = fixture.run(&["--prevent-interpolation"]);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(fixture.calls(), 1);
    assert_eq!(out.stdout, b"echo <value>\n");
}

#[test]
fn unmatched_custom_variable_input_is_preserved() {
    let fixture = Fixture::new(
        "if [ \"$calls\" -eq 1 ]; then\n\
             printf 'enter\\n%s\\n' \"$selection\"\n\
         else\n\
             printf 'custom value\\n'\n\
             exit 1\n\
         fi",
    );
    let out = fixture.run(&[]);
    assert!(out.status.success(), "{out:?}");
    assert_eq!(fixture.calls(), 2);
    assert_eq!(out.stdout, b"echo custom value\n");
}
