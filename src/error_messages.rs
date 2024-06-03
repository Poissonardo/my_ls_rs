use std::env;
use std::process::exit;

pub fn get_error_message(error: String) -> String {
    let cli_args = match env::args().next() {
        Some(prog_name) => prog_name,
        None => exit(1)
    };

    format!("{}: {}\nTry 'ls --help' for more information.", cli_args, error)
}

pub fn get_error_message_without_help_indication(error: String) -> String {
    let cli_args = match env::args().next() {
        Some(prog_name) => prog_name,
        None => exit(1)
    };

    format!("{}: {}", cli_args, error)
}