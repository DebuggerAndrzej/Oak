use crate::utils::{color_print, PrintType};
use std::fs;
use std::path::Path;

pub fn plant_repo() {
    let oak_dir = ".oak";

    if Path::new(oak_dir).is_dir() {
        color_print("Repository already planted!", &PrintType::Warning);
        return;
    }

    match fs::create_dir(oak_dir) {
        Ok(_) => color_print("Repository successfully planted", &PrintType::Success),
        Err(_) => color_print("Couldn't plant repository", &PrintType::Error),
    };
}
