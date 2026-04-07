use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io::{self, IsTerminal, Read};

pub const USER_QUERY: usize = 1;

pub const FILE_PATH: usize = 2;

const ONLY_DIRECTORY_PATH: usize = 1;

const HAS_FILE_PATH_AND_QUERY: usize = 3;

enum Token {
    Field(String),
    Wildcard,   // []
    Index(usize), // [1]
}

pub fn has_only_json_query(args: &[String]) -> bool {
    args.len() == (USER_QUERY + ONLY_DIRECTORY_PATH) && !io::stdin().is_terminal()
}

pub fn is_missing_query(args: &[String]) -> bool {
    args.len() == ONLY_DIRECTORY_PATH
}

pub fn is_missing_file_path(args: &[String]) -> bool {
    args.len() < HAS_FILE_PATH_AND_QUERY
}

pub fn get_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let json: String = fs::read_to_string(file_path)?;
    Ok(json)
}

pub fn parse_nested_json(query: &str) -> String {
    let mut joint_query: String = parse_arrays(query);

    if joint_query.contains(".") {
        joint_query = joint_query.replace(".", "/");
    }

    if joint_query.starts_with("/") {
        joint_query
    } else {
        String::from("/") + &joint_query
    }
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

    let has_wildcards: bool = user_query.contains("[]");

    if has_wildcards {
        parse_query_with_wildcards(user_query, &v);
    } else {
        parse_query_without_wildcards(&v, query_array);
    }

    Ok(())
}

fn parse_arrays(query: &str) -> String {
    let parsed: String = query.replace("[", "/").replace("]", "");

    parsed
}

fn parse_query_without_wildcards(file_string: &Value, query_array: Vec<&str>) {
    for query in query_array {
        // Hanlde nested queries using dot notation
        if query.contains(".") {
            let nested_query = parse_nested_json(query);

            let result = file_string.pointer("/user/address/city");

            print_results(&result, nested_query.as_str());
        } else if query.contains("]") && query.contains("[") {
            let array_query = String::from("/") + &parse_arrays(query);

            let result = file_string.pointer(array_query.as_str());

            print_results(&result, query);
        } else {
            let result = file_string.get(query);

            print_results(&result, query);
        }
    }
}

fn parse_query_with_wildcards(query: &str, file_string: &Value) {
    // Check if empty brackets exists between the string
    // ["user.address", ".city[1].block", ""]

    let first_wildcard_index = query.find("]");

    let has_single_wildcard_at_end = first_wildcard_index.unwrap() + 1 >= query.len();

    if has_single_wildcard_at_end {
        let joint_query = parse_nested_json(&query.replace("[", "]"));

        let result = file_string.pointer(&joint_query).unwrap();

        print_results(&Some(result), query);
    } else {
        let wildcard_queries: Vec<&str> = query.split("[]").collect();

        print!("Wildcard queries: {:?}\n", wildcard_queries);

        let mut result: &Value = file_string;

        for wildcard_query in wildcard_queries {
            let joint_query = parse_nested_json(wildcard_query);

            if wildcard_query.trim().is_empty() {
                continue;
            }

            result = result.pointer(&joint_query).expect(format!("Parsing Failed {joint_query} {result}").as_str());
        }

        print_results(&Some(result), query);
    }
}

fn print_results(result: &Option<&Value>, query: &str) {
    match result {
        Some(value) => println!("Result for query '{}': {}", query, value),
        None => println!("No result found for query '{}'", query),
    }
}
