use thiserror::Error;

pub mod config;
pub mod search;
pub mod file;
#[cfg(test)]
pub mod test_utils;

use config::Config;

/// Error types for the minigrep application
#[derive(Debug, Error)]
pub enum Error {
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("File error: {0}")]
    File(#[from] file::FileError),

    #[error("Search error: {0}")]
    Search(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::Search(err.to_string())
    }
}

/// Runs the minigrep application with the given configuration
///
/// # Arguments
///
/// * `config` - The application configuration
///
/// # Returns
///
/// * `Result<(), Error>` - Ok if successful, Err otherwise with a specific error type
///
/// # Errors
///
/// This function will return an error if:
/// - The file cannot be read
/// - The regex pattern is invalid (when using regex search)
/// - Any other I/O operation fails
pub fn run(config: Config) -> Result<(), Error> {
    if config.recursive {
        // Recursive search through directory
        println!("Searching recursively for '{}' in '{}'", config.query, config.filename);

        // Find all text files in the directory
        let files = file::find_text_files(&config.filename)?;

        if files.is_empty() {
            println!("No text files found in '{}'", config.filename);
            return Ok(());
        }

        println!("Searching in {} file(s)...", files.len());

        // Search in all files
        let results = file::search_files(
            &files,
            &config.query,
            config.case_sensitive,
            config.use_regex,
            config.context_lines,
        )?;

        // Print the results
        if results.is_empty() {
            println!("No matches found for '{}'", config.query);
        } else {
            let match_count = results.iter().filter(|m| m.is_match).count();
            println!("Found {} match(es) in {} file(s):", match_count, files.len());

            // Group results by file
            let mut current_file = None;
            let mut current_group = Vec::new();
            let mut last_line_num = 0;

            for file_match in &results {
                // If we're starting a new file
                if current_file.as_ref().map_or(true, |p| p != &file_match.path) {
                    // Print the previous file's results
                    if !current_group.is_empty() {
                        for (num, text, matched) in &current_group {
                            if *matched {
                                println!("{}:{}", num, text);
                            } else {
                                println!("{}~{}", num, text);
                            }
                        }
                        println!("--");
                        current_group.clear();
                    }

                    // Start a new file
                    current_file = Some(file_match.path.clone());
                    println!("\nFile: {}", file_match.path.display());
                    last_line_num = 0;
                }

                // Add separator between non-continuous line groups within the same file
                if !current_group.is_empty() && file_match.line_num > last_line_num + 1 {
                    // Print the current group
                    for (num, text, matched) in &current_group {
                        if *matched {
                            println!("{}:{}", num, text);
                        } else {
                            println!("{}~{}", num, text);
                        }
                    }
                    println!("--");
                    current_group.clear();
                }

                current_group.push((file_match.line_num, &file_match.line, file_match.is_match));
                last_line_num = file_match.line_num;
            }

            // Print the last group
            if !current_group.is_empty() {
                for (num, text, matched) in &current_group {
                    if *matched {
                        println!("{}:{}", num, text);
                    } else {
                        println!("{}~{}", num, text);
                    }
                }
            }
        }
    } else {
        // Regular search in a single file
        println!("Searching for '{}' in '{}'", config.query, config.filename);

        // Read the file contents
        let contents = file::read_file(&config.filename)?;

        // Perform the search
        if config.context_lines > 0 {
            // Use search with context lines
            let results = if config.use_regex {
                // Use regex-based search
                if config.case_sensitive {
                    search::search_regex_with_context_lines(&config.query, &contents, config.context_lines)?
                } else {
                    search::search_regex_case_insensitive_with_context_lines(&config.query, &contents, config.context_lines)?
                }
            } else {
                // Use simple string search
                if config.case_sensitive {
                    search::search_with_context_lines(&config.query, &contents, config.context_lines)
                } else {
                    search::search_case_insensitive_with_context_lines(&config.query, &contents, config.context_lines)
                }
            };

            // Print the results
            if results.is_empty() {
                println!("No matches found for '{}'", config.query);
            } else {
                let match_count = results.iter().filter(|&(_, _, is_match)| *is_match).count();
                println!("Found {} match(es):", match_count);

                let mut current_group = Vec::new();
                let mut last_line_num = 0;

                // Group continuous lines together and separate non-continuous groups
                for (line_num, line, is_match) in results {
                    // Add separator between non-continuous line groups
                    if !current_group.is_empty() && line_num > last_line_num + 1 {
                        // Print the current group
                        for (num, text, matched) in &current_group {
                            if *matched {
                                println!("{}:{}", num, text);
                            } else {
                                println!("{}~{}", num, text);
                            }
                        }
                        println!("--");
                        current_group.clear();
                    }

                    current_group.push((line_num, line, is_match));
                    last_line_num = line_num;
                }

                // Print the last group
                for (num, text, matched) in &current_group {
                    if *matched {
                        println!("{}:{}", num, text);
                    } else {
                        println!("{}~{}", num, text);
                    }
                }
            }
        } else {
            // Use regular search without context
            let results = if config.use_regex {
                // Use regex-based search
                if config.case_sensitive {
                    search::search_regex(&config.query, &contents)?
                } else {
                    search::search_regex_case_insensitive(&config.query, &contents)?
                }
            } else {
                // Use simple string search
                if config.case_sensitive {
                    search::search(&config.query, &contents)
                } else {
                    search::search_case_insensitive(&config.query, &contents)
                }
            };

            // Print the results
            if results.is_empty() {
                println!("No matches found for '{}'", config.query);
            } else {
                println!("Found {} match(es):", results.len());
                for (line_num, line) in results {
                    println!("{}:{}", line_num, line);
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{create_test_file, cleanup_test_file};

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
            use_regex: false,
            context_lines: 0,
            recursive: false,
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
            use_regex: false,
            context_lines: 0,
            recursive: false,
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
            use_regex: false,
            context_lines: 0,
            recursive: false,
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
            use_regex: false,
            context_lines: 0,
            recursive: false,
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
            use_regex: false,
            context_lines: 0,
            recursive: false,
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
            use_regex: false,
            context_lines: 0,
            recursive: false,
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
            use_regex: false,
            context_lines: 0,
            recursive: false,
        };

        // Run the application
        let result = run(config);

        // Clean up
        cleanup_test_file(filename).unwrap();

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_regex() {
        // Create a test file
        let filename = "test_run_with_regex.txt";
        let contents = "Line one\nLine two\nLine three\nAnother line";

        create_test_file(filename, contents).unwrap();

        // Create a config with regex enabled
        let config = Config {
            query: r"\bline\b".to_string(),  // 'line' as a whole word
            filename: filename.to_string(),
            case_sensitive: true,
            use_regex: true,
            context_lines: 0,
            recursive: false,
        };

        // Run the application
        let result = run(config);

        // Clean up
        cleanup_test_file(filename).unwrap();

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_regex_case_insensitive() {
        // Create a test file
        let filename = "test_run_with_regex_case_insensitive.txt";
        let contents = "Line one\nLine two\nLINE three\nAnother line";

        create_test_file(filename, contents).unwrap();

        // Create a config with regex enabled and case insensitive
        let config = Config {
            query: r"line".to_string(),
            filename: filename.to_string(),
            case_sensitive: false,
            use_regex: true,
            context_lines: 0,
            recursive: false,
        };

        // Run the application
        let result = run(config);

        // Clean up
        cleanup_test_file(filename).unwrap();

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_invalid_regex() {
        // Create a test file
        let filename = "test_run_with_invalid_regex.txt";
        let contents = "Line one\nLine two\nLine three";

        create_test_file(filename, contents).unwrap();

        // Create a config with an invalid regex pattern
        let config = Config {
            query: r"[".to_string(),  // Invalid regex pattern
            filename: filename.to_string(),
            case_sensitive: true,
            use_regex: true,
            context_lines: 0,
            recursive: false,
        };

        // Run the application
        let result = run(config);

        // Clean up
        cleanup_test_file(filename).unwrap();

        assert!(result.is_err());
    }

    #[test]
    fn test_run_with_context_lines() {
        // Create a test file
        let filename = "test_run_with_context.txt";
        let contents = "Line one\nLine two\nLine with test\nAnother line\nTest again\nFinal line";

        create_test_file(filename, contents).unwrap();

        // Create a config with context lines enabled
        let config = Config {
            query: "test".to_string(),
            filename: filename.to_string(),
            case_sensitive: true,
            use_regex: false,
            context_lines: 1,
            recursive: false,
        };

        // Run the application
        let result = run(config);

        // Clean up
        cleanup_test_file(filename).unwrap();

        assert!(result.is_ok());
    }

    #[test]
    fn test_run_with_recursive_search() {
        // Create a temporary directory structure with files
        let dir_name = "test_recursive_search_dir";
        let subdir_name = format!("{}/subdir", dir_name);

        let file1 = format!("{}/file1.txt", dir_name);
        let file2 = format!("{}/file2.txt", subdir_name);

        // Create directory structure
        std::fs::create_dir_all(&subdir_name).unwrap();

        // Create test files with content
        create_test_file(&file1, "File one content\nwith test pattern").unwrap();
        create_test_file(&file2, "File two content\nwith test in subdir").unwrap();

        // Create a config with recursive search enabled
        let config = Config {
            query: "test".to_string(),
            filename: dir_name.to_string(),
            case_sensitive: true,
            use_regex: false,
            context_lines: 0,
            recursive: true,
        };

        // Run the application
        let result = run(config);

        // Clean up
        std::fs::remove_dir_all(dir_name).unwrap();

        assert!(result.is_ok());
    }
}
