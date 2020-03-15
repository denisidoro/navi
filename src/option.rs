use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(after_help = r#"EXAMPLES:
    navi                                   # default behavior
    navi --print                           # doesn't execute the snippet
    navi --path '/some/dir:/other/dir'     # uses custom cheats
    navi search docker                     # uses online data
    navi query git                         # filters results by "git"
    navi best 'sql create db' root mydb    # uses a snippet as a CLI
    navi repo add denisidoro/cheats        # imports cheats from github.com/denisidoro/cheats
    source <(navi widget zsh)              # loads the zsh widget
    navi --fzf-overrides ' --with-nth 1,2' # shows only the comment and tag columns
    navi --fzf-overrides ' --nth 1,2'      # search will consider only the first two columns
    navi --fzf-overrides ' --no-exact'     # looser search algorithm"#)]
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

    /// FZF overrides for cheat selection (must start with an empty space)
    #[structopt(long, env = "NAVI_FZF_OVERRIDES")]
    pub fzf_overrides: Option<String>,

    /// FZF overrides for variable selection (must start with an empty space)
    #[structopt(long, env = "NAVI_FZF_OVERRIDES_VAR")]
    pub fzf_overrides_var: Option<String>,

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
    /// Shows the path for shell widget files
    Widget {
        /// bash, zsh or fish
        shell: String,
    },
}

#[derive(Debug, StructOpt)]
pub enum RepoCommand {
    /// Imports cheatsheets from a repo
    Add {
        /// A URI to a .tar.gz containing .cheat file\n
        /// ("user/repo" will download cheats from github.com/user/repo)
        uri: String,
    },
}

pub enum InternalCommand {
    Preview { line: String },
}

pub fn config_from_env() -> Config {
    Config::from_args()
}

pub fn config_from_iter(args: Vec<&str>) -> Config {
    Config::from_iter(args)
}

pub fn internal_command_from_env() -> Option<InternalCommand> {
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
