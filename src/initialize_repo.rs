use crate::utils::{color_print, PrintType};
use std::fs;
use std::path::Path;

pub fn initialize_repo() {
    let rpv_dir = ".rpv";

    if Path::new(rpv_dir).is_dir() {
        color_print("Repository already initialized!", &PrintType::Warning);
        return;
    }

    match fs::create_dir(rpv_dir) {
        Ok(_) => color_print("Repository successfully initialized", &PrintType::Success),
        Err(_) => color_print("Couldn't initialize repository", &PrintType::Error),
    };
}
