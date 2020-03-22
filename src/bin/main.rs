extern crate navi;

use navi::FileAnIssue;

fn main() -> Result<(), FileAnIssue> {
    navi::handle_config(navi::config_from_env()).map_err(FileAnIssue::new)
}
