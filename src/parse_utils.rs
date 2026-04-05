use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io::{self, IsTerminal, Read};

pub const USER_QUERY: usize = 1;

pub const FILE_PATH: usize = 2;

const ONLY_DIRECTORY_PATH: usize = 1;

const HAS_FILE_PATH_AND_QUERY: usize = 3;

pub fn has_only_json_query(args: &Vec<String>) -> bool {
    args.len() == (USER_QUERY + ONLY_DIRECTORY_PATH) && !io::stdin().is_terminal()
}

pub fn is_missing_query(args: &Vec<String>) -> bool {
    args.len() == ONLY_DIRECTORY_PATH
}

pub fn is_missing_file_path(args: &Vec<String>) -> bool {
    args.len() < HAS_FILE_PATH_AND_QUERY
}

pub fn get_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let json: String = fs::read_to_string(file_path)?;
    Ok(json)
}

pub fn parse_nested_json(query: &str) -> String {
    let queries: Vec<&str> = query.split(".").collect();

    let joint_query: String = String::from("/") + &queries.join("/");

    joint_query
}

pub fn read_stdin() -> Result<String, io::Error> {
    let mut input: String = String::new();

    io::stdin().read_to_string(&mut input)?;

    Ok(input)
}

pub fn get_json_data(user_query: &str, file: String) -> Result<(), Box<dyn Error>> {
    let query_array: Vec<&str> = user_query.split_whitespace().collect();

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&file)?;

    for query in query_array {
        if query.contains(".") {
            let nested_query = parse_nested_json(query);
            let result = v.pointer(&nested_query);
            match result {
                Some(value) => println!("Result for query '{}': {}", query, value),
                None => println!("No result found for query '{}'", query),
            }
        } else {
            let result = v.get(query);
            match result {
                Some(value) => println!("Result for query '{}': {}", query, value),
                None => println!("No result found for query '{}'", query),
            }
        }
    }

    Ok(())
}
