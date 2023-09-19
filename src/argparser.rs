use clap::{Arg, Command};

pub fn cli() -> Command {
    Command::new("oak")
        .about("A fictional versioning CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(Command::new("plant").about("Plants oak repository"))
        .subcommand(
            Command::new("config")
                .about("changes oak configuration")
                .arg_required_else_help(true)
                .arg(
                    Arg::new("username")
                        .long("username")
                        .help("User will be using to sign leafs"),
                )
                .arg(Arg::new("editor").long("editor").help("Editor used in oak")),
        )
}
