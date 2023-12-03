use std::{fs, process::exit};

use crate::{file::File, paths::get_history_file_path, utils::get_timestamp};
pub fn read_latest() -> String {
    let history_file = fs::read_to_string(get_history_file_path()).unwrap();
    let contents: Vec<&str> = history_file.split('\n').collect();
    if contents.is_empty() {
        println!("No history");
        exit(0)
    }
    String::from(contents[0])
}

fn encode_history_item(file: &File) -> String {
    format!(
        "{}::{}::{}",
        file.original_absolute_path.to_str().unwrap(),
        file.name,
        get_timestamp()
    )
}

pub struct HistoryItem {
    pub absolute_path: String,
    pub name: String,
    pub timestamp: String,
}

pub fn decode_history_item(history_item: &str) -> HistoryItem {
    let split = history_item.split("::").collect::<Vec<&str>>();
    match split.as_slice() {
        [absolute_path, name, timestamp] => HistoryItem {
            absolute_path: String::from(*absolute_path),
            name: String::from(*name),
            timestamp: String::from(*timestamp),
        },
        _ => {
            panic!("Invalid history file");
        }
    }
}

pub fn add_to_history(file: &File) -> Result<(), std::io::Error> {
    let history_file_path = get_history_file_path();
    let history_file_contents = fs::read_to_string(&history_file_path).unwrap();
    let history_item = encode_history_item(file);

    let updated_file_contents = format!("{}\n{}", history_item, history_file_contents);
    fs::write(&history_file_path, updated_file_contents)
}

pub fn remove_latest_from_history() -> Result<(), std::io::Error> {
    let history_file_contents = fs::read_to_string(get_history_file_path()).unwrap();

    // split and drop the first line
    let updated_file_contents =
        history_file_contents.split('\n').collect::<Vec<&str>>()[1..].join("\n");

    fs::write(get_history_file_path(), updated_file_contents)
}
