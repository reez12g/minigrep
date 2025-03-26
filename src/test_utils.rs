//! Common test utilities for the minigrep application

use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Creates a temporary test file with the given content
///
/// # Arguments
///
/// * `filename` - The name of the file to create
/// * `content` - The content to write to the file
///
/// # Returns
///
/// * `std::io::Result<()>` - Ok if successful, Err otherwise
pub fn create_test_file(filename: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

/// Cleans up a test file if it exists
///
/// # Arguments
///
/// * `filename` - The name of the file to clean up
///
/// # Returns
///
/// * `std::io::Result<()>` - Ok if successful, Err otherwise
pub fn cleanup_test_file(filename: &str) -> std::io::Result<()> {
    if Path::new(filename).exists() {
        std::fs::remove_file(filename)?;
    }
    Ok(())
}
