use std::env;
use thiserror::Error;

/// Error types for Config operations
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing query string")]
    MissingQuery,

    #[error("Missing filename")]
    MissingFilename,

    #[error("Invalid context value: {0}")]
    InvalidContextValue(String),

    #[error("Invalid option: {0}")]
    InvalidOption(String),
}

/// Configuration for the minigrep application
///
/// This struct holds all the configuration options for the minigrep application,
/// including the search query, target filename, and various search options.
#[derive(Debug, Default)]
pub struct Config {
    /// The string or pattern to search for
    pub query: String,

    /// The file to search in
    pub filename: String,

    /// Whether the search is case-sensitive (true) or case-insensitive (false)
    pub case_sensitive: bool,

    /// Whether to use regex pattern matching (true) or simple string matching (false)
    pub use_regex: bool,

    /// Number of context lines to show before and after each match (0 for no context)
    pub context_lines: usize,

    /// Whether to search recursively through subdirectories (true) or just the specified file (false)
    pub recursive: bool,
}

impl Config {
    /// Creates a new Config instance from command line arguments
    ///
    /// # Arguments
    ///
    /// * `args` - An iterator over the command line arguments
    ///
    /// # Returns
    ///
    /// * `Result<Config, ConfigError>` - A Result containing either a Config or an error
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// - No query string is provided (`ConfigError::MissingQuery`)
    /// - No filename is provided (`ConfigError::MissingFilename`)
    /// - An invalid context value is provided (`ConfigError::InvalidContextValue`)
    /// - An invalid option is provided (`ConfigError::InvalidOption`)
    ///
    /// # Examples
    ///
    /// Basic usage:
    /// ```
    /// use minigrep::config::Config;
    ///
    /// let args = vec!["program", "query", "filename"].into_iter().map(String::from);
    /// let config = Config::new(args).unwrap();
    ///
    /// assert_eq!(config.query, "query");
    /// assert_eq!(config.filename, "filename");
    /// ```
    ///
    /// With options:
    /// ```
    /// use minigrep::config::Config;
    ///
    /// let args = vec!["program", "-i", "-c=2", "query", "filename"].into_iter().map(String::from);
    /// let config = Config::new(args).unwrap();
    ///
    /// assert_eq!(config.query, "query");
    /// assert_eq!(config.filename, "filename");
    /// assert!(!config.case_sensitive);
    /// assert_eq!(config.context_lines, 2);
    /// ```
    pub fn new<T>(mut args: T) -> Result<Config, ConfigError>
    where
        T: Iterator<Item = String>,
    {
        // Skip the program name (first argument)
        args.next();

        // Initialize flags
        let mut ignore_case_flag = false;
        let mut use_regex_flag = false;
        let mut recursive_flag = false;
        let mut context_lines = 0;

        // Process all arguments
        let args_vec: Vec<String> = args.collect();

        // Process flags and collect non-flag arguments
        let mut non_flag_args = Vec::new();

        for arg in args_vec {
            if arg == "-i" || arg == "--ignore-case" {
                ignore_case_flag = true;
            } else if arg == "-x" || arg == "--regex" || arg == "-e" || arg == "--regexp" {
                use_regex_flag = true;
            } else if arg == "-r" || arg == "--recursive" {
                recursive_flag = true;
            } else if arg == "-c" || arg == "--context" {
                context_lines = 2; // Default context lines if not specified
            } else if arg.starts_with("-c=") {
                if let Some(value) = arg.strip_prefix("-c=") {
                    match value.parse::<usize>() {
                        Ok(num) => context_lines = num,
                        Err(_) => return Err(ConfigError::InvalidContextValue(value.to_string())),
                    }
                }
            } else if arg.starts_with("--context=") {
                if let Some(value) = arg.strip_prefix("--context=") {
                    match value.parse::<usize>() {
                        Ok(num) => context_lines = num,
                        Err(_) => return Err(ConfigError::InvalidContextValue(value.to_string())),
                    }
                }
            } else if arg.starts_with("-") && arg != "-" {
                // Unknown option
                return Err(ConfigError::InvalidOption(arg.to_string()));
            } else {
                // Not a flag, keep as a positional argument
                non_flag_args.push(arg);
            }
        }

        // Parse the query string
        let query = match non_flag_args.get(0) {
            Some(arg) => arg.clone(),
            None => return Err(ConfigError::MissingQuery),
        };

        // Parse the filename
        let filename = match non_flag_args.get(1) {
            Some(arg) => arg.clone(),
            None => return Err(ConfigError::MissingFilename),
        };

        // Check if case sensitivity is overridden by environment variable or flag
        let case_sensitive = match env::var("CASE_INSENSITIVE") {
            Ok(_) => false, // If CASE_INSENSITIVE is set (to any value), use case insensitive search
            Err(_) => !ignore_case_flag, // Otherwise, use case sensitive search unless -i/--ignore-case is specified
        };

        Ok(Config {
            query,
            filename,
            case_sensitive,
            use_regex: use_regex_flag,
            context_lines,
            recursive: recursive_flag,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;
    use lazy_static::lazy_static;

    // Use a mutex to ensure tests that modify env vars don't run concurrently
    lazy_static! {
        static ref ENV_MUTEX: Mutex<()> = Mutex::new(());
    }

    #[test]
    fn test_config_new_valid_args() {
        let args = vec!["program", "query", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
        assert!(!config.use_regex);
        assert_eq!(config.context_lines, 0);
    }

    #[test]
    fn test_config_new_missing_query() {
        let args = vec!["program"].into_iter().map(String::from);
        let result = Config::new(args);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::MissingQuery));
    }

    #[test]
    fn test_config_new_missing_filename() {
        let args = vec!["program", "query"].into_iter().map(String::from);
        let result = Config::new(args);

        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ConfigError::MissingFilename));
    }

