use std::{fmt::Debug, fs, process::exit};

use crate::{file::File, paths::PathsTrait, utils::UtilitiesTrait};

pub struct HistoryItem {
    pub absolute_path: String,
    pub name: String,
    pub timestamp: String,
}

pub trait HistoryTrait<P, U> {
    fn read_latest() -> String;
    fn encode_history_item(file: &File<P, U>) -> String;
    fn decode_history_item(history_item: &str) -> HistoryItem;
    fn add_to_history(file: &File<P, U>) -> Result<(), std::io::Error>;
    fn remove_latest_from_history() -> Result<(), std::io::Error>;
}

pub struct History<P, U> {
    _marker: std::marker::PhantomData<(P, U)>,
}

impl<P, U> HistoryTrait<P, U> for History<P, U>
where
    P: PathsTrait + Debug,
    U: UtilitiesTrait + Debug,
{
    fn read_latest() -> String {
        let history_file = fs::read_to_string(P::get_history_file_path()).unwrap();
        let contents: Vec<&str> = history_file.split('\n').collect();
        if contents.is_empty() {
            println!("No history");
            exit(0)
        }
        String::from(contents[0])
    }

    fn encode_history_item(file: &File<P, U>) -> String {
        format!(
            "{}::{}::{}",
            file.original_absolute_path.to_str().unwrap(),
            file.name,
            U::get_timestamp()
        )
    }

    fn decode_history_item(history_item: &str) -> HistoryItem {
        let split = history_item.split("::").collect::<Vec<&str>>();
        match split.as_slice() {
            [absolute_path, name, timestamp] => HistoryItem {
                absolute_path: String::from(*absolute_path),
                name: String::from(*name),
                timestamp: String::from(*timestamp),
            },
            _ => {
                println!("Invalid history file");
                exit(1)
            }
        }
    }

    fn add_to_history(file: &File<P, U>) -> Result<(), std::io::Error> {
        let history_file_path = P::get_history_file_path();
        let history_file_contents = fs::read_to_string(&history_file_path).unwrap();
        let history_item = Self::encode_history_item(file);

        let updated_file_contents = format!("{}\n{}", history_item, history_file_contents);
        fs::write(&history_file_path, updated_file_contents)
    }

    fn remove_latest_from_history() -> Result<(), std::io::Error> {
        let history_file_contents = fs::read_to_string(P::get_history_file_path()).unwrap();

        // split and drop the first line
        let updated_file_contents =
            history_file_contents.split('\n').collect::<Vec<&str>>()[1..].join("\n");

        fs::write(P::get_history_file_path(), updated_file_contents)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use crate::{
        paths::{check_file_exists_else_create, MockPaths},
        utils::MockUtilities,
    };

    #[test]
    fn test_read_latest_with_non_empty_history() {
        let test = TempDir::new_in(".").unwrap();
        fs::create_dir(test.path().join(".temp_dir")).unwrap();
        println!("test: {:?}", test);

        check_file_exists_else_create::<MockPaths>();

        fs::write(MockPaths::get_history_file_path(), "latest_test_entry").unwrap();

        println!(
            "here: {}",
            fs::read_to_string(MockPaths::get_history_file_path()).unwrap()
        );

        let latest = History::<MockPaths, MockUtilities>::read_latest();
        println!("here: {}", latest);
        assert_eq!(latest, "latest_test_entry")
    }

    // #[test]
    // fn test_read_latest_with_empty_history() {
    //     let home_dir = MockPaths::get_home_path();
    //     check_file_exists_else_create::<MockPaths>(&home_dir);
    //     let history_file_path = MockPaths::get_history_file_path(&home_dir);
    //     fs::write(history_file_path, "").unwrap();
    //     assert!(read_latest().eq(""));
    // }

    #[test]
    fn test_encode_history_item() {
        let file = File::<MockPaths, MockUtilities>::new_from_name("test_file");
        // little hacky but if someone changes the formatting logic in the function, this will
        // catch it.
        // I would love to write just a string here but because of the tempdir it will never be the same
        let test_encoding = format!(
            "{}::{}::{}",
            file.original_absolute_path.to_str().unwrap(),
            file.name,
            MockUtilities::get_timestamp()
        );

        assert!(History::encode_history_item(&file).eq(&test_encoding));
    }

    #[test]
    fn test_decode_history_item() {
        let history_item = "test_file::test_file::0";
        let decoded = History::<MockPaths, MockUtilities>::decode_history_item(history_item);
        assert!(decoded.absolute_path.eq("test_file"));
        assert!(decoded.name.eq("test_file"));
        assert!(decoded.timestamp.eq("0"));
    }
}
