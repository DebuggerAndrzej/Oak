use colored::Colorize;

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
