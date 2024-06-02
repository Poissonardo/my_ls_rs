use crate::cli_args_parser::parse_arguments;
use std::process::exit;

mod cli_args_parser;
mod error_messages;

fn main() {
    // parse CLI arguments
    let user_options = match parse_arguments() {
        Ok(user_options) => user_options,
        Err(error) => {
            eprintln!("{}", error);
            exit(1);
        }
    };
    println!("DEBUG: {:#?}", user_options);

    // list FS items for each

    //exit with right code
}