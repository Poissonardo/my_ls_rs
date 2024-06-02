use std::env;
use crate::error_messages::get_error_message;

#[derive(Debug)]
pub struct UserOptions {
    list_hidden: bool,
    list_dirs: bool,
    sort_reverse_order: bool,
    sort_by_time: bool,
    long_listing: bool,
    recursive_exec: bool,
    requested_items: Vec<String>,
}

impl UserOptions {
    fn add_req_item(&mut self, fs_item: String) {
        if self.requested_items.starts_with(&[".".to_string()]) && self.requested_items.len() == 1 {
            self.requested_items.clear();
        }
        self.requested_items.push(fs_item);
    }
}

pub fn parse_arguments() -> Result<UserOptions, String> {
    let cli_args: Vec<String> = env::args().skip(1).collect();
    let mut user_options = UserOptions {
        list_hidden: false,
        list_dirs: false,
        sort_reverse_order: false,
        sort_by_time: false,
        long_listing: false,
        recursive_exec: false,
        requested_items: vec![".".to_string()],
    };

    for arg in cli_args {
        if arg.starts_with('-') && arg.len() > 1 {
            parse_flags(arg, &mut user_options)?;
        } else {
            user_options.add_req_item(arg);
        }
    }
    Ok(user_options)
}

fn parse_flags(arg: String, user_options: &mut UserOptions) -> Result<(), String> {
    for character in arg.chars().skip(1) { //skips the '-' character
        match character {
            'a' => user_options.list_hidden = true,
            'd' => user_options.list_dirs = true,
            'r' => user_options.sort_reverse_order = true,
            't' => user_options.sort_by_time = true,
            'l' => user_options.long_listing = true,
            'R' => user_options.recursive_exec = true,
            _ => return Err(get_error_message(format!("invalid option -- '{}'", character)))
        }
    }
    Ok(())
}