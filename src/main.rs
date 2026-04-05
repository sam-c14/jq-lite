pub mod parse_utils;

use parse_utils::{is_missing_query, has_only_json_query, is_missing_file_path, FILE_PATH, USER_QUERY, get_file, get_json_data, read_stdin};
use serde_json::Result as SerdeJsonResult;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if is_missing_query(&args) {
        eprintln!("Error:User query is required");
        print_usage();
        return;
    }

    let user_query: &str = &args[USER_QUERY];

    if has_only_json_query(&args) {
        let user_input = read_stdin().expect("Unexpected error reading user input");

        parse_json_file("", user_query, &user_input).expect("Failed to parse JSON data");
    } else {
        if is_missing_file_path(&args) {
            eprintln!("Error: Missing query argument");
            print_usage();
            std::process::exit(1);
        }

        let file_path: &str = &args[FILE_PATH];

        parse_json_file(file_path, user_query, "").expect("Failed to parse JSON data");
    }
}

fn parse_json_file(file_path: &str, user_query: &str, processed_file: &str) -> SerdeJsonResult<()> {
    let mut file = String::new();

    if processed_file.len() > 0 {
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

fn print_usage() {
    println!("Usage:");
    println!("  jq-lite <query> <file-path>");
    println!("  cat file.json | jq-lite <query>");
    println!("  curl https://example.com/data.json | jq-lite <query>");
    println!();
    println!("Examples:");
    println!("  jq-lite \"name\" data.json");
    println!("  jq-lite \"user.name\" data.json");
}