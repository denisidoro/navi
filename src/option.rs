        use clap::{App, Arg, ArgMatches, SubCommand};

        pub fn parse<'a>() -> ArgMatches<'a>  {
            App::new("navi")
            .version("0.1.0")
            .about("An interactive cheatsheet tool for the command line")
            .subcommand(
                SubCommand::with_name("widget")
                    .about("returns the absolute path of shell widgets")
                    .arg(
                        Arg::with_name("shell")
                            .help("zsh, bash or fish")
                            .index(1)
                            .required(true),
                    ),
            )
            .subcommand(
                SubCommand::with_name("preview")
                    .about("[internal] pretty-prints a line selection")
                    .arg(Arg::with_name("line").index(1).required(true)),
            )
            .get_matches()
        }
