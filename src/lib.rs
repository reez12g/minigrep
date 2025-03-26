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
    use std::path::Path;

    // Helper function to create a temporary test file
    fn create_test_file(filename: &str, content: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    // Helper function to clean up test files
    fn cleanup_test_file(filename: &str) -> std::io::Result<()> {
        if Path::new(filename).exists() {
            std::fs::remove_file(filename)?;
        }
        Ok(())
    }

    #[test]
    fn test_run_with_matches() {
        // Create a test file
        let filename = "test_run_with_matches.txt";
        let contents = "Line with test\nAnother line\nTest again";

        create_test_file(filename, contents).unwrap();

        // Create a config
        let config = Config {
            query: "test".to_string(),
            filename: filename.to_string(),
            case_sensitive: true,
        };

        // Run the application
        let result = run(config);

        // Clean up
        cleanup_test_file(filename).unwrap();

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_case_insensitive_matches() {
        // Create a test file
        let filename = "test_run_with_case_insensitive.txt";
        let contents = "Line with TEST\nAnother line\nTest again";

        create_test_file(filename, contents).unwrap();

        // Create a config with case_sensitive = false
        let config = Config {
            query: "test".to_string(),
            filename: filename.to_string(),
            case_sensitive: false,
        };

        // Run the application
        let result = run(config);

        // Clean up
        cleanup_test_file(filename).unwrap();

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_no_matches() {
        // Create a test file
        let filename = "test_run_with_no_matches.txt";
        let contents = "Line one\nLine two\nLine three";

        create_test_file(filename, contents).unwrap();

        // Create a config
        let config = Config {
            query: "nonexistent".to_string(),
            filename: filename.to_string(),
            case_sensitive: true,
        };

        // Run the application
        let result = run(config);

        // Clean up
        cleanup_test_file(filename).unwrap();

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_empty_file() {
        // Create an empty test file
        let filename = "test_run_with_empty_file.txt";

        create_test_file(filename, "").unwrap();

        // Create a config
        let config = Config {
            query: "test".to_string(),
            filename: filename.to_string(),
            case_sensitive: true,
        };

        // Run the application
        let result = run(config);

        // Clean up
        cleanup_test_file(filename).unwrap();

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_empty_query() {
        // Create a test file
        let filename = "test_run_with_empty_query.txt";
        let contents = "Line one\nLine two\nLine three";

        create_test_file(filename, contents).unwrap();

        // Create a config with an empty query
        let config = Config {
            query: "".to_string(),
            filename: filename.to_string(),
            case_sensitive: true,
        };

        // Run the application
        let result = run(config);

        // Clean up
        cleanup_test_file(filename).unwrap();

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

    #[test]
    fn test_run_with_unicode_query() {
        // Create a test file with Unicode content
        let filename = "test_run_with_unicode.txt";
        let contents = "こんにちは世界\n你好，世界\nHello, World!";

        create_test_file(filename, contents).unwrap();

        // Create a config with a Unicode query
        let config = Config {
            query: "世界".to_string(),
            filename: filename.to_string(),
            case_sensitive: true,
        };

        // Run the application
        let result = run(config);

        // Clean up
        cleanup_test_file(filename).unwrap();

        assert!(result.is_ok());
    }
}
