use std::fs::{DirEntry, read_dir};
use std::io;
use crate::cli_args_parser::UserOptions;

fn get_items_list(entry: &String, user_options: &UserOptions) -> io::Result<Vec<DirEntry>> {
    let fs_items = read_dir(entry)?;
    let mut filtered_fs_items = Vec::new();

    for item in fs_items {
        let item = item?;

        if let Ok(item_name) = item.file_name().into_string() {
            if item_name.starts_with('.') {
                if user_options.should_list_hidden() {
                    filtered_fs_items.push(item);
                }
            } else {
                filtered_fs_items.push(item);
            }
        }
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
        if let Ok(item_name) = item.file_name().into_string() {
            println!("{}", item_name)
        }
    }
    Ok(())
}