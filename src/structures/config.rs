use crate::finder::FinderChoice;
use anyhow::Error;
use structopt::{clap::AppSettings, StructOpt};

fn parse_finder(src: &str) -> Result<FinderChoice, Error> {
    match src {
        "fzf" => Ok(FinderChoice::Fzf),
        "skim" => Ok(FinderChoice::Skim),
        _ => Err(Error::msg(format!("unknown finder '{}'", src))),
    }
}

#[derive(Debug, StructOpt)]
#[structopt(after_help = r#"MORE INFO:
    Please refer to https://github.com/denisidoro/navi

EXAMPLES:
    navi                                   # default behavior
    navi --print                           # doesn't execute the snippet
    navi --path '/some/dir:/other/dir'     # uses custom cheats
    navi search docker                     # uses online data
    navi query git                         # filters results by "git"
    navi best 'sql create db' root mydb    # uses a snippet as a CLI
    navi repo add denisidoro/cheats        # imports cheats from github.com/denisidoro/cheats
    source <(navi widget zsh)              # loads the zsh widget
    navi --finder 'skim'                   # set which finder is supposed to be used (fzf [default] / skim)
    navi --fzf-overrides '--with-nth 1,2'  # shows only the comment and tag columns
    navi --fzf-overrides '--nth 1,2'       # search will consider only the first two columns
    navi --fzf-overrides '--no-exact'      # looser search algorithm"#)]
#[structopt(setting = AppSettings::ColorAuto)]
#[structopt(setting = AppSettings::ColoredHelp)]
#[structopt(setting = AppSettings::AllowLeadingHyphen)]
pub struct Config {
    /// List of :-separated paths containing .cheat files
    #[structopt(short, long, env = "NAVI_PATH")]
    pub path: Option<String>,

    /// [Experimental] Instead of executing a snippet, saves it to a file
    #[structopt(short, long)]
    pub save: Option<String>,

    /// Instead of executing a snippet, prints it to stdout
    #[structopt(long)]
    pub print: bool,

    /// Prevents autoselection in case of single entry
    #[structopt(long)]
    pub no_autoselect: bool,

    /// Hides preview window
    #[structopt(long)]
    pub no_preview: bool,

    /// finder overrides for cheat selection
    #[structopt(long, env = "NAVI_FZF_OVERRIDES")]
    pub fzf_overrides: Option<String>,

    /// finder overrides for variable selection
    #[structopt(long, env = "NAVI_FZF_OVERRIDES_VAR")]
    pub fzf_overrides_var: Option<String>,

    /// finder overrides for variable selection
    #[structopt(long, env = "NAVI_FINDER", default_value = "fzf", parse(try_from_str = parse_finder))]
    pub finder: FinderChoice,

    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Filters results
    Query {
        /// String used as filter (example: "git")
        query: String,
    },
    /// Uses online repositories for cheatsheets
    Search {
        /// String used as filter (example: "git")
        query: String,
    },
    /// Autoselects the snippet that best matches the query
    Best {
        /// String used as filter (example: "git remove branch")
        query: String,
        /// List of arguments (example: "mybranch" "remote")
        args: Vec<String>,
    },
    /// Performs ad-hoc functions provided by navi
    Fn {
        /// Function name (example: "url::open")
        func: String,
        /// List of arguments (example: "https://google.com")
        args: Vec<String>,
    },
    /// Manages cheatsheet repositories
    Repo {
        #[structopt(subcommand)]
        cmd: RepoCommand,
    },
    /// Used for fzf's preview window
    #[structopt(setting = AppSettings::Hidden)]
    Preview {
        /// Selection line
        line: String,
    },
    /// Shows the path for shell widget files
    Widget {
        /// bash, zsh or fish
        shell: String,
    },
    /// Helper command for Alfred integration
    #[structopt(setting = AppSettings::Hidden)]
    Alfred {
        #[structopt(subcommand)]
        cmd: AlfredCommand,
    },
}

#[derive(Debug, StructOpt)]
pub enum RepoCommand {
    /// Imports cheatsheets from a repo
    Add {
        /// A URI to a git repository containing .cheat files ("user/repo" will download cheats from github.com/user/repo)
        uri: String,
    },
    /// Browses for featured cheatsheet repos
    Browse,
}

#[derive(Debug, StructOpt)]
pub enum AlfredCommand {
    /// Outputs a JSON with commands
    Start,
    /// Outputs a JSON with variable suggestions
    Suggestions,
    /// Transforms the snippet env var with the selected value
    Transform,
    /// Checks whether to use free input
    Check,
}

pub fn config_from_env() -> Config {
    Config::from_args()
}

pub fn config_from_iter(args: Vec<&str>) -> Config {
    Config::from_iter(args)
}
