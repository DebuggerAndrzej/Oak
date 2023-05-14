mod argparser;

fn main() {
    let matches = argparser::cli().get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            println!("Initializing repo...");
        }
        Some(("config", args)) => {
            println!("Configuring rpv...");
            println!("{:?}", args.get_one::<String>("username"));
            if args.contains_id("editor") {
                println!("{:?}", args.get_one::<String>("editor").unwrap())
            }
        }
        _ => {
            println!("Wrong command");
        }
    }
}
