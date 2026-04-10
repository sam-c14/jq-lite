pub const HELP_COMMAND_LONG: &str = "--help";

pub const HELP_COMMAND_SHORT: &str = "-h";

pub fn print_help() {
    println!("jq-lite - Lightweight JSON query tool\n");
    println!("Usage:");
    println!("  jq-lite <query> <file-path>");
    println!("  cat file.json | jq-lite <query>");
    println!("  curl https://example.com/data.json | jq-lite <query>");
    println!(
        "  echo '{{\"name\": \"Alice\", \"user\": {{\"email\": \"alice@example.com\"}}}}'  | jq-lite <query>"
    );
    println!();
    println!("Options:");
    println!("  -h, --help    Show this help information");
    println!();
    println!("Examples:");
    println!("  jq-lite \"name\" data.json");
    println!("  jq-lite \"user.name\" data.json");
    println!("  jq-lite \"name age user.email\" data.json");
    println!("  cat data.json | jq-lite \"name\"");
    println!("  cat data.json | jq-lite \"users.address[]\"");
    println!("  cat data.json | jq-lite \"users.address[].name\"");
    println!();
    println!("Notes:");
    println!("  - Supports top-level and nested queries using dot notation.");
    println!("  - Reads JSON from a file or from stdin when piped.");
    println!("  - Does not currently support array indexing or advanced filters.");
}

pub fn print_error(message: &str) {
    eprintln!("Error: {}", message);
    println!("Use command -h, --help to show help information");
}
