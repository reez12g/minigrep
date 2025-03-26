use std::env;
use thiserror::Error;

/// Error types for Config operations
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing query string")]
    MissingQuery,
    #[error("Missing filename")]
    MissingFilename,
}

/// Configuration for the minigrep application
#[derive(Debug, Default)]
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
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
    /// # Examples
    ///
    /// ```
    /// use minigrep::config::Config;
    ///
    /// let args = vec!["program", "query", "filename"].into_iter().map(String::from);
    /// let config = Config::new(args).unwrap();
    ///
    /// assert_eq!(config.query, "query");
    /// assert_eq!(config.filename, "filename");
    /// ```
    pub fn new<T>(mut args: T) -> Result<Config, ConfigError>
    where
        T: Iterator<Item = String>,
    {
        // Skip the program name (first argument)
        args.next();

        // Initialize flag for case sensitivity
        let mut ignore_case_flag = false;

        // Process all arguments
        let mut args_vec: Vec<String> = args.collect();

        // Check for flags
        args_vec.retain(|arg| {
            if arg == "-i" || arg == "--ignore-case" {
                ignore_case_flag = true;
                false // Remove this argument
            } else {
                true // Keep this argument
            }
        });

        // Parse the query string
        let query = match args_vec.get(0) {
            Some(arg) => arg.clone(),
            None => return Err(ConfigError::MissingQuery),
        };

        // Parse the filename
        let filename = match args_vec.get(1) {
            Some(arg) => arg.clone(),
            None => return Err(ConfigError::MissingFilename),
        };

        // Check if case sensitivity is overridden by environment variable or flag
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err() && !ignore_case_flag;

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_new_valid_args() {
        let args = vec!["program", "query", "filename"].into_iter().map(String::from);
        let config = Config::new(args).unwrap();

        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
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
        // By default, case_sensitive should be true if CASE_INSENSITIVE is not set
        let args = vec!["program", "query", "filename"].into_iter().map(String::from);

        // Temporarily clear the environment variable if it exists
        let original_value = env::var("CASE_INSENSITIVE").ok();
        env::remove_var("CASE_INSENSITIVE");

        let config = Config::new(args).unwrap();

        // Restore the original value if it existed
        if let Some(value) = original_value {
            env::set_var("CASE_INSENSITIVE", value);
        }

        assert!(config.case_sensitive);
    }

    #[test]
    fn test_config_case_sensitive_with_env_var() {
        // When CASE_INSENSITIVE is set, case_sensitive should be false
        let args = vec!["program", "query", "filename"].into_iter().map(String::from);

        // Temporarily set the environment variable
        let original_value = env::var("CASE_INSENSITIVE").ok();
        env::set_var("CASE_INSENSITIVE", "1");

        // Verify the environment variable is set
        assert_eq!(env::var("CASE_INSENSITIVE").unwrap(), "1");

        let config = Config::new(args).unwrap();

        // Restore the original value or remove it
        match original_value {
            Some(value) => env::set_var("CASE_INSENSITIVE", value),
            None => env::remove_var("CASE_INSENSITIVE"),
        }

        // The case_sensitive flag should be false when CASE_INSENSITIVE is set
        assert!(!config.case_sensitive);
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
}
