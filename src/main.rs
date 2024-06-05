use crate::cli_args_parser::parse_arguments;
use crate::error_messages::get_error_message;
use crate::list_fs_items::{get_filtered_user_items, list_dir_content};
use std::process::exit;
use crate::display_fs_items::display_fs_item_normal;

mod cli_args_parser;
mod error_messages;
mod list_fs_items;
mod display_fs_items;

fn main() {
    // parse CLI arguments
    let user_options = match parse_arguments() {
        Ok(user_options) => user_options,
        Err(error) => {
            eprintln!("{}", error);
            exit(1);
        }
    };

    let (containing_items, non_containing_items) = get_filtered_user_items(&user_options);

    //display non-containing fs items
    for (i, item) in non_containing_items.iter().enumerate() {
        let item = item.path();
        if let Err(error) = display_fs_item_normal(item) {
            eprintln!("{}", get_error_message(error.to_string()));
            exit(1);
        }
        if i != non_containing_items.len() - 1 {
            print!("  ");
        }
    }
    println!();
    if !containing_items.is_empty() {
        println!()
    }

    //display content of containing fs items
    for (i, entry) in containing_items.iter().enumerate() {
        if let Err(error) = list_dir_content(entry, &user_options) {
            eprintln!("{}", get_error_message(error.to_string()));
        };
        if i != containing_items.len() - 1 { //adds a newline between output of each asked entry for better readability
            println!()
        }
    }

    //exit with right code
}