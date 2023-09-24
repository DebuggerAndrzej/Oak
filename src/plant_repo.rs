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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::Path;

    #[test]
    fn test_oak_folder_is_created() {
        let dir = env::temp_dir();
        let _ = env::set_current_dir(&dir);
        plant_repo();
        assert!(Path::new(".oak").is_dir())
    }

    #[test]
    fn test_oak_is_detected_and_is_not_created_again_in_subfolder() {
        let dir = env::temp_dir();
        env::set_current_dir(&dir).unwrap();
        plant_repo();
        assert!(Path::new(".oak").is_dir());

        let subfolder_path = dir.join("some").join("nested").join("subfolder");
        fs::create_dir_all(&subfolder_path).unwrap();
        env::set_current_dir(subfolder_path).unwrap();
        assert!(!Path::new(".oak").is_dir())
    }
}
