use crate::handler;
use crate::structures::config;
use crate::url;
use anyhow::Error;

#[derive(Debug)]
pub enum Func {
    UrlOpen,
    Welcome,
}

pub fn main(func: &Func, args: Vec<String>) -> Result<(), Error> {
    match func {
        Func::UrlOpen => url::open(args),
        Func::Welcome => handler::handle_config(config::config_from_iter(
            "navi --path /tmp/navi/irrelevant".split(' ').collect(),
        )),
    }
}
