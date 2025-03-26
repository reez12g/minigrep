use std::env;

/// Configuration for the minigrep application
#[derive(Debug)]
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
    /// * `Result<Config, &'static str>` - A Result containing either a Config or an error message
    pub fn new<T>(mut args: T) -> Result<Config, &'static str> 
    where
        T: Iterator<Item = String>,
    {
        // Skip the program name (first argument)
        args.next();

        // Parse the query string
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing query string"),
        };

        // Parse the filename
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing filename"),
        };

        // Check if case sensitivity is overridden by environment variable
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

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
        assert_eq!(result.unwrap_err(), "Missing query string");
    }

    #[test]
    fn test_config_new_missing_filename() {
        let args = vec!["program", "query"].into_iter().map(String::from);
        let result = Config::new(args);
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Missing filename");
    }
}
