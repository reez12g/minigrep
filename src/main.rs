use std::env;
use std::process;

use minigrep::config::{Config, ConfigError};
use minigrep::Error;

/// The main entry point for the minigrep application
fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}

/// Runs the application, handling errors
fn run() -> Result<(), Error> {
    // Parse command line arguments
    let config = match Config::new(env::args()) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("Error parsing arguments: {}", err);

            // Provide more specific usage information based on the error
            match err {
                ConfigError::MissingQuery => {
                    eprintln!("Missing query string");
                }
                ConfigError::MissingFilename => {
                    eprintln!("Missing filename");
                }
                ConfigError::InvalidContextValue(ref value) => {
                    eprintln!("Invalid context value: {}", value);
                    eprintln!("Context value must be a positive number");
                }
                ConfigError::InvalidOption(ref option) => {
                    eprintln!("Invalid option: {}", option);
                }
            }

            eprintln!("Usage: minigrep [OPTIONS] <query> <filename>");
            eprintln!("Options:");
            eprintln!("  -i, --ignore-case    Perform case insensitive search");
            eprintln!("  -r, --regex          Use regular expression for pattern matching");
            eprintln!("  -c, --context        Show 2 lines of context around each match");
            eprintln!("  -c=N, --context=N    Show N lines of context around each match");
            return Err(Error::Config(err));
        }
    };

    // Display search parameters
    println!("Searching for '{}' in '{}'", config.query, config.filename);
    println!("Case sensitive: {}", config.case_sensitive);
    println!("Using regex: {}", config.use_regex);
    if config.context_lines > 0 {
        println!("Context lines: {}", config.context_lines);
    }

    // Run the application
    match minigrep::run(config) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Application error: {}", e);
            Err(e)
        }
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

    #[test]
    fn test_cli_with_regex_flag() {
        // Test running the CLI with the -r flag
        let output = Command::new("cargo")
            .args(&["run", "--quiet", "--", "-r", "b.dy", "poem.txt"])
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&output.stdout);

        // Print the actual output for debugging
        println!("Actual output: {}", stdout);

        assert!(stdout.contains("Searching for 'b.dy'"));
        assert!(stdout.contains("Using regex: true"));
    }
}
