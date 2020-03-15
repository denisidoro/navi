use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(after_help = "EXAMPLES:
    navi                                   # default behavior
    navi --print                           # doesn't execute the snippet
    navi --path '/some/dir:/other/dir'     # uses custom cheats
    navi search docker                     # uses online data
    navi query git                         # filters results by \"git\"
    navi best 'sql create db' root mydb    # uses a snippet as a CLI
    source \"$(navi widget zsh)\"            # loads the zsh widget
    navi --fzf-overrides ' --with-nth 1,2' # shows only the comment and tag columns
    navi --fzf-overrides ' --nth 1,2'      # search will consider only the first two columns
    navi --fzf-overrides ' --no-exact'     # looser search algorithm")]
pub struct Config {
    /// List of :-separated paths containing .cheat files
    #[structopt(short, long, env = "NAVI_PATH")]
    pub path: Option<String>,

    /// [alpha] Instead of executing a snippet, saves it to a file
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

    /// FZF overrides for cheat selection  (must start with an empty space)
    #[structopt(long, env = "NAVI_FZF_OVERRIDES")]
    pub fzf_overrides: Option<String>,

    /// FZF overrides for variable selection  (must start with an empty space)
    #[structopt(long, env = "NAVI_FZF_OVERRIDES_VAR")]
    pub fzf_overrides_var: Option<String>,

    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Filters results
    Query { query: String },
    /// Uses online repositories for cheatsheets
    Search { query: String },
    /// Autoselects the snippet that best matches the query
    Best { query: String, args: Vec<String> },
    /// Performs ad-hoc functions provided by navi
    Fn { func: String, args: Vec<String> },
    /// Repo
    Repo {
        #[structopt(subcommand)]
        cmd: RepoCommand,
    },
    /// Shows the path for shell widget files
    Widget { shell: String },
}

#[derive(Debug, StructOpt)]
pub enum RepoCommand {
    /// Adds a repo (user/repo will download cheats from github.com/user/repo)
    Add { uri: String },
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
