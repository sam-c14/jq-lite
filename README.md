# 📦 jq-lite

A lightweight CLI tool built in Rust for querying JSON data using simple dot notation.

---

## 🚀 Current Features

- ✅ Read JSON from a file
- ✅ Query top-level fields
- ✅ Query nested JSON using dot notation
- ✅ Multiple queries in a single command (space-separated)
- ✅ Uses `serde_json` for parsing

---

## 📖 Usage

```bash
jq-lite <query> <file-path>
```

## 🧪 Examples

- Basic query
```bash
jq-lite "name" data.json
```

- Nested query
```bash
jq-lite "user.name" data.json
```

- Multiple queries
```bash
jq-lite "name age user.email" data.json
```

## 🛠️ How It Works

* Queries are split by whitespace
* Nested queries (user.name) are converted into JSON Pointer format:
```bash
user.name → /user/name
```

### Uses
- `Value::get()` for top-level fields
- `Value::pointer()` for nested access

## ⚠️ Current Limitations
- ❌ No array indexing (users[0])
- ❌ No stdin support
- ❌ No pretty printing
- ❌ Basic error handling
- ❌ No query validation
- ❌ No support for pipes or filters
- ❌ Not streaming (loads full JSON into memory)

## Recent Addons

1. **Add stdin support for JSON input**

    Allow piping JSON into jq-lite:
    ```bash
    cat data.json | jq-lite "name"
    ```

2. **Support raw JSON string as input**

    Allow users to pass JSON directly instead of file path:
    ```bash
    jq-lite "name" '{"name":"John"}'
    ```
3. **Improve CLI argument validation**

    - Handle missing arguments
    - Show usage instructions
    - Prevent panics on missing inputs
    
4. **Add `--help `flag**

    Display usage, examples, and available options

## 🧭 Roadmap (GitHub Issues)

## 🟡 Query Engine Improvements

5. **Support array indexing in queries**
    ```bash
    jq-lite "users[0].name" data.json
    ```

6. **Support wildcard array queries**
    ```bash
    jq-lite "users[].name" data.json
    ```

7. **Refactor query parsing logic**
    Replace simple `.split(".")` with a proper parser to support:

    - Arrays
    - Future filters
    - Better extensibility

8. **Validate query syntax before execution**

    Return meaningful errors for invalid queries

## 🔵 Output & Formatting

9. **Add `--pretty` flag for formatted output**

    Pretty-print JSON results

10. **Add `--compact` flag**

    Return minified JSON output

11. **Add `--raw` flag**

    Print raw values without JSON quotes

## 🔴 Error Handling

12. **Improve error handling and messaging**

    - File not found
    - Invalid JSON
    - Missing fields
    - Invalid queries

13. **Add strict mode for query failures**

    - Fail when a query returns no result
    - Return non-zero exit code

## ⚡ Performance & Architecture

14. **Optimize memory usage in query parsing**

    Avoid unnecessary allocations and cloning

15. **Implement query AST (Abstract Syntax Tree)**

    Example:
    ```bash
    users[0].name → Field("users") → Index(0) → Field("name")
    ```

16. **Add streaming JSON parsing**

    Handle large JSON files without loading entire file into memory

## 🧪 Testing

17. **Add unit tests**
- Nested queries
- Missing fields
- Invalid JSON

18. **Add CLI integration tests**

    Test full command execution

## 💡 Future Enhancements (Stretch Goals)

19. **Support pipe operator (|)**

```bash
jq-lite "users | name" data.json
```

20. **Add simple filtering support**

```bash
jq-lite "users[?age>18]" data.json
```

21. **Add interactive REPL mode**

```bash
jq-lite
> user.name
```

## 🏗️ Building

```bash
cargo build --release
```

## 📄 License

MIT

## 🤝 Contributing

Contributions are welcome. Please open an issue before submitting a PR.