use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io::{self, IsTerminal, Read};

pub const USER_QUERY: usize = 1;

pub const FILE_PATH: usize = 2;

const PRETTY_FLAG: &str = "--pretty";

const RAW_FLAG: &str = "--raw";

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

pub fn read_stdin() -> Result<String, io::Error> {
    let mut input: String = String::new();

    io::stdin().read_to_string(&mut input)?;

    Ok(input)
}

pub fn get_json_data(user_query: &str, file: String, flag: &str) -> Result<(), Box<dyn Error>> {
    let v: Value = serde_json::from_str(&file)?;

    println!("Flag {}\n", flag);

    let tokens = tokenize(user_query).map_err(|e| format!("Tokenization error: {}", e))?;

    validate_tokens(&tokens).unwrap();

    let results = execute_query(&tokens, &v);

    if results.is_empty() {
        println!("No results found for the query '{}'", user_query);
    } else {
        println!("Results for the query '{}':\n", user_query);

        for result in results {
            if flag == PRETTY_FLAG {
                println!("{}\n", serde_json::to_string_pretty(result)?);
            } else if flag == RAW_FLAG {
                match result {
                    Value::String(s) => println!("{}\n", s),
                    _ => println!("{}\n", result),
                }
            } else {
                println!("{}\n", result);
            }
        }
    }

    Ok(())
}

fn format_error_message(error_type: &str, message: &str) -> String {
    format!("{error_type} Error: {message}")
}

fn tokenize(query: &str) -> Result<Vec<Token>, String> {
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

                    let mut found_closing_bracket = false;

                    while let Some(c) = chars.next() {
                        if c == ']' {
                            found_closing_bracket = true;
                            break;
                        }
                        number.push(c);
                    }

                    if !found_closing_bracket {
                        eprintln!(
                            "{}",
                            format_error_message("Tokenization", "Unclosed '[' in query")
                        );
                        std::process::exit(2)
                    }

                    let index_mismatch_error = format!("Invalid array index: '{}'", number);

                    let index = number.parse::<usize>().unwrap_or_else(|_| {
                        eprintln!(
                            "{}",
                            format_error_message("Tokenization", &index_mismatch_error)
                        );
                        std::process::exit(2)
                    });

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

    Ok(tokens)
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

fn validate_tokens(tokens: &[Token]) -> Result<(), String> {
    if tokens.is_empty() {
        eprintln!("{}", format_error_message("Validation", "Empty query"));
        std::process::exit(2);
    }

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::Field(name) => {
                if name.is_empty() {
                    eprintln!("{}", format_error_message("Validation", "Empty field name"));
                    std::process::exit(2);
                }
            }

            Token::Index(_) | Token::Wildcard => {
                if i == 0 {
                    eprintln!(
                        "{}",
                        format_error_message("Validation", "Query cannot start with array access")
                    );
                    std::process::exit(2);
                }
            }
        }
    }

    Ok(())
}
