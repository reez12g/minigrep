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

    #[test]
    fn test_read_file_success() {
        // Create a temporary file
        let filename = "test_read_file_success.txt";
        let content = "Test content";

        {
            let mut file = File::create(filename).unwrap();
            file.write_all(content.as_bytes()).unwrap();
        }

        // Test reading the file
        let result = read_file(filename);

        // Clean up
        fs::remove_file(filename).unwrap();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), content);
    }

    #[test]
    fn test_read_file_not_found() {
        let result = read_file("nonexistent_file.txt");

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("File not found"));
    }
}
