use crate::cmds::func::Func;
use crate::cmds::info::Info;
use crate::common::shell::Shell;
use crate::env_vars;
use crate::finder::FinderChoice;
use anyhow::Error;
use structopt::{clap::AppSettings, StructOpt};

static mut NOTIFIED_DEPRECATION: bool = false;

fn parse_finder(src: &str) -> Result<FinderChoice, Error> {
    match src.to_lowercase().as_str() {
        "fzf" => Ok(FinderChoice::Fzf),
        "skim" => Ok(FinderChoice::Skim),
        _ => Err(Error::msg(format!("unknown finder '{}'", src))),
    }
}

fn parse_shell(src: &str) -> Result<Shell, Error> {
    match src.to_lowercase().as_str() {
        "bash" => Ok(Shell::Bash),
        "zsh" => Ok(Shell::Zsh),
        "fish" => Ok(Shell::Fish),
        _ => Err(Error::msg(format!("unknown shell '{}'", src))),
    }
}

fn parse_func(src: &str) -> Result<Func, Error> {
    match src.to_lowercase().as_str() {
        "url::open" => Ok(Func::UrlOpen),
        "welcome" => Ok(Func::Welcome),
        _ => Err(Error::msg(format!("unknown shell '{}'", src))),
    }
}

fn parse_info(src: &str) -> Result<Info, Error> {
    match src.to_lowercase().as_str() {
        "cheats-path" => Ok(Info::CheatsPath),
        _ => Err(Error::msg(format!("unknown info '{}'", src))),
    }
}
#[derive(Debug, StructOpt)]
#[structopt(after_help = r#"MORE INFO:
    Please refer to https://github.com/denisidoro/navi

EXAMPLES:
    navi                                     # default behavior
    navi --print                             # doesn't execute the snippet
    navi --tldr docker                       # search for docker cheatsheets using tldr
    navi --cheatsh docker                    # search for docker cheatsheets using cheatsh
    navi --path '/some/dir:/other/dir'       # uses .cheat files from custom paths
    navi --query git                         # filters results by "git"
    navi 'create db' --best-match            # uses a snippet as a CLI
    name=mydb navi 'create db' --best-match  # same, but set the value for the <name> variable
    navi repo add denisidoro/cheats          # imports cheats from a git repository
    source <(navi widget zsh)                # loads the zsh widget
    navi --finder 'skim'                     # set skim as finder, instead of fzf
    navi --fzf-overrides '--with-nth 1,2'    # shows only the comment and tag columns
    navi --fzf-overrides '--no-select-1'     # prevents autoselection in case of single line
    navi --fzf-overrides '--nth 1,2'         # search will consider only the first two columns
    navi --fzf-overrides '--no-exact'        # looser search algorithm"#)]
#[structopt(setting = AppSettings::ColorAuto)]
#[structopt(setting = AppSettings::ColoredHelp)]
#[structopt(setting = AppSettings::AllowLeadingHyphen)]
pub struct Config {
    /// List of :-separated paths containing .cheat files
    #[structopt(short, long, env = env_vars::PATH)]
    pub path: Option<String>,

    /// [Experimental] Instead of executing a snippet, saves it to a file
    #[structopt(short, long)]
    save: Option<String>,

    /// Instead of executing a snippet, prints it to stdout
    #[structopt(long)]
    print: bool,

    /// Prevents autoselection in case of single entry
    #[structopt(long)]
    no_autoselect: bool,

    /// Hides preview window
    #[structopt(long)]
    pub no_preview: bool,

    /// Returns the best match
    #[structopt(long)]
    best_match: bool,

    /// Search for cheatsheets using the tldr-pages repository
    #[structopt(long)]
    tldr: Option<String>,

    /// Search for cheatsheets using the cheat.sh repository
    #[structopt(long)]
    cheatsh: Option<String>,

    /// Query
    #[structopt(short, long)]
    query: Option<String>,

    /// finder overrides for cheat selection
    #[structopt(long, env = env_vars::FZF_OVERRIDES)]
    pub fzf_overrides: Option<String>,

    /// finder overrides for variable selection
    #[structopt(long, env = env_vars::FZF_OVERRIDES_VAR)]
    pub fzf_overrides_var: Option<String>,

    /// which finder application to use
    #[structopt(long, env = env_vars::FINDER, default_value = "fzf", parse(try_from_str = parse_finder))]
    pub finder: FinderChoice,

    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Filters results
    #[structopt(setting = AppSettings::Hidden)]
    Query {
        /// String used as filter (example: "git")
        query: String,
    },
    /// Autoselects the snippet that best matches the query
    #[structopt(setting = AppSettings::Hidden)]
    Best {
        /// String used as filter (example: "git remove branch")
        query: String,
        /// List of arguments (example: "mybranch" "remote")
        args: Vec<String>,
    },
    /// Performs ad-hoc functions provided by navi
    Fn {
        /// Function name (example: "url::open")
        #[structopt(parse(try_from_str = parse_func))]
        func: Func,
        /// List of arguments (example: "https://google.com")
        args: Vec<String>,
    },
    /// Manages cheatsheet repositories
    Repo {
        #[structopt(subcommand)]
        cmd: RepoCommand,
    },
    /// Used for fzf's preview window when selecting snippets
    #[structopt(setting = AppSettings::Hidden)]
    Preview {
        /// Selection line
        line: String,
    },
    /// Used for fzf's preview window when selecting variable suggestions
    #[structopt(setting = AppSettings::Hidden)]
    PreviewVar {
        /// Selection line
        selection: String,
        /// Query match
        query: String,
        /// Typed text
        variable: String,
    },
    /// Shows the path for shell widget files
    Widget {
        #[structopt(default_value = "bash", parse(try_from_str = parse_shell))]
        shell: Shell,
    },
    /// Shows info
    Info {
        #[structopt(parse(try_from_str = parse_info))]
        info: Info,
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
    unsafe {
        if NOTIFIED_DEPRECATION {
            return;
        }
        eprintln!(
            r"⚠️ The following syntax has been DEPRECATED:
navi {}

Please check `navi --help` for more info on how to achieve the same result with the new syntax.

The deprecated syntax will be removed in the first version released on 2021! ⚠️
",
            syntax
        );
        NOTIFIED_DEPRECATION = true;
    }
}

pub enum Source {
    FILESYSTEM(Option<String>),
    TLDR(String),
    CHEATSH(String),
}

pub enum Action {
    SAVE(String),
    PRINT,
    EXECUTE,
}

impl Config {
    pub fn source(&self) -> Source {
        if let Some(query) = self.tldr.clone() {
            Source::TLDR(query)
        } else if let Some(query) = self.cheatsh.clone() {
            Source::CHEATSH(query)
        } else {
            Source::FILESYSTEM(self.path.clone())
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
            _ => {
                let q = self.query.clone();
                if q.is_some() {
                    return q;
                }
                if self.get_best_match() {
                    match self.source() {
                        Source::TLDR(q) => Some(q),
                        Source::CHEATSH(q) => Some(q),
                        _ => Some(String::from("")),
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn get_best_match(&self) -> bool {
        if let Some(Command::Best { .. }) = &self.cmd {
            deprecated("best <query>");
            true
        } else {
            self.best_match
        }
    }

    pub fn autoselect(&self) -> bool {
        if self.no_autoselect {
            deprecated("--no-autoselect");
            false
        } else {
            true
        }
    }
}

pub fn config_from_env() -> Config {
    Config::from_args()
}

pub fn config_from_iter(args: Vec<&str>) -> Config {
    Config::from_iter(args)
}
