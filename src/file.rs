use std::{
    fs,
    path::{Path, PathBuf},
};

use dialoguer::console;

use crate::{
    history,
    paths::get_trash_path,
    utils::{exit_if_not_valid_directory, get_absolute_path, get_timestamp},
};

#[derive(Debug)]
pub struct File {
    pub history_item: String,
    pub original_absolute_path: PathBuf,
    pub name_with_timestamp: String,
    pub name: String,
    trash_path: PathBuf,
}

impl File {
    /// returns a string of the format:
    /// /home/username/.bin_chicken/trash/absolute_path item_name::timestamp
    pub fn new_from_name(item: &str) -> File {
        let timestamp = get_timestamp();
        let absolute_path = get_absolute_path(item);
        let name = Path::new(item).file_name().unwrap().to_str().unwrap();
        let name_with_timestamp = format!("{}::{}", name, timestamp);

        File {
            history_item: format!(
                "{}::{}::{}",
                absolute_path.to_str().unwrap(),
                name,
                timestamp
            ),
            original_absolute_path: absolute_path,
            name: String::from(name),
            name_with_timestamp: String::from(&name_with_timestamp),
            trash_path: Path::join(&get_trash_path(), name_with_timestamp),
        }
    }

    pub fn new_from_history() -> File {
        let latest = history::read_latest();
        let history_item = history::decode_history_item(&latest);
        let name_with_timestamp = format!("{}::{}", history_item.name, history_item.timestamp);

        File {
            name: history_item.name,
            original_absolute_path: PathBuf::from(history_item.absolute_path),
            name_with_timestamp: String::from(&name_with_timestamp),
            history_item: String::from(&latest),
            trash_path: Path::join(&get_trash_path(), name_with_timestamp),
        }
    }

    pub fn move_to_trash(&self) {
        exit_if_not_valid_directory(&self.name);

        match fs::rename(&self.original_absolute_path, &self.trash_path) {
            Ok(_) => {
                history::add_to_history(self).unwrap();
                println!("moved {:?} to trash", self.name)
            }
            Err(_) => println!(
                "{}: failed to move {:?} to trash",
                console::style("Error").red(),
                &self.name
            ),
        }
    }

    pub fn restore_from_trash(&self) {
        println!("{:#?}", &self);
        match fs::rename(&self.trash_path, &self.original_absolute_path) {
            Ok(_) => {
                history::remove_latest_from_history().unwrap();
                println!("moved {:?} from trash", self.name)
            }
            Err(_) => println!(
                "{}: failed to move {:?} from trash",
                console::style("Error").red(),
                &self.name
            ),
        }
    }
}
