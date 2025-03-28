use std::fs::File;
use std::io::{self, prelude::*};
use thiserror::Error;

/// Error types for file operations
#[derive(Debug, Error)]
pub enum FileError {
    #[error("File not found: {0}")]
    NotFound(String),

    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    #[error("Failed to read file: {0}")]
    ReadError(String),
}

/// Reads the contents of a file into a string
///
/// # Arguments
///
/// * `filename` - The path to the file to read
///
/// # Returns
///
/// * `Result<String, FileError>` - The file contents or a specific error
///
/// # Errors
///
/// This function will return an error if:
/// - The file does not exist (`FileError::NotFound`)
/// - The file cannot be read due to permissions or other IO errors (`FileError::IoError`)
/// - The file contains invalid UTF-8 (`FileError::ReadError`)
///
/// # Examples
///
/// ```
/// use minigrep::file::read_file;
/// use std::fs::File;
/// use std::io::Write;
///
/// // Create a temporary file
/// let filename = "example.txt";
/// let content = "Hello, world!";
/// let mut file = File::create(filename).unwrap();
/// file.write_all(content.as_bytes()).unwrap();
///
/// // Read the file
/// let result = read_file(filename);
/// assert!(result.is_ok());
/// assert_eq!(result.unwrap(), content);
///
/// // Clean up
/// std::fs::remove_file(filename).unwrap();
/// ```
pub fn read_file(filename: &str) -> Result<String, FileError> {
    // Open the file, handling the "not found" case specifically
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            return Err(FileError::NotFound(filename.to_string()));
        }
        Err(e) => return Err(FileError::IoError(e)),
    };

    // Read the file contents
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(FileError::ReadError(e.to_string())),
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    use crate::test_utils::{create_test_file, cleanup_test_file};

    #[test]
    fn test_read_file_success() {
        // Create a temporary file
        let filename = "test_read_file_success.txt";
        let content = "Test content";

        create_test_file(filename, content).unwrap();

        // Test reading the file
        let result = read_file(filename);

        // Clean up
        cleanup_test_file(filename).unwrap();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);
    }

    #[test]
    fn test_read_file_not_found() {
        let filename = "nonexistent_file.txt";

        // Ensure the file doesn't exist
        cleanup_test_file(filename).ok();

        let result = read_file(filename);

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("File not found"));
    }

    #[test]
    fn test_read_file_empty() {
        // Create an empty file
        let filename = "test_read_file_empty.txt";

        create_test_file(filename, "").unwrap();

        // Test reading the empty file
        let result = read_file(filename);

        // Clean up
        cleanup_test_file(filename).unwrap();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_read_file_with_multiple_lines() {
        // Create a file with multiple lines
        let filename = "test_read_file_multiline.txt";
        let content = "Line 1\nLine 2\nLine 3";

        create_test_file(filename, content).unwrap();

        // Test reading the file
        let result = read_file(filename);

        // Clean up
        cleanup_test_file(filename).unwrap();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);
    }

    #[test]
    fn test_read_file_with_unicode() {
        // Create a file with Unicode characters
        let filename = "test_read_file_unicode.txt";
        let content = "こんにちは世界\n你好，世界\nHello, World!";

        create_test_file(filename, content).unwrap();

        // Test reading the file
        let result = read_file(filename);

        // Clean up
        cleanup_test_file(filename).unwrap();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);
    }
}
