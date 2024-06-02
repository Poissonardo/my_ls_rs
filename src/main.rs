use crate::cli_args_parser::parse_arguments;
use std::process::exit;
use crate::error_messages::get_error_message;
use crate::list_fs_items::list_dir_content;

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
    //println!("DEBUG: {:#?}", &user_options);

    // list FS items for each
    for entry in user_options.requested_items() {
        match list_dir_content(entry, &user_options) {
            Ok(()) => {},
            Err(error) => {
                eprintln!("{}", get_error_message(error.to_string()));
                exit(2);
            }
        };
    }

    //exit with right code
}