    #[test]
    fn test_config_new_with_extra_args() {
        // Extra arguments should be ignored
        let args = vec!["program", "query", "filename", "extra"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
    }

    #[test]
    fn test_config_case_sensitive_default() {
        // Acquire the mutex to prevent other tests from interfering with env vars
        let _lock = ENV_MUTEX.lock().unwrap();

        // By default, case_sensitive should be true if CASE_INSENSITIVE is not set
        let args = vec!["program", "query", "filename"].into_iter().map(String::from);

        // Explicitly clear the environment variable
        env::remove_var("CASE_INSENSITIVE");

        // Create a new Config
        let config = Config::new(args).unwrap();

        assert!(config.case_sensitive);
        assert!(!config.use_regex);
    }

    #[test]
    fn test_config_case_sensitive_with_env_var() {
        // Acquire the mutex to prevent other tests from interfering with env vars
        let _lock = ENV_MUTEX.lock().unwrap();

        // When CASE_INSENSITIVE is set, case_sensitive should be false
        let args = vec!["program", "query", "filename"].into_iter().map(String::from);

        // Set the environment variable
        env::set_var("CASE_INSENSITIVE", "1");

        // Create a new Config
        let config = Config::new(args).unwrap();

        // Clean up
        env::remove_var("CASE_INSENSITIVE");

        // The case_sensitive flag should be false when CASE_INSENSITIVE is set
        assert!(!config.case_sensitive);
        assert!(!config.use_regex);
    }

    #[test]
    fn test_config_with_empty_query() {
        // Empty query should be valid
        let args = vec!["program", "", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "");
        assert_eq!(config.filename, "filename");
    }

    #[test]
    fn test_config_with_special_characters() {
        // Query with special characters should be valid
        let args = vec!["program", ".*+?^${}()|[]\\", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, ".*+?^${}()|[]\\");
        assert_eq!(config.filename, "filename");
    }

    #[test]
    fn test_config_with_unicode_characters() {
        // Query with Unicode characters should be valid
        let args = vec!["program", "こんにちは世界", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "こんにちは世界");
        assert_eq!(config.filename, "filename");
    }

    #[test]
    fn test_config_with_ignore_case_short_flag() {
        // Test with -i flag
        let args = vec!["program", "-i", "query", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
        assert!(!config.case_sensitive);
    }

    #[test]
    fn test_config_with_ignore_case_long_flag() {
        // Test with --ignore-case flag
        let args = vec!["program", "--ignore-case", "query", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
        assert!(!config.case_sensitive);
    }

    #[test]
    fn test_config_with_flag_in_different_position() {
        // Test with flag in a different position
        let args = vec!["program", "query", "-i", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
        assert!(!config.case_sensitive);
    }

    #[test]
    fn test_config_with_multiple_flags() {
        // Test with both short and long flags (should still work)
        let args = vec!["program", "-i", "--ignore-case", "query", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
        assert!(!config.case_sensitive);
    }

    #[test]
    fn test_config_with_regex_short_flag() {
        // Test with -x flag
        let args = vec!["program", "-x", "pattern", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "pattern");
        assert_eq!(config.filename, "filename");
        assert!(config.use_regex);
    }

    #[test]
    fn test_config_with_regex_long_flag() {
        // Test with --regex flag
        let args = vec!["program", "--regex", "pattern", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "pattern");
        assert_eq!(config.filename, "filename");
        assert!(config.use_regex);
    }

    #[test]
    fn test_config_with_regex_and_ignore_case_flags() {
        // Test with both regex and ignore case flags
        let args = vec!["program", "-x", "-i", "pattern", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "pattern");
        assert_eq!(config.filename, "filename");
        assert!(config.use_regex);
        assert!(!config.case_sensitive);
    }

    #[test]
    fn test_config_with_context_short_flag() {
        // Test with -c flag
        let args = vec!["program", "-c", "query", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
        assert_eq!(config.context_lines, 2);
    }

    #[test]
    fn test_config_with_context_long_flag() {
        // Test with --context flag
        let args = vec!["program", "--context", "query", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
        assert_eq!(config.context_lines, 2);
    }

    #[test]
    fn test_config_with_context_value_short_flag() {
        // Test with -c=3 flag
        let args = vec!["program", "-c=3", "query", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
        assert_eq!(config.context_lines, 3);
    }

    #[test]
    fn test_config_with_context_value_long_flag() {
        // Test with --context=5 flag
        let args = vec!["program", "--context=5", "query", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
        assert_eq!(config.context_lines, 5);
    }

    #[test]
    fn test_config_with_multiple_flags_including_context() {
        // Test with -i, -x, and -c flags together
        let args = vec!["program", "-i", "-x", "-c", "query", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
        assert!(!config.case_sensitive);
        assert!(config.use_regex);
        assert_eq!(config.context_lines, 2);
    }

    #[test]
    fn test_config_with_recursive_short_flag() {
        // Test with -r flag
        let args = vec!["program", "-r", "query", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
        assert!(config.recursive);
    }

    #[test]
    fn test_config_with_recursive_long_flag() {
        // Test with --recursive flag
        let args = vec!["program", "--recursive", "query", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
        assert!(config.recursive);
    }

    #[test]
    fn test_config_with_all_flags() {
        // Test with all flags together
        let args = vec!["program", "-i", "-x", "-r", "-c=3", "query", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
        assert!(!config.case_sensitive);
        assert!(config.use_regex);
        assert!(config.recursive);
        assert_eq!(config.context_lines, 3);
    }
}
