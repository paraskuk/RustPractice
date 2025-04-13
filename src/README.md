Here's a README for this Rust utility project:

# Rust Utility Functions

A collection of basic utility functions implemented in Rust, demonstrating fundamental programming concepts and Rust language features.

## Overview

This project contains several utility functions for working with arrays and strings, showcasing:

- Array manipulation and statistics
- String processing
- Traits and generics
- Structs and implementations
- Enumerations and pattern matching

## Features

### Array Utilities
- `average`: Calculates the mean of an integer array
- `min_array`: Finds the minimum value in an array
- `max_array`: Finds the maximum value in an array

### String Utilities
- `custom_trim`: A from-scratch implementation of string trimming without using standard library functions

### Data Structures
- `Rectangle`: A struct with methods for area calculation and display
- `Shape`: An enum for representing different geometric shapes

### Generic Functions
- `add_boxes`: Demonstrates generics with boxed values
- `compare_and_print`: Shows trait constraints with generics

## Getting Started

```bash
# Clone the repository
git clone <repository-url>

# Navigate to the project directory
cd rust-utility-functions

# Run the project
cargo run
```

## Usage Examples

```rust
// Working with arrays
let arr = vec![1, 2, 3, 4, 5, 6];
let avg = average(&arr);
let min = min_array(&arr);
let max = max_array(&arr);

// Using the custom string trim function
let s = "  Hello, world!  ";
let trimmed = custom_trim(s);

// Working with rectangles
let rect = Rectangle::new(10, 20);
let area = rect.area();
```

## Requirements

- Rust (stable)
- Cargo package manager

## License

This project is available under the [MIT License](LICENSE).