use crate::cli_args_parser::parse_arguments;
use std::process::exit;
use crate::error_messages::get_error_message;
use crate::list_fs_items::{get_filtered_user_items, list_dir_content};

mod cli_args_parser;
mod error_messages;
mod list_fs_items;

fn main() {
    // parse CLI arguments
    let user_options = match parse_arguments() {
        Ok(user_options) => user_options,
        Err(error) => {
            eprintln!("{}", error);
            exit(1);
        }
    };

    //display non-containing fs items
    let (containing_items, _non_containing_items) = get_filtered_user_items(&user_options);
    //todo: Add display for non-containing fs items

    //display content of containing fs items
    for entry in containing_items {
        if let Err(error) = list_dir_content(entry, &user_options) {
            eprintln!("{}", get_error_message(error.to_string()));
        };
    }

    //exit with right code
}