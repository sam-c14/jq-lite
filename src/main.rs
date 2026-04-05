pub mod help_utils;
pub mod parse_utils;

use help_utils::{print_help, print_error, HELP_COMMAND_LONG, HELP_COMMAND_SHORT};
use parse_utils::{
    FILE_PATH, USER_QUERY, get_file, get_json_data, has_only_json_query, is_missing_file_path,
    is_missing_query, read_stdin,
};
use serde_json::Result as SerdeJsonResult;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let h1= String::from(HELP_COMMAND_LONG);
    let h2 = String::from(HELP_COMMAND_SHORT);

    if args.len() > 1 && (args.contains(&h1) || args.contains(&h2)) {
        print_help();
        std::process::exit(0);
    }

    if is_missing_query(&args) {
        print_error("User query is required");
        std::process::exit(1);
    }

    let user_query: &str = &args[USER_QUERY];

    if has_only_json_query(&args) {
        let user_input = read_stdin().expect("Unexpected error reading user input");

        parse_json_file("", user_query, &user_input).expect("Failed to parse JSON data");
    } else {
        if is_missing_file_path(&args) {
            print_error("Missing query argument");
            std::process::exit(1);
        }

        let file_path: &str = &args[FILE_PATH];

        parse_json_file(file_path, user_query, "").expect("Failed to parse JSON data");
    }
}

fn parse_json_file(file_path: &str, user_query: &str, processed_file: &str) -> SerdeJsonResult<()> {
    let mut file = String::new();

    if !processed_file.is_empty() {
        file.push_str(processed_file);
    } else {
        match get_file(file_path) {
            Ok(json) => {
                file = json;
                println!("File retrived")
            }
            Err(e) => {
                println!("Error reading file: {}", e);
                return Ok(());
            }
        }
    }

    get_json_data(user_query, file).expect("Failed to get JSON data");

    Ok(())
}
