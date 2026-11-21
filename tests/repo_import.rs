#![cfg(all(unix, not(feature = "disable-repo-management")))]

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

struct TestDir(PathBuf);

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn repo_import_previews_absolute_paths() {
    let unique = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let root = TestDir(std::env::temp_dir().join(format!("navi-repo-{}-{unique}", std::process::id())));
    let source = root.0.join("user/cheats");
    let bin = root.0.join("bin");
    fs::create_dir_all(&source).unwrap();
    fs::create_dir_all(&bin).unwrap();
    let contents = "% example\n# Preview this file\necho imported\n";
    fs::write(source.join("example's file.cheat"), contents).unwrap();
    for args in [
        vec!["init", "--quiet"],
        vec!["add", "."],
        vec![
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.com",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "--quiet",
            "-m",
            "fixture",
        ],
    ] {
        assert!(Command::new("git")
            .args(args)
            .current_dir(&source)
            .status()
            .unwrap()
            .success());
    }

    // Model the finder's shell-escaped {} substitution, then execute the preview
    // navi actually supplies. No terminal or installed finder is needed.
    let finder = bin.join("fzf");
    fs::write(
        &finder,
        r#"#!/usr/bin/env bash
set -eu
if [[ "$1" == --version ]]; then
    echo 0.60.0
    exit
fi
preview=''
multi=false
while (( $# )); do
    case "$1" in
        --preview) preview="$2"; shift ;;
        --multi) multi=true ;;
    esac
    shift
done
input=$(cat)
if [[ "$input" == $'Yes\nNo' ]]; then
    echo No
elif [[ "$multi" == true ]]; then
    file="${input%%$'\n'*}"
    printf -v quoted '%q' "$file"
    bash -c "${preview//\{\}/$quoted}" > "$NAVI_TEST_PREVIEW"
    printf '%s\n' "$file"
else
    # The import succeeded; cancel the subsequent snippet picker.
    exit 130
fi
"#,
    )
    .unwrap();
    fs::set_permissions(&finder, fs::Permissions::from_mode(0o755)).unwrap();
    let path = std::env::join_paths(std::iter::once(bin).chain(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    )))
    .unwrap();

    for dirname in ["data", "Application Support", "user's data"] {
        let data = root.0.join(dirname);
        let preview = root.0.join("preview");
        let output = Command::new(env!("CARGO_BIN_EXE_navi"))
            .args(["repo", "add"])
            .arg(format!("file://{}", source.display()))
            .env("PATH", &path)
            .env("XDG_DATA_HOME", &data)
            .env("XDG_CONFIG_HOME", root.0.join("config"))
            .env("NAVI_CONFIG", root.0.join("config.yaml"))
            .env("NAVI_CONFIG_YAML", "{}")
            .env("NAVI_FINDER", "fzf")
            .env("NAVI_TEST_PREVIEW", &preview)
            .env("NAVI_PATH", data.join("navi/cheats"))
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(130),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(fs::read_to_string(&preview).unwrap(), contents, "{dirname}");
        assert_eq!(
            fs::read_to_string(data.join("navi/cheats/user__cheats/example's file.cheat")).unwrap(),
            contents,
            "{dirname}"
        );
        assert!(!data.join("navi/cheats/tmp").exists());
    }
}
