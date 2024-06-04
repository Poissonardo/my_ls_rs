use std::cmp::Ordering;
use crate::cli_args_parser::UserOptions;
use crate::error_messages::get_error_message_without_help_indication;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct UserItemEntry {
    path: PathBuf,
    entry: String,
}

impl UserItemEntry {
    pub fn new(entry: String, path: PathBuf) -> UserItemEntry {
        UserItemEntry {path, entry}
    }
}

//filters items requested by user
pub fn get_filtered_user_items(user_options: &UserOptions) -> (Vec<UserItemEntry>, Vec<UserItemEntry>) {
    let mut containing_items = Vec::new();
    let mut non_containing_items = Vec::new();

    for entry in user_options.requested_items() {
        let path = Path::new(entry).to_path_buf();

        if path.is_dir() && !user_options.should_list_dirs() {
            containing_items.push(UserItemEntry::new(entry.to_string(), path));
        } else if path.exists() {
            non_containing_items.push(UserItemEntry::new(entry.to_string(), path));
        } else {
            eprintln!("{}", get_error_message_without_help_indication(format!("cannot access '{}': No such file or directory", entry)));
        }
    }
    (containing_items, non_containing_items)
}

pub fn list_dir_content(entry: UserItemEntry, user_options: &UserOptions) -> Result<(), String> {
    //get fs items
    let mut filtered_dir_entries = get_items_list(entry, user_options)?;

    //sort fs items
    if user_options.should_sort_by_time() {
        sort_dir_entries_by_time(&mut filtered_dir_entries)?;
    } else {
        //todo sort alphabetically
    }

    //todo reverse sort order if needed

    //display fs items

    // debug
    for item in filtered_dir_entries {
        if let Some(item_name) = item.file_name() {
            if let Some(item_name) = item_name.to_str() {
                println!("{}", item_name)
            }
        } else {
            println!("{}", item.to_str().unwrap());
        }
    }
    Ok(())
}

fn sort_dir_entries_by_time(dir_entries: &mut Vec<PathBuf>) -> Result<(), String>{
    dir_entries.sort_by(|a, b| {
        match (a.metadata(), b.metadata()) {

            (Ok(a_data), Ok(b_data)) => {
                let a_modif_date = a_data.modified().unwrap_or(SystemTime::UNIX_EPOCH);
                let b_modif_date = b_data.modified().unwrap_or(SystemTime::UNIX_EPOCH);

                match b_modif_date.cmp(&a_modif_date) {
                    Ordering::Equal => { //sorts alphabetically if two entities have been modified at the same time
                        let (a_str, b_str) = (a.to_str().unwrap_or(""), b.to_str().unwrap_or(""));
                        a_str.cmp(b_str)
                    },
                    order => order
                }
            },
            _ => std::cmp::Ordering::Equal
        }
    });
    Ok(())
}

fn get_items_list(entry: UserItemEntry, user_options: &UserOptions) -> Result<Vec<PathBuf>, String> {
    let fs_items = match read_dir(&entry.path) {
        Ok(fs_items) => fs_items,
        Err(error) => return Err(format!("cannot access '{}': {}", entry.entry, error.to_string()))
    };
    let mut filtered_fs_items = Vec::new();

    for item in fs_items {
        let item = match item {
            Ok(item) => item,
            Err(error) => return Err(error.to_string())
        };

        if let Ok(item_name) = item.file_name().into_string() {
            if item_name.starts_with('.') {
                if user_options.should_list_hidden() {
                    filtered_fs_items.push(item.path());
                }
            } else {
                filtered_fs_items.push(item.path());
            }
        }
    }
    if user_options.should_list_hidden() {
        filtered_fs_items.push(Path::new(".").to_path_buf());
        filtered_fs_items.push(Path::new("..").to_path_buf());
    }
    Ok(filtered_fs_items)
}