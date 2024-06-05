use std::cmp::Ordering;
use std::ffi::OsStr;
use crate::cli_args_parser::UserOptions;
use crate::error_messages::get_error_message_without_help_indication;
use crate::display_fs_items::display_fs_items;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

pub struct UserItemEntry {
    path: PathBuf,
    entry: String,
}

impl UserItemEntry {
    pub fn new(entry: String, path: PathBuf) -> UserItemEntry {
        UserItemEntry { path, entry }
    }

    pub fn entry(&self) -> &String {
        &self.entry
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
    let mut filtered_dir_entries = get_items_list(&entry, user_options)?;

    //sort fs items
    if user_options.should_sort_by_time() {
        sort_dir_entries_by_time(&mut filtered_dir_entries)?;
    } else {
        sort_dir_entries_alphabetically(&mut filtered_dir_entries)?;
    }

    if user_options.should_sort_in_reverse_order() {
        filtered_dir_entries.reverse();
    }

    //display fs items
    display_fs_items(&filtered_dir_entries, user_options, &entry)?;
    Ok(())
}

fn sort_dir_entries_by_time(dir_entries: &mut Vec<PathBuf>) -> Result<(), String> {
    dir_entries.sort_by(|a, b| {
        match (a.metadata(), b.metadata()) {
            (Ok(a_data), Ok(b_data)) => {
                let a_modif_date = a_data.modified().unwrap_or(SystemTime::UNIX_EPOCH);
                let b_modif_date = b_data.modified().unwrap_or(SystemTime::UNIX_EPOCH);

                match b_modif_date.cmp(&a_modif_date) {
                    Ordering::Equal => {
                        compare_path_buf_alphabetically(a, b) //sorts alphabetically if two entities have been modified at the same time
                    }
                    order => order
                }
            }
            _ => Ordering::Equal
        }
    });
    Ok(())
}

fn sort_dir_entries_alphabetically(dir_entries: &mut Vec<PathBuf>) -> Result<(), String> {
    dir_entries.sort_by(|a, b| {
        compare_path_buf_alphabetically(a, b)
    });
    Ok(())
}

fn compare_path_buf_alphabetically(a: &PathBuf, b: &PathBuf) -> Ordering {
    let a_str = a.file_name().unwrap_or(OsStr::new("")).to_str().unwrap_or("").trim_start_matches('.');
    let b_str = b.file_name().unwrap_or(OsStr::new("")).to_str().unwrap_or("").trim_start_matches('.');
    a_str.cmp(b_str)
}

fn get_items_list(entry: &UserItemEntry, user_options: &UserOptions) -> Result<Vec<PathBuf>, String> {
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