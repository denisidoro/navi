use anyhow::Error;
use std::process;

pub fn abort(operation: &str, issue_number: u32) -> Result<(), Error> {
    eprintln!("This version of navi doesn't support {}.", operation);
    eprintln!(
        "Please check https://github.com/denisidoro/navi/issues/{} for more info.",
        issue_number
    );
    eprintln!("");
    eprintln!("You were probably using the bash implementation of navi and are now using the Rust one, which isn't feature complete yet.");
    eprintln!("In the near future, the Rust version will have all previous features.");
    eprintln!("");
    eprintln!("I'm sorry for the inconvenience.");
    process::exit(42)
}
