use regex::{Regex, RegexBuilder};

/// Searches for lines in contents that match a predicate function
///
/// # Arguments
///
/// * `contents` - The text to search in
/// * `predicate` - A function that takes a line and returns true if it matches
///
/// # Returns
///
/// * `Vec<(usize, &str)>` - A vector of tuples containing line numbers (1-indexed) and matching lines
///
/// # Examples
///
/// ```
/// use minigrep::search::search_with;
///
/// let contents = "Line one\nLine two\nLine three";
/// let matches = search_with(contents, |line| line.contains("two"));
///
/// assert_eq!(vec![(2, "Line two")], matches);
/// ```
pub fn search_with<'a, F>(contents: &'a str, predicate: F) -> Vec<(usize, &'a str)>
where
    F: Fn(&str) -> bool,
{
    contents
        .lines()
        .enumerate()
        .filter(|&(_, line)| predicate(line))
        .map(|(index, line)| (index + 1, line)) // Convert to 1-indexed line number
        .collect()
}

/// Searches for lines containing the query string (case-sensitive)
///
/// # Arguments
///
/// * `query` - The string to search for
/// * `contents` - The text to search in
///
/// # Returns
///
/// * `Vec<(usize, &str)>` - A vector of tuples containing line numbers (1-indexed) and matching lines
///
/// # Examples
///
/// ```
/// use minigrep::search::search;
///
/// let query = "duct";
/// let contents = "Rust:\nsafe, fast, productive.\nPick three.";
///
/// assert_eq!(vec![(2, "safe, fast, productive.")], search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    search_with(contents, |line| line.contains(query))
}

/// Searches for lines containing the query string (case-insensitive)
///
/// # Arguments
///
/// * `query` - The string to search for
/// * `contents` - The text to search in
///
/// # Returns
///
/// * `Vec<(usize, &str)>` - A vector of tuples containing line numbers (1-indexed) and matching lines
///
/// # Examples
///
/// ```
/// use minigrep::search::search_case_insensitive;
///
/// let query = "rUsT";
/// let contents = "Rust:\nsafe, fast, productive.\nTrust me.";
///
/// assert_eq!(
///     vec![(1, "Rust:"), (3, "Trust me.")],
///     search_case_insensitive(query, contents)
/// );
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let query_lower = query.to_lowercase();
    search_with(contents, |line| {
        line.to_lowercase().contains(&query_lower)
    })
}

/// Searches for lines matching the regular expression pattern (case-sensitive)
///
/// # Arguments
///
/// * `pattern` - The regular expression pattern to search for
/// * `contents` - The text to search in
///
/// # Returns
///
/// * `Result<Vec<(usize, &str)>, regex::Error>` - A Result containing either a vector of tuples with line numbers and matching lines or a regex error
///
/// # Examples
///
/// ```
/// use minigrep::search::search_regex;
///
/// let pattern = r"\w+, \w+";
/// let contents = "Rust:\nsafe, fast, productive.\nPick three.";
///
/// assert_eq!(vec![(2, "safe, fast, productive.")], search_regex(pattern, contents).unwrap());
/// ```
pub fn search_regex<'a>(pattern: &str, contents: &'a str) -> Result<Vec<(usize, &'a str)>, regex::Error> {
    let regex = Regex::new(pattern)?;
    Ok(search_with(contents, |line| regex.is_match(line)))
}

