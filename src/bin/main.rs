extern crate navi;

fn main() -> Result<(), anyhow::Error> {
    navi::handle_config(navi::config_from_env()).map_err(|e| navi::FileAnIssue::new(e).into())
}
