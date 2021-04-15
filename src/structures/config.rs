use crate::cmds::func::Func;
use crate::cmds::info::Info;
use crate::env_var;
use crate::finder::FinderChoice;
use crate::shell::Shell;
use clap::{crate_version, AppSettings, Clap};
use std::str::FromStr;

const FINDER_POSSIBLE_VALUES: &[&str] = &[&"fzf", &"skim"];
const SHELL_POSSIBLE_VALUES: &[&str] = &[&"bash", &"zsh", &"fish"];
const FUNC_POSSIBLE_VALUES: &[&str] = &[&"url::open", &"welcome", &"widget::last_command", &"map::expand"];
const INFO_POSSIBLE_VALUES: &[&str] = &[&"cheats-path"];

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
            "widget::last_command" => Ok(Func::WidgetLastCommand),
            "map::expand" => Ok(Func::MapExpand),
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

MORE ENVIRONMENT VARIABLES:
    NAVI_TAG_WIDTH                               # tag column width as window integer %
    NAVI_COMMENT_WIDTH                           # comment column width as window integer %
    NAVI_SHELL                                   # shell used in shell outs

EXAMPLES:
    navi                                         # default behavior
    navi --print                                 # doesn't execute the snippet
    navi --tldr docker                           # search for docker cheatsheets using tldr
    navi --cheatsh docker                        # search for docker cheatsheets using cheatsh
    navi --path '/some/dir:/other/dir'           # use .cheat files from custom paths
    navi --query git                             # filter results by "git"
    navi --query 'create db' --best-match        # autoselect the snippet that best matches a query
    db=my navi --query 'create db' --best-match  # same, but set the value for the <name> variable
    navi repo add denisidoro/cheats              # import cheats from a git repository
    eval "$(navi widget zsh)"                    # load the zsh widget
    navi --finder 'skim'                         # set skim as finder, instead of fzf
    navi --fzf-overrides '--with-nth 1,2'        # show only the comment and tag columns
    navi --fzf-overrides '--no-select-1'         # prevent autoselection in case of single line
    navi --fzf-overrides-var '--no-select-1'     # same, but for variable selection
    navi --fzf-overrides '--nth 1,2'             # only consider the first two columns for search
    navi --fzf-overrides '--no-exact'            # use looser search algorithm
    NAVI_SHELL=dash navi                         # use dash in shell outs
    NAVI_TAG_WIDTH=30 NAVI_COMMENT_WIDTH=40 navi # customize column widths
    navi --tag-rules='git,!checkout'             # show non-checkout git snippets only"#)]
#[clap(setting = AppSettings::ColorAuto)]
#[clap(setting = AppSettings::ColoredHelp)]
#[clap(setting = AppSettings::AllowLeadingHyphen)]
#[clap(version = crate_version!())]
pub struct Config {
    /// List of :-separated paths containing .cheat files
    #[clap(short, long, env = env_var::PATH)]
    pub path: Option<String>,

    /// Instead of executing a snippet, prints it to stdout
    #[clap(long)]
    print: bool,

    /// Hides preview window
    #[clap(long)]
    pub no_preview: bool,

    /// Returns the best match
    #[clap(long)]
    pub best_match: bool,

    /// Search for cheatsheets using the tldr-pages repository
    #[clap(long)]
    tldr: Option<String>,

    /// [Experimental] Comma-separated list that acts as filter for tags. Parts starting with ! represent negation
    #[clap(long)]
    tag_rules: Option<String>,

    /// Search for cheatsheets using the cheat.sh repository
    #[clap(long)]
    cheatsh: Option<String>,

    /// Query
    #[clap(short, long)]
    query: Option<String>,

    /// finder overrides for cheat selection
    #[clap(long, env = env_var::FZF_OVERRIDES)]
    pub fzf_overrides: Option<String>,

    /// finder overrides for variable selection
    #[clap(long, env = env_var::FZF_OVERRIDES_VAR)]
    pub fzf_overrides_var: Option<String>,

    /// which finder application to use
    #[clap(long, env = env_var::FINDER, default_value = "fzf", possible_values = FINDER_POSSIBLE_VALUES, case_insensitive = true)]
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
    /// [Experimental] Performs ad-hoc, internal functions provided by navi
    Fn {
        /// Function name (example: "url::open")
        #[clap(possible_values = FUNC_POSSIBLE_VALUES, case_insensitive = true)]
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
    /// Outputs shell widget source code
    Widget {
        #[clap(possible_values = SHELL_POSSIBLE_VALUES, case_insensitive = true, default_value = "bash")]
        shell: Shell,
    },
    /// Shows info
    Info {
        #[clap(possible_values = INFO_POSSIBLE_VALUES, case_insensitive = true)]
        info: Info,
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

pub enum Source {
    Filesystem(Option<String>, Option<String>),
    Tldr(String),
    Cheats(String),
}

pub enum Action {
    Print,
    Execute,
}

impl Config {
    pub fn source(&self) -> Source {
        if let Some(query) = self.tldr.clone() {
            Source::Tldr(query)
        } else if let Some(query) = self.cheatsh.clone() {
            Source::Cheats(query)
        } else {
            Source::Filesystem(self.path.clone(), self.tag_rules.clone())
        }
    }

    pub fn action(&self) -> Action {
        if self.print {
            Action::Print
        } else {
            Action::Execute
        }
    }

    pub fn get_query(&self) -> Option<String> {
        let q = self.query.clone();
        if q.is_some() {
            return q;
        }
        if self.best_match {
            match self.source() {
                Source::Tldr(q) => Some(q),
                Source::Cheats(q) => Some(q),
                _ => Some(String::from("")),
            }
        } else {
            None
        }
    }
}

pub fn config_from_env() -> Config {
    Config::parse()
}

pub fn config_from_iter(args: Vec<&str>) -> Config {
    Config::parse_from(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_possible_values() {
        for v in SHELL_POSSIBLE_VALUES {
            assert_eq!(true, Shell::from_str(v).is_ok())
        }
    }

    #[test]
    fn test_info_possible_values() {
        for v in INFO_POSSIBLE_VALUES {
            assert_eq!(true, Info::from_str(v).is_ok())
        }
    }

    #[test]
    fn test_func_possible_values() {
        for v in FUNC_POSSIBLE_VALUES {
            assert_eq!(true, Func::from_str(v).is_ok())
        }
    }

    #[test]
    fn test_finder_possible_values() {
        for v in FINDER_POSSIBLE_VALUES {
            assert_eq!(true, FinderChoice::from_str(v).is_ok())
        }
    }
}
