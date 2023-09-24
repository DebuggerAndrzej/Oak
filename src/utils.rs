use colored::Colorize;
use std::env;
use std::path::Path;

pub enum PrintType {
    Warning,
    Error,
    Success,
}

pub fn color_print(text: &str, print_type: &PrintType) {
    match print_type {
        PrintType::Warning => println!("{}", text.yellow()),
        PrintType::Error => println!("{}", text.red()),
        PrintType::Success => println!("{}", text.green()),
    }
}

pub fn is_oak_repo() -> bool {
    let mut path = env::current_dir().unwrap();
    let folder = Path::new(".oak");

    loop {
        path.push(folder);

        if path.is_dir() {
            return true;
        }

        if !(path.pop() && path.pop()) {
            return false;
        }
    }
}
