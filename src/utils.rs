use std::{
    path::Path,
    process::exit,
    time::{SystemTime, UNIX_EPOCH},
};

pub trait UtilitiesTrait {
    fn get_timestamp() -> u64;
    fn exit_if_not_valid_directory(path: &str);
}

#[derive(Debug)]
pub struct ProdUtilities {}

impl UtilitiesTrait for ProdUtilities {
    fn get_timestamp() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    fn exit_if_not_valid_directory(path: &str) {
        if !Path::new(path).exists() {
            println!("\"{}\" doesn't exist", path);
            exit(1)
        }
    }
}

#[derive(Debug)]
pub struct MockUtilities {}

impl UtilitiesTrait for MockUtilities {
    fn get_timestamp() -> u64 {
        0
    }
    fn exit_if_not_valid_directory(path: &str) {
        if !Path::new(path).exists() {
            println!("\"{}\" doesn't exist", path);
            exit(1)
        }
    }
}
