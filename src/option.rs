use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Config {
    #[structopt(name = "supervisor", default_value = "Puck", long = "supervisor")]
    supervising_faerie: String,
    /// The faerie tree this cookie is being made in.
    tree: Option<String>,
    #[structopt(subcommand)]  // Note that we mark a field as a subcommand
    cmd: Command
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Pound acorns into flour for cookie dough.
    Pound {
        acorns: u32
    },
    /// Add magical sparkles -- the secret ingredient!
    Sparkle {
        #[structopt(short, parse(from_occurrences))]
        magicality: u64,
        #[structopt(short)]
        color: String
    },
    Finish(Finish),
}

// Subcommand can also be externalized by using a 1-uple enum variant
#[derive(Debug, StructOpt)]
struct Finish {
    #[structopt(short)]
    time: u32,
    #[structopt(subcommand)]  // Note that we mark a field as a subcommand
    finish_type: FinishType
}

// subsubcommand!
#[derive(Debug, StructOpt)]
enum FinishType {
    Glaze {
        applications: u32
    },
    Powder {
        flavor: String,
        dips: u32
    }
}

pub fn parse() -> Config {
    Config::from_args()
}
