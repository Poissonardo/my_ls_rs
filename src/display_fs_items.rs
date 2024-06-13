use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use nix::unistd::{Uid, User};
use crate::cli_args_parser::UserOptions;
use crate::list_fs_items::UserItemEntry;
use chrono::{Datelike, DateTime, Local, Timelike};
use colored::Colorize;

const S_IRUSR: u32 = 0o400;
const S_IWUSR: u32 = 0o200;
const S_IXUSR: u32 = 0o100;

const S_IRGRP: u32 = 0o040;
const S_IWGRP: u32 = 0o020;
const S_IXGRP: u32 = 0o010;

const S_IROTH: u32 = 0o004;
const S_IWOTH: u32 = 0o002;
const S_IXOTH: u32 = 0o001;
const MONTHS: [&str; 11] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Sep", "Oct", "Nov", "Dec"];

pub fn display_fs_items(dir_entries: &Vec<PathBuf>, user_options: &UserOptions, entry: &UserItemEntry) -> Result<(), String> {
    if user_options.should_display_as_separate_entries() { //displays listed dir name
        println!("{}:", entry.entry());
    }

    if user_options.should_display_long_listing() {
        display_fs_item_vec_long_listing(dir_entries)?;
    } else {
        display_fs_item_vec_normal(dir_entries)?;
    }
    Ok(())
}

fn display_fs_item_long_listing(item: &PathBuf) -> Result<(), String> {
    let metadata = match item.metadata() {
        Ok(metadata) => metadata,
        Err(error) => return Err(error.to_string())
    };
    let item_type = {
        if item.is_dir() {
            'd'
        } else if item.is_file() {
            '-'
        } else if item.is_symlink() {
            'l'
        } else {
            '-'
        }
    };
    let item_permissions = {
        let mode = metadata.permissions().mode();
        let mut perms_string = String::new();

        if (mode & S_IRUSR) != 0 {
            perms_string.push('r');
        } else {
            perms_string.push('-');
        }

        if (mode & S_IWUSR) != 0 {
            perms_string.push('w');
        } else {
            perms_string.push('-');
        }

        if (mode & S_IXUSR) != 0 {
            perms_string.push('x');
        } else {
            perms_string.push('-');
        }

        if (mode & S_IRGRP) != 0 {
            perms_string.push('r');
        } else {
            perms_string.push('-');
        }

        if (mode & S_IWGRP) != 0 {
            perms_string.push('w');
        } else {
            perms_string.push('-');
        }

        if (mode & S_IXGRP) != 0 {
            perms_string.push('x');
        } else {
            perms_string.push('-');
        }

        if (mode & S_IROTH) != 0 {
            perms_string.push('r');
        } else {
            perms_string.push('-');
        }

        if (mode & S_IWOTH) != 0 {
            perms_string.push('w');
        } else {
            perms_string.push('-');
        }

        if (mode & S_IXOTH) != 0 {
            perms_string.push('x');
        } else {
            perms_string.push('-');
        }

        perms_string
    };
    let nb_links = metadata.st_nlink();
    let uid = Uid::from_raw(metadata.st_uid());
    let user = match User::from_uid(uid) {
        Ok(Some(user)) => user.name,
        Ok(None) => String::new(),
        Err(err) => return Err(err.to_string()),
    };
    let gid = Uid::from_raw(metadata.st_gid());
    let group = match User::from_uid(gid) {
        Ok(Some(group)) => group.name,
        Ok(None) => String::new(),
        Err(err) => return Err(err.to_string()),
    };
    let size = metadata.st_size();

    let sysdate = match metadata.modified() {
        Ok(date) => date,
        Err(error) => return Err(error.to_string())
    };
    let date : DateTime<Local> = sysdate.into();

    print!("{}{} {} {} {} {} {} {} {:02}:{:02} ", item_type, item_permissions, nb_links, user, group, size, MONTHS[(date.month() - 1) as usize], date.day(), date.hour(), date.minute());
    display_fs_item_normal(item)?;
    println!();
    Ok(())
}


fn display_fs_item_vec_long_listing(dir_entries: &Vec<PathBuf>) -> Result<(), String> {
    println!("total {}", get_nb_blocks(dir_entries)?);
    for item in dir_entries {
        display_fs_item_long_listing(item)?;
    }
    Ok(())
}

fn get_nb_blocks(dir_entries: &Vec<PathBuf>) -> Result<u64, String> {
    let mut nb_blocks = 0;

    for entry in dir_entries {
        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(error) => return Err(error.to_string())
        };
        nb_blocks += metadata.st_blocks();
    }
    Ok(nb_blocks / 2)
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

pub fn display_fs_item_normal(fs_item: &PathBuf) -> Result<(), String> {

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