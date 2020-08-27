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
    navi query git                         # filters results by "git"
    navi best 'sql create db'              # uses a snippet as a CLI
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

    /// Returns the best match
    #[structopt(long)]
    pub single: bool,

    /// Query
    #[structopt(short, long)]
    pub query: Option<String>,

    /// finder overrides for cheat selection
    #[structopt(long, env = "NAVI_FZF_OVERRIDES")]
    pub fzf_overrides: Option<String>,

    /// finder overrides for variable selection
    #[structopt(long, env = "NAVI_FZF_OVERRIDES_VAR")]
    pub fzf_overrides_var: Option<String>,

    /// which finder application to use
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
    /// Search for cheatsheets using the tldr repository
    Tldr {
        /// bash, zsh or fish
        query: String,
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

fn deprecated(syntax: &str) {
    eprintln!(
        r"Warning: the following syntax has been DEPRECATED:
navi {}

Please check navi --help for more info on how to achieve the same result with the new syntax.

The deprecated syntax will be removed in the first version released on 2021!",
        syntax
    );
}

pub enum Source {
    FILESYSTEM(Option<String>),
    TLDR(String),
}

pub enum Action {
    SAVE(String),
    PRINT,
    EXECUTE,
}

impl Config {
    pub fn source(&self) -> Source {
        match self.cmd.as_ref() {
            Some(Command::Tldr { query }) => Source::TLDR(query.clone()),
            _ => Source::FILESYSTEM(self.path.clone()),
        }
    }

    pub fn action(&self) -> Action {
        if let Some(filepath) = self.save.clone() {
            Action::SAVE(filepath)
        } else if self.print {
            Action::PRINT
        } else {
            Action::EXECUTE
        }
    }

    pub fn get_query(&self) -> Option<String> {
        match &self.cmd {
            Some(Command::Query { query }) => {
                deprecated("query <query>");
                Some(query.clone())
            }
            Some(Command::Best { query, .. }) => {
                deprecated("best <query>");
                Some(query.clone())
            }
            _ => self.query.clone(),
        }
    }

    pub fn get_single(&self) -> bool {
        if let Some(Command::Best { .. }) = &self.cmd {
            deprecated("best <query>");
            true
        } else {
            self.single
        }
    }

    pub fn get_no_autoselect(&self) -> bool {
        if self.no_autoselect {
            deprecated("--no-autoselect");
            true
        } else {
            false
        }
    }
}

pub fn config_from_env() -> Config {
    Config::from_args()
}

pub fn config_from_iter(args: Vec<&str>) -> Config {
    Config::from_iter(args)
}
