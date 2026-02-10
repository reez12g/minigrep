use std::fs::{self, File};
use std::io::{self, prelude::*};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
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

    #[error("Path is not a directory: {0}")]
    NotADirectory(String),

    #[error("Path is not a file: {0}")]
    NotAFile(String),
}

/// Represents a file match with path and content
#[derive(Debug)]
pub struct FileMatch {
    /// Path to the file
    pub path: PathBuf,
    /// Line number (1-indexed)
    pub line_num: usize,
    /// Line content
    pub line: String,
    /// Whether this is a direct match or a context line
    pub is_match: bool,
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

/// Checks if a path is a file and has a valid UTF-8 text content
///
/// # Arguments
///
/// * `path` - The path to check
///
/// # Returns
///
/// * `bool` - True if the path is a file with valid UTF-8 content, false otherwise
fn is_text_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    // Try to read a small portion of the file to check if it's valid UTF-8
    if let Ok(file) = File::open(path) {
        let mut buffer = [0; 1024]; // Read first 1KB to check
        let mut reader = io::BufReader::new(file);

        match reader.read(&mut buffer) {
            Ok(bytes_read) if bytes_read > 0 => {
                // Check if the content is valid UTF-8
                return String::from_utf8(buffer[..bytes_read].to_vec()).is_ok();
            }
            Ok(_) => return true, // Empty file is considered a text file
            Err(_) => return false,
        }
    }

    false
}

/// Directory names to skip during recursive search to avoid scanning build/VCS metadata.
const DEFAULT_IGNORED_DIRS: [&str; 3] = [".git", "target", "node_modules"];

fn should_skip_directory(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| DEFAULT_IGNORED_DIRS.contains(&name))
        .unwrap_or(false)
}

fn collect_text_files(
    dir: &Path,
    visited: &mut HashSet<PathBuf>,
    result: &mut Vec<PathBuf>,
) -> Result<(), FileError> {
    let canonical = fs::canonicalize(dir)?;
    if !visited.insert(canonical) {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let entry_path = entry.path();
        let metadata = fs::symlink_metadata(&entry_path)?;
        let file_type = metadata.file_type();

        // Never recurse into symlinks to avoid directory cycles.
        if file_type.is_symlink() {
            continue;
        }

        if file_type.is_dir() {
            if should_skip_directory(&entry_path) {
                continue;
            }
            collect_text_files(&entry_path, visited, result)?;
        } else if file_type.is_file() && is_text_file(&entry_path) {
            result.push(entry_path);
        }
    }

    Ok(())
}

/// Recursively finds all text files in a directory
///
/// # Arguments
///
/// * `dir_path` - The directory path to search in
///
/// # Returns
///
/// * `Result<Vec<PathBuf>, FileError>` - A vector of paths to text files or an error
///
/// # Errors
///
/// This function will return an error if:
/// - The directory does not exist (`FileError::NotFound`)
/// - The path is not a directory (`FileError::NotADirectory`)
/// - There is an IO error while reading the directory (`FileError::IoError`)
pub fn find_text_files<P: AsRef<Path>>(dir_path: P) -> Result<Vec<PathBuf>, FileError> {
    let path = dir_path.as_ref();
    let path_display = path.to_string_lossy();

    if !path.exists() {
        return Err(FileError::NotFound(path_display.to_string()));
    }

    if !path.is_dir() {
        if path.is_file() {
            // If it's a single file, return it as a vector with one element
            return Ok(vec![path.to_path_buf()]);
        } else {
            return Err(FileError::NotADirectory(path_display.to_string()));
        }
    }

    let mut visited = HashSet::new();
    let mut result = Vec::new();
    collect_text_files(path, &mut visited, &mut result)?;

    Ok(result)
}

/// Searches for matches in a file
///
/// # Arguments
///
/// * `path` - Path to the file
/// * `query` - The string or pattern to search for
/// * `case_sensitive` - Whether the search is case-sensitive
/// * `use_regex` - Whether to use regex pattern matching
/// * `context_lines` - Number of context lines to include
///
/// # Returns
///
/// * `Result<Vec<FileMatch>, FileError>` - A vector of file matches or an error
pub fn search_file(
    path: &Path,
    query: &str,
    case_sensitive: bool,
    use_regex: bool,
    context_lines: usize,
) -> Result<Vec<FileMatch>, FileError> {
    // Read the file contents
    let file_path_str = path.to_string_lossy().to_string();
    let contents = read_file(&file_path_str)?;

    let mut results = Vec::new();

    // Perform the search based on the options
    let matches = if use_regex {
        if case_sensitive {
            match crate::search::search_regex_with_context_lines(query, &contents, context_lines) {
                Ok(m) => m,
                Err(e) => return Err(FileError::ReadError(e.to_string())),
            }
        } else {
            match crate::search::search_regex_case_insensitive_with_context_lines(query, &contents, context_lines) {
                Ok(m) => m,
                Err(e) => return Err(FileError::ReadError(e.to_string())),
            }
        }
    } else {
        if case_sensitive {
            crate::search::search_with_context_lines(query, &contents, context_lines)
        } else {
            crate::search::search_case_insensitive_with_context_lines(query, &contents, context_lines)
        }
    };

    // Convert the matches to FileMatch structs
    for (line_num, line, is_match) in matches {
        results.push(FileMatch {
            path: path.to_path_buf(),
            line_num,
            line: line.to_string(),
            is_match,
        });
    }

    Ok(results)
}

