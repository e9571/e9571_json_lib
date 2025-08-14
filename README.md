# e9571_json_lib Usage Examples

This document demonstrates the usage of the `e9571_json_lib` module in a Rust program, designed for casino-related JSON operations such as parsing, formatting, and converting data for user profiles, betting records, or API payloads.

## Source Code Example

Below is a Rust program showcasing various JSON-related functions from the `e9571_json_lib` module. The code uses `serde_json` for JSON handling and performs operations like JSON parsing, conversion, and type checking.

```rust
use e9571_json_lib::e9571_json_lib::*;
use serde_json::{Value, json};
use std::collections::HashMap;

fn main() {
    // Example 1: Json_Package
    println!("=== Json_Package ===");
    let value = json!({"name": "Alice", "age": 30});
    println!("JSON: {}", json_package(&value));

    // Example 2: Str_To_Json
    println!("\n=== Str_To_Json ===");
    let json_str = r#"{"name": "Bob", "age": 25}"#;
    println!("JSON to Map: {:?}", str_to_json(json_str));
    let non_standard = "name:Charlie,age:40";
    println!("Non-standard JSON to Map: {:?}", str_to_json(non_standard));

    // Example 3: Map_to_json
    println!("\n=== Map_to_json ===");
    let mut map: HashMap<String, Value> = HashMap::new();
    map.insert("name".to_string(), json!("Dave"));
    map.insert("age".to_string(), json!(50));
    println!("Map to JSON: {}", map_to_json(&map));

    // Example 4: Json_to_map
    println!("\n=== Json_to_map ===");
    match json_to_map(json_str) {
        Ok(map) => println!("Map: {:?}", map),
        Err(e) => println!("Error: {}", e),
    }
    let invalid_json = "invalid";
    match json_to_map(invalid_json) {
        Ok(map) => println!("Map: {:?}", map),
        Err(e) => println!("Invalid JSON Error: {}", e),
    }

    // Example 5: Typeof_Json
    println!("\n=== Typeof_Json ===");
    println!("Type of string: {}", typeof_json(&json!("test")));
    println!("Type of int: {}", typeof_json(&json!(42)));
    println!("Type of float: {}", typeof_json(&json!(3.14)));
    println!("Type of object: {}", typeof_json(&json!({"key": "value"})));

    // Example 6: Str_interface_to_json
    println!("\n=== Str_interface_to_json ===");
    let non_standard = "name:Alice,age:30";
    println!("Non-standard JSON: {:?}", str_interface_to_json(non_standard));

    // Example 7: Str_To_Json_node
    println!("\n=== Str_To_Json_node ===");
    let node_json = r#"["name":"Alice","age":"30"]"#;
    println!("Node.js JSON: {:?}", str_to_json_node(node_json));

    // Example 8: List_to_Json
    println!("\n=== List_to_Json ===");
    let list = vec!["item1".to_string(), "item2".to_string()];
    println!("List to JSON: {}", list_to_json(&list));
    let empty_list: Vec<String> = vec![];
    println!("Empty List to JSON: {}", list_to_json(&empty_list));
}
```

## Explanation of Functions

The `e9571_json_lib` module provides utility functions for JSON processing, ideal for casino applications to handle user data, betting records, or API responses.

1. **`json_package`**:
   - Converts a `serde_json::Value` to a formatted JSON string.
   - **Use Case**: Serializing user profiles or bet details for storage or API payloads.

2. **`str_to_json`**:
   - Parses a JSON string (standard or non-standard) into a `HashMap`.
   - **Use Case**: Converting user input or API responses into structured data.

3. **`map_to_json`**:
   - Converts a `HashMap<String, Value>` to a JSON string.
   - **Use Case**: Formatting structured data for database storage or API responses.

4. **`json_to_map`**:
   - Parses a JSON string into a `HashMap`, with error handling for invalid JSON.
   - **Use Case**: Extracting fields from betting records or user data.

5. **`typeof_json`**:
   - Returns the type of a `serde_json::Value` (e.g., string, int, float, object).
   - **Use Case**: Validating data types in JSON payloads (e.g., bet amounts or user IDs).

6. **`str_interface_to_json`**:
   - Converts a non-standard string format (e.g., `key:value`) to a JSON object.
   - **Use Case**: Processing user inputs or legacy data formats.

7. **`str_to_json_node`**:
   - Parses Node.js-style JSON (e.g., array-based key-value pairs) into a structured format.
   - **Use Case**: Handling JSON from Node.js-based casino APIs.

8. **`list_to_json`**:
   - Converts a vector of strings to a JSON array.
   - **Use Case**: Serializing lists of bet options or game results.

## Casino Scenario Usage

These functions are ideal for casino applications, such as:
- **User Data Processing**: Serializing or parsing user profiles (`json_package`, `str_to_json`, `map_to_json`).
- **API Integration**: Handling JSON responses from betting or payment APIs (`json_to_map`, `str_to_json_node`).
- **Data Validation**: Checking data types in JSON payloads (`typeof_json`).
- **Input Handling**: Converting non-standard user inputs to JSON (`str_interface_to_json`).
- **List Serialization**: Formatting game options or results as JSON arrays (`list_to_json`).

## Example Output

The output depends on the implementation of `e9571_json_lib`. An example output might look like:

```
=== Json_Package ===
JSON: {"name":"Alice","age":30}

=== Str_To_Json ===
JSON to Map: {"name": String("Bob"), "age": Number(25)}
Non-standard JSON to Map: {"name": String("Charlie"), "age": Number(40)}

=== Map_to_json ===
Map to JSON: {"name":"Dave","age":50}

=== Json_to_map ===
Map: {"name": String("Bob"), "age": Number(25)}
Invalid JSON Error: expected value at line 1 column 1

=== Typeof_Json ===
Type of string: string
Type of int: number
Type of float: number
Type of object: object

=== Str_interface_to_json ===
Non-standard JSON: {"name":"Alice","age":30}

=== Str_To_Json_node ===
Node.js JSON: {"name":"Alice","age":"30"}

=== List_to_Json ===
List to JSON: ["item1","item2"]
Empty List to JSON: []
```

## Notes
- **Module Dependency**: The `e9571_json_lib` module is assumed to be available and correctly implemented, relying on `serde_json` for JSON handling.
- **Error Handling**: Functions like `json_to_map` use `Result` to handle invalid JSON gracefully.
- **Casino Context**: These functions are suitable for managing user data, bet records, or API payloads in a casino system.
- **GitHub Rendering**: This Markdown uses Rust syntax highlighting, clear headings, and structured explanations for optimal display.
- **Security**: Ensure proper validation of inputs (e.g., JSON strings) to prevent parsing errors in production.