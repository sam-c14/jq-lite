use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io::{self, IsTerminal, Read};

pub const USER_QUERY: usize = 1;

pub const FILE_PATH: usize = 2;

const ONLY_DIRECTORY_PATH: usize = 1;

const HAS_FILE_PATH_AND_QUERY: usize = 3;

#[derive(Debug)]
enum Token {
    Field(String),
    Wildcard,     // []
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
    let v: Value = serde_json::from_str(&file)?;

    let tokens = tokenize(user_query);
    let results = execute_query(&tokens, &v);

    println!("Results for query '{}':\n", user_query);

    for result in results {
        println!("{}", result);
    }

    Ok(())
}

fn parse_arrays(query: &str) -> String {
    let parsed: String = query.replace("[", "/").replace("]", "");

    parsed
}

fn tokenize(query: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut buffer = String::new();
    let mut chars = query.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '.' => {
                // End of a field
                if !buffer.is_empty() {
                    tokens.push(Token::Field(buffer.clone()));
                    buffer.clear();
                }
            }

            '[' => {
                // Flush any field before [
                if !buffer.is_empty() {
                    tokens.push(Token::Field(buffer.clone()));
                    buffer.clear();
                }

                // Look ahead
                if let Some(']') = chars.peek() {
                    // This is []
                    chars.next(); // consume ]
                    tokens.push(Token::Wildcard);
                } else {
                    // This is [index]
                    let mut number = String::new();

                    while let Some(c) = chars.next() {
                        if c == ']' {
                            break;
                        }
                        number.push(c);
                    }

                    let index = number.parse::<usize>().expect("Invalid array index");

                    tokens.push(Token::Index(index));
                }
            }

            _ => {
                buffer.push(ch);
            }
        }
    }

    // Flush remaining buffer
    if !buffer.is_empty() {
        tokens.push(Token::Field(buffer));
    }

    tokens
}

fn execute_query<'a>(tokens: &'a Vec<Token>, root: &'a Value) -> Vec<&'a Value> {
    let mut current: Vec<&Value> = vec![root];

    for token in tokens {
        let mut next = Vec::new();

        for value in current {
            match token {
                Token::Field(key) => {
                    if let Some(v) = value.get(key) {
                        next.push(v);
                    }
                }
                Token::Wildcard => {
                    if let Some(arr) = value.as_array() {
                        for item in arr {
                            next.push(item);
                        }
                    }
                }
                Token::Index(i) => {
                    if let Some(arr) = value.as_array() {
                        if let Some(v) = arr.get(*i) {
                            next.push(v);
                        }
                    }
                }
            }
        }

        current = next;
    }

    current
}