/// Searches for lines matching the regular expression pattern (case-insensitive)
///
/// # Arguments
///
/// * `pattern` - The regular expression pattern to search for
/// * `contents` - The text to search in
///
/// # Returns
///
/// * `Result<Vec<(usize, &str)>, regex::Error>` - A Result containing either a vector of tuples with line numbers and matching lines or a regex error
///
/// # Examples
///
/// ```
/// use minigrep::search::search_regex_case_insensitive;
///
/// let pattern = r"rust";
/// let contents = "Rust:\nsafe, fast, productive.\nTrust me.";
///
/// assert_eq!(
///     vec![(1, "Rust:"), (3, "Trust me.")],
///     search_regex_case_insensitive(pattern, contents).unwrap()
/// );
/// ```
pub fn search_regex_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Result<Vec<(usize, &'a str)>, regex::Error> {
    let regex = RegexBuilder::new(pattern)
        .case_insensitive(true)
        .build()?;
    
    Ok(search_with(contents, |line| regex.is_match(line)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec![(2, "safe, fast, productive.")], search(query, contents));
    }

    #[test]
    fn test_case_sensitive_no_match() {
        let query = "DUCT"; // Uppercase won't match in case-sensitive mode
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(Vec::<(usize, &str)>::new(), search(query, contents));
    }

    #[test]
    fn test_case_sensitive_multiple_matches() {
        let query = "the";
        let contents = "\
The quick brown fox
jumps over the lazy dog.
The end.";

        assert_eq!(vec![(2, "jumps over the lazy dog.")], search(query, contents));
    }

    #[test]
    fn test_case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![(1, "Rust:"), (4, "Trust me.")],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn test_case_insensitive_multiple_matches() {
        let query = "the";
        let contents = "\
The quick brown fox
jumps over the lazy dog.
The end.";

        assert_eq!(
            vec![(1, "The quick brown fox"), (2, "jumps over the lazy dog."), (3, "The end.")],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn test_empty_query() {
        let query = "";
        let contents = "Some content";

        // An empty query should match all lines
        assert_eq!(vec![(1, "Some content")], search(query, contents));
    }

    #[test]
    fn test_empty_contents() {
        let query = "test";
        let contents = "";

        // Empty contents should return an empty vector
        assert_eq!(Vec::<(usize, &str)>::new(), search(query, contents));
    }

    #[test]
    fn test_multiline_content() {
        let query = "line";
        let contents = "First line\nSecond line\nThird line";

        assert_eq!(
            vec![(1, "First line"), (2, "Second line"), (3, "Third line")],
            search(query, contents)
        );
    }

    #[test]
    fn test_special_characters() {
        let query = ".*";
        let contents = "Regex .* wildcards\nNormal text";

        assert_eq!(vec![(1, "Regex .* wildcards")], search(query, contents));
    }

    #[test]
    fn test_unicode_characters() {
        let query = "こんにちは";
        let contents = "Hello World\nこんにちは世界\nGoodbye";

        assert_eq!(vec![(2, "こんにちは世界")], search(query, contents));
    }

    #[test]
    fn test_regex_search() {
        let pattern = r"D\w+";  // Words starting with 'D'
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec![(4, "Duct tape.")],
            search_regex(pattern, contents).unwrap()
        );
    }

    #[test]
    fn test_regex_search_multiple_matches() {
        let pattern = r"over|fox";
        let contents = "\
Line one
The quick brown fox
jumps over the lazy dog.";

        // This should match any line containing "over" or "fox"
        let results = search_regex(pattern, contents).unwrap();
        assert!(results.contains(&(2, "The quick brown fox")));
        assert!(results.contains(&(3, "jumps over the lazy dog.")));
        assert_eq!(2, results.len());
    }

    #[test]
    fn test_regex_search_no_match() {
        let pattern = r"\d+";  // One or more digits
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            Vec::<(usize, &str)>::new(),
            search_regex(pattern, contents).unwrap()
        );
    }

    #[test]
    fn test_regex_case_insensitive() {
        let pattern = r"rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec![(1, "Rust:"), (4, "Trust me.")],
            search_regex_case_insensitive(pattern, contents).unwrap()
        );
    }

    #[test]
    fn test_regex_with_invalid_pattern() {
        let pattern = r"["; // Invalid regex pattern
        let contents = "Some content";

        assert!(search_regex(pattern, contents).is_err());
    }

    #[test]
    fn test_regex_with_empty_pattern() {
        let pattern = "";
        let contents = "Some content";

        // An empty pattern matches all lines
        assert_eq!(
            vec![(1, "Some content")],
            search_regex(pattern, contents).unwrap()
        );
    }
}
