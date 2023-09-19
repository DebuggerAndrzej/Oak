mod argparser;
mod plant_repo;
mod utils;

use utils::{color_print, PrintType};

fn main() {
    let matches = argparser::cli().get_matches();

    match matches.subcommand() {
        Some(("plant", _)) => {
            plant_repo::plant_repo();
        }
        Some(("config", args)) => {
            println!("Configuring oak...");
            println!("{:?}", args.get_one::<String>("username"));
            if args.contains_id("editor") {
                println!("{:?}", args.get_one::<String>("editor").unwrap());
            }
        }
        _ => {
            color_print("Wrong command", &PrintType::Error);
        }
    }
}
