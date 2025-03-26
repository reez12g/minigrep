use std::env;
use std::process;

use minigrep::config::{Config, ConfigError};

/// The main entry point for the minigrep application
fn main() {
    // Parse command line arguments
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);

        // Provide more specific usage information based on the error
        match err {
            ConfigError::MissingQuery => {
                eprintln!("Missing query string");
            }
            ConfigError::MissingFilename => {
                eprintln!("Missing filename");
            }
        }

        eprintln!("Usage: minigrep [OPTIONS] <query> <filename>");
        eprintln!("Options:");
        eprintln!("  -i, --ignore-case    Perform case insensitive search");
        process::exit(1);
    });

    // Display search parameters
    println!("Searching for '{}' in '{}'", config.query, config.filename);
    println!("Case sensitive: {}", config.case_sensitive);

    // Run the application
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_cli_no_args() {
        // Test running the CLI with no arguments
        let output = Command::new("cargo")
            .args(&["run", "--quiet"])
            .output()
            .expect("Failed to execute command");

        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(stderr.contains("Error parsing arguments"));
        assert!(stderr.contains("Usage: minigrep [OPTIONS] <query> <filename>"));
    }

    #[test]
    fn test_cli_missing_filename() {
        // Test running the CLI with only a query argument
        let output = Command::new("cargo")
            .args(&["run", "--quiet", "--", "test"])
            .output()
            .expect("Failed to execute command");

        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(stderr.contains("Error parsing arguments"));
        assert!(stderr.contains("Missing filename"));
    }

    #[test]
    fn test_cli_nonexistent_file() {
        // Test running the CLI with a nonexistent file
        let output = Command::new("cargo")
            .args(&["run", "--quiet", "--", "test", "nonexistent.txt"])
            .output()
            .expect("Failed to execute command");

        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(stderr.contains("Application error"));
        assert!(stderr.contains("File not found"));
    }

    #[test]
    fn test_cli_with_valid_args() {
        // Test running the CLI with valid arguments
        let output = Command::new("cargo")
            .args(&["run", "--quiet", "--", "body", "poem.txt"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(stdout.contains("Searching for 'body'"));
        assert!(stdout.contains("Found"));
    }

    #[test]
    fn test_cli_with_ignore_case_flag() {
        // Test running the CLI with the -i flag
        let output = Command::new("cargo")
            .args(&["run", "--quiet", "--", "-i", "BODY", "poem.txt"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(stdout.contains("Searching for 'BODY'"));
        assert!(stdout.contains("Case sensitive: false"));
        assert!(stdout.contains("Found"));
    }

    #[test]
    fn test_cli_with_long_ignore_case_flag() {
        // Test running the CLI with the --ignore-case flag
        let output = Command::new("cargo")
            .args(&["run", "--quiet", "--", "--ignore-case", "BODY", "poem.txt"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(stdout.contains("Searching for 'BODY'"));
        assert!(stdout.contains("Case sensitive: false"));
        assert!(stdout.contains("Found"));
    }
}
