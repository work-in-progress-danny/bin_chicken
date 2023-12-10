use std::{
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
};

use dialoguer::console;

use crate::{
    history::{History, HistoryTrait},
    paths::PathsTrait,
    utils::UtilitiesTrait,
};

#[derive(Debug)]
pub struct File<P, U> {
    pub history_item: String,
    pub original_absolute_path: PathBuf,
    pub name_with_timestamp: String,
    pub name: String,
    pub trash_path: PathBuf,
    _marker: std::marker::PhantomData<(P, U)>,
}

impl<P, U> File<P, U>
where
    P: PathsTrait + Debug,
    U: UtilitiesTrait + Debug,
{
    /// returns a string of the format:
    /// /home/username/.bin_chicken/trash/absolute_path item_name::timestamp
    pub fn new_from_name(item: &str) -> File<P, U> {
        let timestamp = U::get_timestamp();
        let absolute_path = P::get_absolute_path(item);
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
            trash_path: Path::join(&P::get_trash_path(), name_with_timestamp),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn new_from_history() -> File<P, U> {
        let latest = History::<P, U>::read_latest();
        let history_item = History::<P, U>::decode_history_item(&latest);
        let name_with_timestamp = format!("{}::{}", history_item.name, history_item.timestamp);

        File {
            name: history_item.name,
            original_absolute_path: PathBuf::from(history_item.absolute_path),
            name_with_timestamp: String::from(&name_with_timestamp),
            history_item: String::from(&latest),
            trash_path: Path::join(&P::get_trash_path(), name_with_timestamp),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn move_to_trash(&self) {
        U::exit_if_not_valid_directory(&self.name);

        match fs::rename(&self.original_absolute_path, &self.trash_path) {
            Ok(_) => {
                History::<P, U>::add_to_history(self).unwrap();
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
        match fs::rename(&self.trash_path, &self.original_absolute_path) {
            Ok(_) => {
                History::<P, U>::remove_latest_from_history().unwrap();
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
