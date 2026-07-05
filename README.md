# JSON Parser in Rust

A simple JSON parser written from scratch in Rust as a learning project. The parser tokenizes a JSON string and builds an in-memory representation of the JSON using a recursive descent parser.

## Features

* Parses JSON objects
* Parses JSON arrays
* Supports strings
* Supports integer numbers
* Supports booleans (`true` and `false`)
* Supports `null`
* Supports nested objects and arrays
* Simple API to access object fields

## Project Structure

```text
src/
├── lexer.rs      // Tokenizes the input 
├── parser.rs     // Builds the JSON tree
└── main.rs       // Example usage
```

> If everything is currently in a single file, this structure represents how the project can be organized as it grows.

## Getting Started

### Clone the repository

```bash
git clone <repository-url>
cd <repository-name>
```

### Run the project

```bash
cargo run
```

The example JSON inside `main.rs` will be tokenized, parsed, and the parsed output will be printed to the console.

## Example

Input:

```json
{
    "name": "alice",
    "age": 30,
    "scores": [95, 87, 100],
    "active": true
}
```

Example usage:

```rust
let mut lexer = Lexer::new(raw_json);
let tokens = lexer.tokenize();

let json = Parser::parse(tokens);

println!("{:#?}", json);
```

You can also access fields from the parsed object:

```rust
if let Some(value) = json.get("age") {
    println!("{:?}", value);
}
```

## Supported JSON Types

* Object
* Array
* String
* Integer Number
* Boolean
* Null

## Current Limitations

This project is intended for learning parser implementation and currently does not support:

* Floating-point numbers
* Negative numbers
* Scientific notation
* Escaped characters inside strings
* Unicode escape sequences
* Detailed error reporting

## Future Improvements

* Better error handling using `Result`
* Full JSON specification support
* JSON serialization (`JsonValue` → JSON string)
* More comprehensive test suite
* Improved object storage using `HashMap`

## License

This project is open for learning and experimentation. Feel free to explore the code, suggest improvements, or build upon it.
