use crate::cmds::func::Func;
use crate::cmds::info::Info;
use crate::common::shell::Shell;
use crate::env_vars;
use crate::finder::FinderChoice;
use clap::{crate_version, AppSettings, Clap};
use std::str::FromStr;

static mut NOTIFIED_DEPRECATION: bool = false;

impl FromStr for FinderChoice {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fzf" => Ok(FinderChoice::Fzf),
            "skim" => Ok(FinderChoice::Skim),
            _ => Err("no match"),
        }
    }
}

impl FromStr for Shell {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bash" => Ok(Shell::Bash),
            "zsh" => Ok(Shell::Zsh),
            "fish" => Ok(Shell::Fish),
            _ => Err("no match"),
        }
    }
}

impl FromStr for Func {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "url::open" => Ok(Func::UrlOpen),
            "welcome" => Ok(Func::Welcome),
            _ => Err("no match"),
        }
    }
}

impl FromStr for Info {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cheats-path" => Ok(Info::CheatsPath),
            _ => Err("no match"),
        }
    }
}

#[derive(Debug, Clap)]
#[clap(after_help = r#"MORE INFO:
    Please refer to https://github.com/denisidoro/navi

EXAMPLES:
    navi                                             # default behavior
    navi --print                                     # doesn't execute the snippet
    navi --tldr docker                               # search for docker cheatsheets using tldr
    navi --cheatsh docker                            # search for docker cheatsheets using cheatsh
    navi --path '/some/dir:/other/dir'               # use .cheat files from custom paths
    navi --query git                                 # filter results by "git"
    navi --query 'create db' --best-match            # autoselect the snippet that best matches a query
    name=mydb navi --query 'create db' --best-match  # same, but set the value for the <name> variable
    navi repo add denisidoro/cheats                  # import cheats from a git repository
    eval "$(navi widget zsh)"                        # load the zsh widget
    navi --finder 'skim'                             # set skim as finder, instead of fzf
    navi --fzf-overrides '--with-nth 1,2'            # show only the comment and tag columns
    navi --fzf-overrides '--no-select-1'             # prevent autoselection in case of single line
    navi --fzf-overrides '--nth 1,2'                 # only consider the first two columns for search
    navi --fzf-overrides '--no-exact'                # use looser search algorithm"#)]
#[clap(setting = AppSettings::ColorAuto)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(setting = AppSettings::AllowLeadingHyphen)]
#[clap(version = crate_version!())]
pub struct Config {
    /// List of :-separated paths containing .cheat files
    #[clap(short, long, env = env_vars::PATH)]
    pub path: Option<String>,

    /// [Experimental] Instead of executing a snippet, saves it to a file
    #[clap(short, long)]
    save: Option<String>,

    /// Instead of executing a snippet, prints it to stdout
    #[clap(long)]
    print: bool,

    /// Prevents autoselection in case of single entry
    #[clap(long)]
    no_autoselect: bool,

    /// Hides preview window
    #[clap(long)]
    pub no_preview: bool,

    /// Returns the best match
    #[clap(long)]
    best_match: bool,

    /// Search for cheatsheets using the tldr-pages repository
    #[clap(long)]
    tldr: Option<String>,

    /// Search for cheatsheets using the cheat.sh repository
    #[clap(long)]
    cheatsh: Option<String>,

    /// Query
    #[clap(short, long)]
    query: Option<String>,

    /// finder overrides for cheat selection
    #[clap(long, env = env_vars::FZF_OVERRIDES)]
    pub fzf_overrides: Option<String>,

    /// finder overrides for variable selection
    #[clap(long, env = env_vars::FZF_OVERRIDES_VAR)]
    pub fzf_overrides_var: Option<String>,

    /// which finder application to use
    #[clap(long, env = env_vars::FINDER, default_value = "fzf", possible_values = &["fzf", "skim"], case_insensitive = true)]
    pub finder: FinderChoice,

    #[clap(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Debug, Clap)]
pub enum Command {
    /// Filters results
    #[clap(setting = AppSettings::Hidden)]
    Query {
        /// String used as filter (example: "git")
        query: String,
    },
    /// Autoselects the snippet that best matches the query
    #[clap(setting = AppSettings::Hidden)]
    Best {
        /// String used as filter (example: "git remove branch")
        query: String,
        /// List of arguments (example: "mybranch" "remote")
        args: Vec<String>,
    },
    /// Performs ad-hoc functions provided by navi
    Fn {
        /// Function name (example: "url::open")
        #[clap(possible_values = &["url::welcome", "open"], case_insensitive = true)]
        func: Func,
        /// List of arguments (example: "https://google.com")
        args: Vec<String>,
    },
    /// Manages cheatsheet repositories
    Repo {
        #[clap(subcommand)]
        cmd: RepoCommand,
    },
    /// Used for fzf's preview window when selecting snippets
    #[clap(setting = AppSettings::Hidden)]
    Preview {
        /// Selection line
        line: String,
    },
    /// Used for fzf's preview window when selecting variable suggestions
    #[clap(setting = AppSettings::Hidden)]
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
        #[clap(possible_values = &["bash", "zsh", "fish"], case_insensitive = true, default_value = "bash")]
        shell: Shell,
    },
    /// Shows info
    Info {
        #[clap(possible_values = &["cheats-path"], case_insensitive = true)]
        info: Info,
    },
    /// Helper command for Alfred integration
    #[clap(setting = AppSettings::Hidden)]
    Alfred {
        #[clap(subcommand)]
        cmd: AlfredCommand,
    },
}

#[derive(Debug, Clap)]
pub enum RepoCommand {
    /// Imports cheatsheets from a repo
    Add {
        /// A URI to a git repository containing .cheat files ("user/repo" will download cheats from github.com/user/repo)
        uri: String,
    },
    /// Browses for featured cheatsheet repos
    Browse,
}

#[derive(Debug, Clap)]
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
    Config::parse()
}

pub fn config_from_iter(args: Vec<&str>) -> Config {
    Config::parse_from(args)
}
