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

pub fn parse() -> Config {
    Config::from_args()
}
