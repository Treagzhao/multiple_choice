# multiple_choice

A Rust proc-macro library that provides an attribute macro `#[triple_verify]` for verifying function results through multiple executions.

## Overview

The `#[triple_verify]` attribute macro automatically executes a function three times and compares the results:
- If all three results match, returns the result
- If two results match, returns the matching result and prints a warning
- If all three results differ, panics with a detailed error message

## Features

- **Easy to use**: Simply add `#[triple_verify]` to any function
- **Flexible**: Works with any function that has non-mutable borrow parameters and returns a type implementing `PartialEq`
- **Informative**: Includes function name in error and warning messages for easier debugging
- **Supports complex types**: Works with tuples, arrays, vectors, and custom structs

## Requirements

- Rust 2024 edition or later
- Functions using this macro must:
  1. Have no mutable borrow parameters
  2. Return a type that implements `PartialEq` (or tuples/arrays of such types)

## Installation

Add this library as a dependency to your Cargo.toml:

```toml
[dependencies]
multiple_choice = { path = "../path/to/multiple_choice" }
```

## Usage

```rust
extern crate multiple_choice;

use multiple_choice::triple_verify;

#[triple_verify]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[triple_verify]
fn create_struct(id: i32, name: &str, value: f64) -> MyStruct {
    MyStruct {
        id,
        name: name.to_string(),
        value,
    }
}

fn main() {
    println!("Result: {}", add(2, 3));
}
```

## Example Output

### Normal case (all results match):
```
Result: 5
```

### Warning case (two results match):
```
WARNING: Function add: Two results match, one differs
Result: 5
```

### Panic case (all results differ):
```
thread 'main' panicked at src/main.rs:10:1:
Function different_value: All three results differ: 1, 2, 3
```

## Testing

The library includes a comprehensive test project that verifies:
- Basic types (i32, bool, f64)
- Tuples (including tuples of structs)
- Arrays (including arrays of structs)
- Vectors (Vec<T>)
- Custom structs
- Panic behavior

To run the tests:
```bash
cd test_project
cargo test
```

## License

MIT