use crate::commands;
use crate::finder::FinderChoice;

use clap::{crate_version, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(after_help = "\x1b[0;33mMORE INFO:\x1b[0;0m
    Please refer to \x1b[0;32mhttps://github.com/denisidoro/navi\x1b[0;0m

\x1b[0;33mENVIRONMENT VARIABLES:\x1b[0m
    \x1b[0;32mNAVI_CONFIG\x1b[0;0m            # path to config file
    \x1b[0;32mNAVI_CONFIG_YAML\x1b[0;0m       # config file content

\x1b[0;33mFEATURE STABILITY:\x1b[0m
    \x1b[0;32mexperimental\x1b[0;0m           # may be removed or changed at any time
    \x1b[0;32mdeprecated\x1b[0;0m             # may be removed in 3 months after first being deprecated

\x1b[0;33mCOMMON NAVI COMMANDS:\x1b[0m
    Run \x1b[0;32mnavi fn welcome\x1b[0;0m to browse the cheatsheet for navi itself

\x1b[0;33mEXAMPLES:\x1b[0m
    navi                                         # default behavior
    navi fn welcome                              # show cheatsheets for navi itself
    navi --print                                 # doesn't execute the snippet
    navi --tldr docker                           # search for docker cheatsheets using tldr
    navi --cheatsh docker                        # search for docker cheatsheets using cheatsh
    navi --path '/some/dir:/other/dir'           # use .cheat files from custom paths
    navi --query git                             # filter results by \"git\"
    navi --query 'create db' --best-match        # autoselect the snippet that best matches a query
    db=my navi --query 'create db' --best-match  # same, but set the value for the <name> variable
    navi repo add denisidoro/cheats              # import cheats from a git repository
    eval \"$(navi widget zsh)\"                    # load the zsh widget
    navi --finder 'skim'                         # set skim as finder, instead of fzf
    navi --fzf-overrides '--with-nth 1,2'        # show only the comment and tag columns
    navi --fzf-overrides '--no-select-1'         # prevent autoselection in case of single line
    navi --fzf-overrides-var '--no-select-1'     # same, but for variable selection
    navi --fzf-overrides '--nth 1,2'             # only consider the first two columns for search
    navi --fzf-overrides '--no-exact'            # use looser search algorithm
    navi --tag-rules='git,!checkout'             # show non-checkout git snippets only")]
#[clap(version = crate_version!())]
pub(super) struct ClapConfig {
    /// Colon-separated list of paths containing .cheat files
    #[arg(short, long)]
    pub path: Option<String>,

    /// Instead of executing a snippet, prints it to stdout
    #[arg(long)]
    #[cfg(not(feature = "disable-command-execution"))]
    pub print: bool,

    /// Returns the best match
    #[arg(long)]
    pub best_match: bool,

    /// Prevents variable interpolation
    #[arg(long)]
    pub prevent_interpolation: bool,

    /// Searches for cheatsheets using the tldr-pages repository
    #[arg(long)]
    pub tldr: Option<String>,

    /// [Experimental] Comma-separated list that acts as filter for tags. Parts starting with ! represent negation
    #[arg(long)]
    pub tag_rules: Option<String>,

    /// Searches for cheatsheets using the cheat.sh repository
    #[arg(long)]
    pub cheatsh: Option<String>,

    /// Prepopulates the search field
    #[arg(short, long, allow_hyphen_values = true)]
    pub query: Option<String>,

    /// Finder overrides for snippet selection
    #[arg(long, allow_hyphen_values = true)]
    pub fzf_overrides: Option<String>,

    /// Finder overrides for variable selection
    #[arg(long, allow_hyphen_values = true)]
    pub fzf_overrides_var: Option<String>,

    /// Finder application to use
    #[arg(long, ignore_case = true)]
    pub finder: Option<FinderChoice>,

    #[command(subcommand)]
    pub cmd: Option<Command>,
}

impl ClapConfig {
    pub fn new() -> Self {
        Self::parse()
    }
}

// #[derive(Subcommand, Debug, Clone, Runnable, HasDeps)]
#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// [Experimental] Calls internal functions
    Fn(commands::func::Input),
    /// Manages cheatsheet repositories
    #[cfg(not(feature = "disable-repo-management"))]
    Repo(commands::repo::Input),
    /// Used for fzf's preview window when selecting snippets
    #[command(hide = true)]
    Preview(commands::preview::Input),
    /// Used for fzf's preview window when selecting variable suggestions
    #[command(hide = true)]
    PreviewVar(commands::preview::var::Input),
    /// Used for fzf's preview window when selecting variable suggestions
    #[command(hide = true)]
    PreviewVarStdin(commands::preview::var_stdin::Input),
    /// Outputs shell widget source code
    Widget(commands::shell::Input),
    /// Shows info
    Info(commands::info::Input),
}

#[derive(Debug)]
pub enum Source {
    Filesystem(Option<String>),
    Tldr(String),
    Cheats(String),
    Welcome,
}

pub enum Action {
    Print,
    Execute,
}
