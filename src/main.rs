use std::env;
use std::fs;
use serde_json::{Result as SerdeJsonResult, Value};
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let user_query: &str = &args[1];

    let file_path: &str = &args[2];

    parse_json_file(file_path, user_query).expect("Failed to parse JSON data");

    println!("Rust Application Started");
}

fn get_file(file_path: &str) -> Result<String, Box<dyn Error>> {
    let json: String = fs::read_to_string(file_path)?;
    Ok(json)
}

fn parse_json_file(file_path: &str, user_query: &str) -> SerdeJsonResult<()> {
    let mut file = String::new();

     match get_file(file_path) {
        Ok(json) => {
            file = json;
            println!("File found")
        },
        Err(e) => {
            println!("Error reading file: {}", e);
            return Ok(());
        }
    }

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

fn parse_nested_json(query: &str) -> String {
    let queries: Vec<&str> = query.split(".").collect();

    let joint_query: String = String::from("/") + &queries.join("/");

    joint_query
}