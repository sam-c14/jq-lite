pub mod parse_utils;

use parse_utils::{get_file, get_json_data, read_stdin};
use serde_json::Result as SerdeJsonResult;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let user_query: &str = &args[1];

    if args.len() < 3 {
        let user_input = read_stdin().expect("Unexpected error reading user input");
        
        parse_json_file("", user_query, &user_input).expect("Failed to parse JSON data");
    } else {
        let file_path: &str = &args[2];

        parse_json_file(file_path, user_query, "").expect("Failed to parse JSON data");
    }
}

fn parse_json_file(
    file_path: &str,
    user_query: &str,
    processed_file: &str,
) -> SerdeJsonResult<()> {
    let mut file = String::new();

    if processed_file.len() > 0 {
        file.push_str(processed_file);
    } else {
        match get_file(file_path) {
            Ok(json) => {
                file = json;
                println!("File found")
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
