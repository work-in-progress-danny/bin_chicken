use std::{
    env,
    path::{Path, PathBuf},
    process::exit,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub fn exit_if_not_valid_directory(path: &str) {
    if !Path::new(path).exists() {
        println!("\"{}\" doesn't exist", path);
        exit(1)
    }
}

pub fn get_absolute_path(path: &str) -> PathBuf {
    let path = Path::new(path).file_name().unwrap().to_str().unwrap();
    let env_path = env::current_dir().unwrap();
    Path::join(&env_path, path)
}
