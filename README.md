# jq-lite

A lightweight Rust tool for parsing JSON data and retrieving values based on user queries.

## Features

- Parse JSON data
- Query JSON structures
- Return values matching user queries

## Usage

```bash
jq-lite <query> <json-input>
```

## Requirements

- Rust 1.56 or later

## Building

```bash
cargo build --release
```

## Example

```bash
./jq-lite ".name" '{"name": "John", "age": 30}'
```

## License

MIT
