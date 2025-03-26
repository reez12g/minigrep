use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Reads the contents of a file into a string
///
/// # Arguments
///
/// * `filename` - The path to the file to read
///
/// # Returns
///
/// * `Result<String, Box<dyn Error>>` - The file contents or an error
pub fn read_file(filename: &str) -> Result<String, Box<dyn Error>> {
    // Check if the file exists
    let path = Path::new(filename);
    if !path.exists() {
        return Err(format!("File not found: {}", filename).into());
    }

    // Open the file
    let mut file = File::open(filename)?;

    // Read the file contents
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    // Helper function to create a temporary test file
    fn create_test_file(filename: &str, content: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    // Helper function to clean up test files
    fn cleanup_test_file(filename: &str) -> std::io::Result<()> {
        if Path::new(filename).exists() {
            fs::remove_file(filename)?;
        }
        Ok(())
    }

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
