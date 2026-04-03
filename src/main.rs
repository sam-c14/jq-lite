use std::env;
use std::fs;
use serde_json::{Result as SerdeJsonResult, Value};
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let user_query: &String = &args[1];

    let file_path: &String = &args[2];

    parse_json_file(file_path, user_query).expect("Failed to parse JSON data");

    println!("Rust Application Started");
}

fn get_file(file_path: &String) -> Result<String, Box<dyn Error>> {
    let json: String = fs::read_to_string(file_path)?;
    Ok(json)
}

fn parse_json_file(file_path: &String, user_query: &String) -> SerdeJsonResult<()> {
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
        if v[query].is_null() {
            println!("Query '{}' not found in JSON data.", query);   
        } else {
            println!("Query '{}' gave {}", query, v[query]);
        }
    }

    Ok(())
}