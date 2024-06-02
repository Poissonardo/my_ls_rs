use std::fs::read_dir;
use std::io;
use std::path::{Path, PathBuf};
use crate::cli_args_parser::UserOptions;

fn get_items_list(entry: &String, user_options: &UserOptions) -> io::Result<Vec<PathBuf>> {
    let fs_items = read_dir(entry)?;
    let mut filtered_fs_items = Vec::new();

    for item in fs_items {
        let item = item?;

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

pub fn list_dir_content(entry: &String, user_options: &UserOptions) -> io::Result<()> {
    //get fs items
    let filtered_dir_entries = get_items_list(entry, user_options)?;

    //sort fs items

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