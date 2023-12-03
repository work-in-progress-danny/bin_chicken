use std::{
    env::home_dir,
    fs,
    path::{Path, PathBuf},
};

pub fn check() {
    if !get_bin_chicken_path().exists() {
        println!("Creating ~/.bin_chicken/");
        fs::create_dir(&get_bin_chicken_path()).unwrap();
    };

    if !get_trash_path().exists() {
        println!("Creating ~/.bin_chicken/trash/");
        fs::create_dir(&get_trash_path()).unwrap();
    };

    if !get_history_file_path().exists() {
        println!("Creating ~/.bin_chicken/history");
        fs::write(&get_history_file_path(), "").unwrap();
    };
}

fn get_home_path() -> PathBuf {
    home_dir().unwrap()
}

fn get_bin_chicken_path() -> PathBuf {
    Path::join(&get_home_path(), ".bin_chicken")
}

pub fn get_trash_path() -> PathBuf {
    Path::join(&get_bin_chicken_path(), "trash")
}

pub fn get_history_file_path() -> PathBuf {
    Path::join(&get_bin_chicken_path(), "history")
}
