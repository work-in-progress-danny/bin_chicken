use std::{
    env, fs,
    path::{Path, PathBuf},
};

use tempfile::TempDir;

pub fn check_file_exists_else_create<P: PathsTrait>() {
    let bin_chicken_dir = P::get_bin_chicken_path();
    let trash_dir = P::get_trash_path();
    let history_file = P::get_history_file_path();

    if !bin_chicken_dir.exists() {
        println!("Creating {}", bin_chicken_dir.to_str().unwrap());
        println!("{:?}", bin_chicken_dir);
        fs::create_dir(bin_chicken_dir).expect("something went wrong");
    };

    if !trash_dir.exists() {
        println!("Creating {}", trash_dir.to_str().unwrap());
        fs::create_dir(trash_dir).unwrap();
    };

    if !history_file.exists() {
        println!("Creating {}", history_file.to_str().unwrap());
        fs::write(history_file, "").unwrap();
    };
}

#[derive(Debug)]
pub struct ProdPaths;

impl PathsTrait for ProdPaths {}

pub trait PathsTrait {
    fn get_home_path() -> PathBuf {
        dirs::home_dir().expect("Could not find home directory")
    }

    fn get_bin_chicken_path() -> PathBuf {
        Path::join(&Self::get_home_path(), ".bin_chicken")
    }

    fn get_trash_path() -> PathBuf {
        Path::join(&Self::get_bin_chicken_path(), "trash")
    }

    fn get_history_file_path() -> PathBuf {
        Path::join(&Self::get_bin_chicken_path(), "history.txt")
    }

    fn get_absolute_path(path: &str) -> PathBuf {
        let path = Path::new(path).file_name().unwrap().to_str().unwrap();
        let env_path = env::current_dir().unwrap();
        Path::join(&env_path, path)
    }
}

#[derive(Debug)]
pub struct MockPaths;

impl PathsTrait for MockPaths {
    fn get_home_path() -> PathBuf {
        std::env::temp_dir()
    }
    fn get_bin_chicken_path() -> PathBuf {
        Path::join(&Self::get_home_path(), ".bin_chicken")
    }
    fn get_trash_path() -> PathBuf {
        Path::join(&Self::get_bin_chicken_path(), "trash")
    }
    fn get_history_file_path() -> PathBuf {
        Path::join(&Self::get_bin_chicken_path(), "history.txt")
    }

    fn get_absolute_path(path: &str) -> PathBuf {
        let temp_path = tempfile::tempdir().unwrap().path().to_path_buf();
        Path::join(&temp_path, path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_file_exists_else_create() {
        let bin_chicken_dir = MockPaths::get_bin_chicken_path();
        let trash_dir = MockPaths::get_trash_path();
        let history_file = MockPaths::get_history_file_path();
        println!("bin_chicken_dir: {:?}", bin_chicken_dir);
        println!("bin_chicken_dir: {:?}", bin_chicken_dir.is_dir());

        assert!(!bin_chicken_dir.exists());
        assert!(!trash_dir.exists());
        assert!(!history_file.exists());

        check_file_exists_else_create::<MockPaths>();

        assert!(bin_chicken_dir.exists());
        assert!(trash_dir.exists());
        assert!(history_file.exists());
    }
}
