use std::env;
use std::process;

use minigrep::config::{Config, ConfigError};
use minigrep::Error;

#[derive(Debug, PartialEq, Eq)]
enum CliOutcome {
    Matched,
    NoMatch,
    Help,
}

fn print_usage() {
    eprintln!("Usage: minigrep [OPTIONS] <query> <filename>");
    eprintln!("Options:");
    eprintln!("  -i, --ignore-case    Perform case insensitive search");
    eprintln!("  -x, --regex          Use regular expression for pattern matching");
    eprintln!("  -r, --recursive      Search recursively through subdirectories");
    eprintln!("  -c, --context        Show 2 lines of context around each match");
    eprintln!("  -c=N, --context=N    Show N lines of context around each match");
    eprintln!("  -h, --help           Show this help message");
}

/// The main entry point for the minigrep application
fn main() {
    match run() {
        Ok(CliOutcome::Matched) | Ok(CliOutcome::Help) => {}
        Ok(CliOutcome::NoMatch) => process::exit(1),
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(2);
        }
    }
}

/// Runs the application, handling errors
fn run() -> Result<CliOutcome, Error> {
    // Parse command line arguments
    let config = match Config::new(env::args()) {
        Ok(config) => config,
        Err(ConfigError::HelpRequested) => {
            print_usage();
            return Ok(CliOutcome::Help);
        }
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
                ConfigError::TooManyArguments(ref argument) => {
                    eprintln!("Too many arguments. Unexpected positional argument: {}", argument);
                }
                ConfigError::HelpRequested => {}
            }

            print_usage();
            return Err(Error::Config(err));
        }
    };

    // Run the application
    match minigrep::run(config) {
        Ok(0) => Ok(CliOutcome::NoMatch),
        Ok(_) => Ok(CliOutcome::Matched),
        Err(e) => {
            eprintln!("Application error: {}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use lazy_static::lazy_static;
    use std::fs;
    use std::process::{Command, Output};
    use std::sync::Mutex;

    lazy_static! {
        static ref CLI_MUTEX: Mutex<()> = Mutex::new(());
    }

    fn run_cli(args: &[&str]) -> Output {
        let _lock = CLI_MUTEX.lock().unwrap();
        Command::new("cargo")
            .args(args)
            .output()
            .expect("Failed to execute command")
    }

    #[test]
    fn test_cli_no_args() {
        // Test running the CLI with no arguments
        let output = run_cli(&["run", "--quiet"]);

        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(stderr.contains("Error parsing arguments"));
        assert!(stderr.contains("Usage: minigrep [OPTIONS] <query> <filename>"));
        assert_eq!(output.status.code(), Some(2));
    }

    #[test]
    fn test_cli_missing_filename() {
        // Test running the CLI with only a query argument
        let output = run_cli(&["run", "--quiet", "--", "test"]);

        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(stderr.contains("Error parsing arguments"));
        assert!(stderr.contains("Missing filename"));
        assert_eq!(output.status.code(), Some(2));
    }

    #[test]
    fn test_cli_nonexistent_file() {
        // Test running the CLI with a nonexistent file
        let output = run_cli(&["run", "--quiet", "--", "test", "nonexistent.txt"]);

        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(stderr.contains("File not found"));
        assert_eq!(output.status.code(), Some(2));
    }

    #[test]
    fn test_cli_with_valid_args() {
        // Test running the CLI with valid arguments
        let output = run_cli(&["run", "--quiet", "--", "body", "poem.txt"]);

        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(stdout.contains("1:I'm nobody! Who are you?"));
        assert_eq!(output.status.code(), Some(0));
    }

    #[test]
    fn test_cli_with_ignore_case_flag() {
        // Test running the CLI with the -i flag
        let output = run_cli(&["run", "--quiet", "--", "-i", "BODY", "poem.txt"]);

        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(stdout.contains("1:I'm nobody! Who are you?"));
        assert_eq!(output.status.code(), Some(0));
    }

    #[test]
    fn test_cli_with_long_ignore_case_flag() {
        // Test running the CLI with the --ignore-case flag
        let output = run_cli(&["run", "--quiet", "--", "--ignore-case", "BODY", "poem.txt"]);

        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(stdout.contains("1:I'm nobody! Who are you?"));
        assert_eq!(output.status.code(), Some(0));
    }

    #[test]
    fn test_cli_with_regex_flag() {
        // Test running the CLI with the -x flag (regex)
        let output = run_cli(&["run", "--quiet", "--", "-x", "b.dy", "poem.txt"]);

        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(stdout.contains("1:I'm nobody! Who are you?"));
        assert_eq!(output.status.code(), Some(0));
    }

    #[test]
    fn test_cli_with_recursive_flag() {
        // Test running the CLI with the -r flag (recursive)
        let output = run_cli(&["run", "--quiet", "--", "-r", "body", "."]);

        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(stdout.contains("File:"));
        assert_eq!(output.status.code(), Some(0));
    }

    #[test]
    fn test_cli_help() {
        let output = run_cli(&["run", "--quiet", "--", "--help"]);

        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(output.status.success());
        assert!(stderr.contains("Usage: minigrep [OPTIONS] <query> <filename>"));
    }

    #[test]
    fn test_cli_no_match_exit_code() {
        let output = run_cli(&["run", "--quiet", "--", "zzzz_missing_pattern", "poem.txt"]);

        assert_eq!(output.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&output.stdout).is_empty());
    }

    #[test]
    fn test_cli_search_invalid_utf8_file() {
        let filename = "test_cli_invalid_utf8.bin";
        fs::write(filename, vec![0xff, 0xfe, b'a', b'b', b'c', b'\n']).unwrap();

        let output = run_cli(&["run", "--quiet", "--", "abc", filename]);

        fs::remove_file(filename).unwrap();

        assert_eq!(output.status.code(), Some(0));
        assert!(String::from_utf8_lossy(&output.stdout).contains("1:"));
    }
}