/// Searches for matches in multiple files
///
/// # Arguments
///
/// * `files` - Vector of file paths to search in
/// * `query` - The string or pattern to search for
/// * `case_sensitive` - Whether the search is case-sensitive
/// * `use_regex` - Whether to use regex pattern matching
/// * `context_lines` - Number of context lines to include
///
/// # Returns
///
/// * `Result<Vec<FileMatch>, FileError>` - A vector of file matches or an error
pub fn search_files(
    files: &[PathBuf],
    query: &str,
    case_sensitive: bool,
    use_regex: bool,
    context_lines: usize,
) -> Result<Vec<FileMatch>, FileError> {
    let mut all_results = Vec::new();

    for file_path in files {
        match search_file(file_path, query, case_sensitive, use_regex, context_lines) {
            Ok(mut results) => {
                all_results.append(&mut results);
            }
            Err(e) => {
                // Skip files that can't be read or searched
                eprintln!("Warning: {}", e);
            }
        }
    }

    Ok(all_results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
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

    #[test]
    fn test_find_text_files_single_file() {
        // Create a temporary file
        let filename = "test_find_text_files_single.txt";
        let content = "Test content";

        create_test_file(filename, content).unwrap();

        // Test finding the file
        let result = find_text_files(filename);

        // Clean up
        cleanup_test_file(filename).unwrap();

        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 1);
        assert_eq!(files[0].file_name().unwrap(), "test_find_text_files_single.txt");
    }

    #[test]
    fn test_find_text_files_directory() {
        // Create a temporary directory with files
        let dir_name = "test_find_text_files_dir";
        let file1 = format!("{}/file1.txt", dir_name);
        let file2 = format!("{}/file2.txt", dir_name);
        let subdir = format!("{}/subdir", dir_name);
        let file3 = format!("{}/file3.txt", subdir);

        // Create directory structure
        fs::create_dir_all(&subdir).unwrap();
        create_test_file(&file1, "Content 1").unwrap();
        create_test_file(&file2, "Content 2").unwrap();
        create_test_file(&file3, "Content 3").unwrap();

        // Test finding files recursively
        let result = find_text_files(dir_name);

        // Clean up
        fs::remove_dir_all(dir_name).unwrap();

        assert!(result.is_ok());
        let files = result.unwrap();
        assert_eq!(files.len(), 3);

        // Check that all files were found (order may vary)
        let file_names: Vec<String> = files.iter()
            .map(|p| p.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        assert!(file_names.contains(&"file1.txt".to_string()));
        assert!(file_names.contains(&"file2.txt".to_string()));
        assert!(file_names.contains(&"file3.txt".to_string()));
    }

    #[test]
    fn test_search_file() {
        // Create a temporary file
        let filename = "test_search_file.txt";
        let content = "Line one\nLine two\nLine three with test\nLine four";

        create_test_file(filename, content).unwrap();

        // Test searching the file
        let path = Path::new(filename);
        let result = search_file(path, "test", true, false, 1);

        // Clean up
        cleanup_test_file(filename).unwrap();

        assert!(result.is_ok());
        let matches = result.unwrap();
        assert_eq!(matches.len(), 3); // Match + context lines

        // Check the match
        let match_line = matches.iter().find(|m| m.is_match).unwrap();
        assert_eq!(match_line.line_num, 3);
        assert_eq!(match_line.line, "Line three with test");
        assert_eq!(match_line.path, path);
    }

    #[test]
    fn test_search_files() {
        // Create temporary files
        let dir_name = "test_search_files_dir";
        let file1 = format!("{}/file1.txt", dir_name);
        let file2 = format!("{}/file2.txt", dir_name);

        fs::create_dir(dir_name).unwrap();
        create_test_file(&file1, "File one content\nwith test pattern\nand more lines").unwrap();
        create_test_file(&file2, "File two content\nno match here\nbut test is on this line").unwrap();

        // Get the file paths
        let files = vec![
            PathBuf::from(&file1),
            PathBuf::from(&file2),
        ];

        // Test searching multiple files
        let result = search_files(&files, "test", true, false, 0);

        // Clean up
        fs::remove_dir_all(dir_name).unwrap();

        assert!(result.is_ok());
        let matches = result.unwrap();
        assert_eq!(matches.len(), 2); // One match from each file

        // Check that we have matches from both files
        let file_paths: Vec<PathBuf> = matches.iter()
            .map(|m| m.path.clone())
            .collect();

        assert!(file_paths.contains(&PathBuf::from(&file1)));
        assert!(file_paths.contains(&PathBuf::from(&file2)));
    }

    #[test]
    fn test_recursive_directory_search() {
        // Create a temporary directory structure with files
        let dir_name = "test_recursive_dir";
        let subdir_name = format!("{}/subdir", dir_name);
        let nested_subdir = format!("{}/nested", subdir_name);

        let file1 = format!("{}/file1.txt", dir_name);
        let file2 = format!("{}/file2.txt", dir_name);
        let file3 = format!("{}/file3.txt", subdir_name);
        let file4 = format!("{}/file4.txt", nested_subdir);

        // Create directory structure
        fs::create_dir_all(&nested_subdir).unwrap();

        // Create test files with content
        create_test_file(&file1, "File one content\nwith test pattern").unwrap();
        create_test_file(&file2, "File two content\nno match here").unwrap();
        create_test_file(&file3, "File three content\nwith test in subdir").unwrap();
        create_test_file(&file4, "File four content\nwith test in nested subdir").unwrap();

        // Test finding files recursively
        let files = find_text_files(dir_name).unwrap();
        assert_eq!(files.len(), 4);

        // Test searching in all files
        let result = search_files(&files, "test", true, false, 0).unwrap();
        assert_eq!(result.len(), 3); // Three files contain "test"

        // Clean up
        fs::remove_dir_all(dir_name).unwrap();
    }
}
