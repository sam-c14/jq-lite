use serde_json::{Value, json};
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
    let mut v: Value = json!("{}");

    match serde_json::from_str(&file) {
        Ok(json_file) => {
            v = json_file;
        }
        Err(e) => {
            eprintln!("Invalid JSON error: {}", e);
            std::process::exit(2);
        }
    }

    let tokens = tokenize(user_query).unwrap_or_else(|e| {
        eprintln!("{}", e);
        std::process::exit(2);
    });

    validate_tokens(&tokens).unwrap_or_else(|error| {
        let error_message = format_error_message("Validation", &error.to_string());
        eprintln!("{}", error_message);
        std::process::exit(2);
    });

    let results = execute_query(&tokens, &v);

    if results.is_empty() {
        eprintln!("No results found for the query '{}'", user_query);
        std::process::exit(1);
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

fn tokenize(query: &str) -> Result<Vec<Token>, Box<dyn Error>> {
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
                        let error_message =
                            format_error_message("Tokenization", "Unclosed '[' in query");

                        return Err(error_message.into());
                    }

                    let index_mismatch_error = format!("Invalid array index: '{}'", number);

                    let index = number
                        .parse::<usize>()
                        .map_err(|_| format_error_message("Tokenization", &index_mismatch_error));

                    tokens.push(Token::Index(index?));
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

fn validate_tokens(tokens: &[Token]) -> Result<(), Box<dyn Error>> {
    if tokens.is_empty() {
        return Err("Empty query".into());
    }

    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::Field(name) => {
                if name.is_empty() {
                    return Err("Empty field name".into());
                }
            }

            Token::Index(_) | Token::Wildcard => {
                if i == 0 {
                    return Err("Query cannot start with array access".into());
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parse_utils::{tokenize, validate_tokens, get_file};

    const VALID_TOKEN: &str = "user.name";

    const INVALID_TOKEN: &str = "user.name[";

    const VALID_TOKEN_TTV: &str = "user.address[]";
    
    const INVALID_TOKEN_TTV: &str = "[].address[]";

    const VALID_FILE_PATH: &str = "src/data.json";

    const INVALID_FILE_PATH: &str = "data.json";

    #[test]
    fn test_tokenize() {
        // Test Valid Token
        let result = tokenize(VALID_TOKEN).unwrap();

        assert_eq!(result.len() > 0, true);

        // Test Invalid Token

        match tokenize(INVALID_TOKEN) {
            Ok(_val) => {}
            Err(e) => {
                assert_ne!(e.to_string(), "");
            }
        }
    }

    #[test]
    fn test_token_validation() {
        let valid_tokens = tokenize(VALID_TOKEN_TTV).unwrap();

        // Test Valid Token Sequence

        match validate_tokens(&valid_tokens) {
            Ok(_val) => {}
            Err(e) => {
                assert_eq!(e.to_string(), "");
            }
        }

        let invalid_tokens = tokenize(INVALID_TOKEN_TTV).unwrap();

        // Test Invalid Token Sequence

         match validate_tokens(&invalid_tokens) {
            Ok(_val) => {}
            Err(e) => {
                assert_ne!(e.to_string(), "");
            }
        }
    }

    #[test]
    fn test_get_file_util() {

        match get_file(VALID_FILE_PATH) {
            Ok(_json) => {
               
            },
            Err(e) => {
                 assert_eq!(e.to_string(), "");
            }
        } 
        
        match get_file(INVALID_FILE_PATH) {
            Ok(_json) => {},
            Err(e) => {
                assert_ne!(e.to_string(), "");
            }
        }
    }
}
