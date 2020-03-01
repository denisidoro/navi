use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(short, long, env = "NAVI_PATH")]
    pub path: Option<String>,

    #[structopt(long)]
    pub print: bool,

    #[structopt(long)]
    pub no_autoselect: bool,

    #[structopt(long)]
    pub no_preview: bool,

    /*#[structopt(long)]
    pub col_widths: Option<String>,

    #[structopt(long)]
    pub col_colors: Option<String>,*/
    #[structopt(long)]
    pub fzf_overrides: Option<String>,

    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    Query { args: Vec<String> },
    Home,
    Search { args: Vec<String> },
    Best { args: Vec<String> },
    Widget { shell: String },
}

pub enum InternalCommand {
    Preview { line: String },
}

pub fn parse() -> Config {
    Config::from_args()
}

pub fn internal_command() -> Option<InternalCommand> {
    let mut args = env::args();
    args.next();
    if args.next() == Some(String::from("preview")) {
        Some(InternalCommand::Preview {
            line: args.next().unwrap(),
        })
    } else {
        None
    }
}
