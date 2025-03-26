use std::error::Error;

pub mod config;
pub mod search;
pub mod file;

use config::Config;

/// Runs the minigrep application with the given configuration
///
/// # Arguments
///
/// * `config` - The application configuration
///
/// # Returns
///
/// * `Result<(), Box<dyn Error>>` - Ok if successful, Err otherwise
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // Read the file contents
    let contents = file::read_file(&config.filename)?;

    // Perform the search
    let results = if config.case_sensitive {
        search::search(&config.query, &contents)
    } else {
        search::search_case_insensitive(&config.query, &contents)
    };

    // Print the results
    if results.is_empty() {
        println!("No matches found for '{}'", config.query);
    } else {
        println!("Found {} match(es):", results.len());
        for line in results {
            println!("{}", line);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_run_with_matches() {
        // Create a test file
        let filename = "test_run_with_matches.txt";
        let contents = "Line with test\nAnother line\nTest again";

        {
            let mut file = File::create(filename).unwrap();
            file.write_all(contents.as_bytes()).unwrap();
        }

        // Create a config
        let config = Config {
            query: "test".to_string(),
            filename: filename.to_string(),
            case_sensitive: true,
        };

        // Run the application
        let result = run(config);

        // Clean up
        std::fs::remove_file(filename).unwrap();

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_file_not_found() {
        let config = Config {
            query: "test".to_string(),
            filename: "nonexistent_file.txt".to_string(),
            case_sensitive: true,
        };

        let result = run(config);

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("File not found"));
    }
}
