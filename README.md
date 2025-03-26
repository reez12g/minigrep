# Minigrep

A simple command-line utility for searching text in files, similar to the Unix `grep` command.

## Features

- Search for text in files
- Case-sensitive and case-insensitive search modes
- Regular expression pattern matching
- Command-line flags for easy configuration
- Detailed error messages
- Modular code structure

## Usage

```bash
# Basic usage
minigrep [OPTIONS] <query> <filename>

# Options:
#   -i, --ignore-case    Perform case insensitive search
#   -r, --regex          Use regular expression for pattern matching

# Example: Search for "body" in poem.txt
minigrep body poem.txt

# Example: Case-insensitive search for "body" in poem.txt
minigrep -i body poem.txt
# or
CASE_INSENSITIVE=1 minigrep body poem.txt

# Example: Regular expression search
minigrep -r "b.dy" poem.txt

# Example: Case-insensitive regular expression search
minigrep -i -r "b.dy" poem.txt
```

## Project Structure

The project has been refactored into a modular structure:

- `src/main.rs`: Entry point for the application
- `src/lib.rs`: Main library code and integration tests
- `src/config/mod.rs`: Configuration handling
- `src/search/mod.rs`: Text search functionality
- `src/file/mod.rs`: File operations

## Development

### Building

```bash
cargo build
```

### Running

```bash
cargo run -- <query> <filename>
```

### Testing

```bash
cargo test
```

The project includes comprehensive tests for all modules:

- **Config Tests**: Tests for command-line argument parsing, environment variable handling, and various input types
- **Search Tests**: Tests for case-sensitive and case-insensitive search, empty queries/contents, Unicode support, and more
- **File Tests**: Tests for file reading, error handling, and various file content types
- **Integration Tests**: Tests for the main application functionality and CLI behavior

## License

This project is open source and available under the MIT License.
