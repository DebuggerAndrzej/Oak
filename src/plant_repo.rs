use crate::utils::{color_print, is_oak_repo, PrintType};
use std::fs;

pub fn plant_repo() {
    let oak_dir = ".oak";

    if is_oak_repo() {
        color_print("Repository already planted!", &PrintType::Warning);
        return;
    }

    match fs::create_dir(oak_dir) {
        Ok(_) => color_print("Repository successfully planted", &PrintType::Success),
        Err(_) => color_print("Couldn't plant repository", &PrintType::Error),
    };
}
