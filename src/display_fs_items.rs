use std::path::PathBuf;
use crate::cli_args_parser::UserOptions;
use crate::list_fs_items::UserItemEntry;
use colored::Colorize;

pub fn display_fs_items(dir_entries: &Vec<PathBuf>, user_options: &UserOptions, entry: &UserItemEntry) -> Result<(), String> {
    if user_options.should_display_as_separate_entries() { //displays listed dir name
        println!("{}:", entry.entry());
    }

    if user_options.should_display_long_listing() {
        todo!()
    } else {
        display_fs_item_vec_normal(dir_entries)?;
    }
    Ok(())
}

fn display_fs_item_vec_normal(dir_entries: &Vec<PathBuf>) -> Result<(), String> {
    for (i, item) in dir_entries.iter().enumerate() {
        display_fs_item_normal(item)?;
        if i != dir_entries.len() - 1 {
            print!("  ");
        }
    }
    println!();
    Ok(())
}

pub fn display_fs_item_normal(fs_item: &PathBuf) -> Result<(), String>{

    let output_string = match fs_item.file_name() {
        Some(result) => result.to_str().unwrap_or(""),
        None => return Err("cannot display some items, error while pulling item name.".to_string())
    };

    if fs_item.is_dir() {
        print!("{}/", output_string.blue().bold());
    } else {
        print!("{}", output_string)
    }
    Ok(())
}