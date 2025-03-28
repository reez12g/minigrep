//! Common test utilities for the minigrep application
//!
//! This module provides utility functions for creating and cleaning up test files
//! used in the test suite. These functions help ensure proper test isolation and
//! cleanup of temporary resources.

use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use thiserror::Error;

/// Error types for test utilities
#[derive(Debug, Error)]
pub enum TestError {
    #[error("Failed to create test file '{0}': {1}")]
    CreateError(String, io::Error),

    #[error("Failed to write to test file '{0}': {1}")]
    WriteError(String, io::Error),

    #[error("Failed to clean up test file '{0}': {1}")]
    CleanupError(String, io::Error),
}

/// Creates a temporary test file with the given content
///
/// # Arguments
///
/// * `filename` - The name of the file to create
/// * `content` - The content to write to the file
///
/// # Returns
///
/// * `Result<(), TestError>` - Ok if successful, Err with specific error otherwise
///
/// # Errors
///
/// This function will return an error if:
/// - The file cannot be created
/// - The content cannot be written to the file
///
/// # Examples
///
/// ```
/// use minigrep::test_utils::create_test_file;
/// use minigrep::test_utils::cleanup_test_file;
///
/// // Create a test file
/// let result = create_test_file("test_example.txt", "Test content");
/// assert!(result.is_ok());
///
/// // Clean up
/// cleanup_test_file("test_example.txt").unwrap();
/// ```
pub fn create_test_file(filename: &str, content: &str) -> Result<(), TestError> {
    let mut file = File::create(filename)
        .map_err(|e| TestError::CreateError(filename.to_string(), e))?;

    file.write_all(content.as_bytes())
        .map_err(|e| TestError::WriteError(filename.to_string(), e))?;

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
/// * `Result<(), TestError>` - Ok if successful, Err with specific error otherwise
///
/// # Errors
///
/// This function will return an error if:
/// - The file exists but cannot be removed due to permissions or other IO errors
///
/// # Examples
///
/// ```
/// use minigrep::test_utils::{create_test_file, cleanup_test_file};
/// use std::path::Path;
///
/// // Create a test file
/// create_test_file("test_cleanup.txt", "Test content").unwrap();
///
/// // Verify it exists
/// assert!(Path::new("test_cleanup.txt").exists());
///
/// // Clean up
/// let result = cleanup_test_file("test_cleanup.txt");
/// assert!(result.is_ok());
///
/// // Verify it's gone
/// assert!(!Path::new("test_cleanup.txt").exists());
/// ```
pub fn cleanup_test_file(filename: &str) -> Result<(), TestError> {
    if Path::new(filename).exists() {
        std::fs::remove_file(filename)
            .map_err(|e| TestError::CleanupError(filename.to_string(), e))?;
    }
    Ok(())
}
