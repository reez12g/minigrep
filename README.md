# Minigrep

A simple command-line utility for searching text in files, similar to the Unix `grep` command.

## Features

- Search for text in files
- Case-sensitive and case-insensitive search modes
- Detailed error messages
- Modular code structure

## Usage

```bash
# Basic usage
minigrep <query> <filename>

# Example: Search for "body" in poem.txt
minigrep body poem.txt

# Case-insensitive search (using environment variable)
CASE_INSENSITIVE=1 minigrep <query> <filename>

# Example: Case-insensitive search for "body" in poem.txt
CASE_INSENSITIVE=1 minigrep body poem.txt
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

## License

This project is open source and available under the MIT License.
