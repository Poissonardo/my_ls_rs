use std::env;
use crate::error_messages::get_error_message;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct UserOptions {
    list_hidden: bool,
    list_dirs: bool,
    sort_reverse_order: bool,
    sort_by_time: bool,
    long_listing: bool,
    recursive_exec: bool,
    display_as_separate_entries: bool,
    requested_items: Vec<String>,
}

impl UserOptions {
    pub fn requested_items(&self) -> &Vec<String> {
        &self.requested_items
    }

    pub fn should_list_hidden(&self) -> bool {
        self.list_hidden
    }

    pub fn should_list_dirs(&self) -> bool {
        self.list_dirs
    }

    /*pub fn should_sort_reverse_order(&self) -> bool {
        self.sort_reverse_order
    }

    pub fn should_sort_by_time(&self) -> bool {
        self.sort_by_time
    }

    pub fn should_do_long_listing(&self) -> bool {
        self.long_listing
    }

    pub fn should_do_recursive_exec(&self) -> bool {
        self.recursive_exec
    }

    pub fn should_display_as_separate_entries(&self) -> bool {
        self.display_as_separate_entries
    }*/

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
        display_as_separate_entries: false,
        requested_items: vec![".".to_string()],
    };

    for arg in cli_args {
        if arg.starts_with('-') && arg.len() > 1 {
            parse_flags(arg, &mut user_options)?;
        } else {
            user_options.add_req_item(arg);
        }
    }

    // if user requests listing of multiple fs entries
    if user_options.requested_items.len() > 1 {
        user_options.display_as_separate_entries = true;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_flags_test_all() {
        let arg = "-adrtlR".to_string();
        let mut user_options = UserOptions {
            list_hidden: false,
            list_dirs: false,
            sort_reverse_order: false,
            sort_by_time: false,
            long_listing: false,
            recursive_exec: false,
            display_as_separate_entries: false,
            requested_items: vec![".".to_string()],
        };

        let expected_user_options = UserOptions {
            list_hidden: true,
            list_dirs: true,
            sort_reverse_order: true,
            sort_by_time: true,
            long_listing: true,
            recursive_exec: true,
            display_as_separate_entries: false,
            requested_items: vec![".".to_string()],
        };

        parse_flags(arg, &mut user_options).unwrap();
        assert_eq!(user_options, expected_user_options);
    }

    #[test]
    fn parse_flags_test_empty() {
        let arg = "-".to_string();
        let mut user_options = UserOptions {
            list_hidden: false,
            list_dirs: false,
            sort_reverse_order: false,
            sort_by_time: false,
            long_listing: false,
            recursive_exec: false,
            display_as_separate_entries: false,
            requested_items: vec![".".to_string()],
        };

        let expected_user_options = UserOptions {
            list_hidden: false,
            list_dirs: false,
            sort_reverse_order: false,
            sort_by_time: false,
            long_listing: false,
            recursive_exec: false,
            display_as_separate_entries: false,
            requested_items: vec![".".to_string()],
        };

        parse_flags(arg, &mut user_options).unwrap();
        assert_eq!(user_options, expected_user_options);
    }

    #[test]
    fn parse_flags_test_unknown_flag() {
        let arg = "-V".to_string();
        let mut user_options = UserOptions {
            list_hidden: false,
            list_dirs: false,
            sort_reverse_order: false,
            sort_by_time: false,
            long_listing: false,
            recursive_exec: false,
            display_as_separate_entries: false,
            requested_items: vec![".".to_string()],
        };

        assert!(parse_flags(arg, &mut user_options).is_err());
    }
}