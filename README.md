# CLI Tool

## Project Description

The CLI Tool is a command-line application written in Rust. It is designed to search for a specified query string within a given file and output any lines that contain the query. The tool performs case-insensitive searches to ensure that matches are found regardless of letter case.

## Features

- Searches for a query string within a file.
- Case-insensitive search to improve match accuracy.

## Files

- **`main.rs`**: The entry point of the application. Handles argument parsing and executes the search.
- **`cli.rs`**: Contains functions for running the search, parsing configuration, and performing the search operation.
- **`Config`**: A struct that holds the query string and filename.

## Testing

The project includes tests to verify the following:

- Correct parsing of command-line arguments.
- Proper handling of files, including cases where files do not exist.
- Accurate and case-insensitive search functionality.

Tests can be run using the command `cargo test`.

