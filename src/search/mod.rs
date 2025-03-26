/// Searches for lines in contents that match a predicate function
///
/// # Arguments
///
/// * `contents` - The text to search in
/// * `predicate` - A function that takes a line and returns true if it matches
///
/// # Returns
///
/// * `Vec<&str>` - A vector of lines that match the predicate
///
/// # Examples
///
/// ```
/// use minigrep::search::search_with;
///
/// let contents = "Line one\nLine two\nLine three";
/// let matches = search_with(contents, |line| line.contains("two"));
///
/// assert_eq!(vec!["Line two"], matches);
/// ```
pub fn search_with<'a, F>(contents: &'a str, predicate: F) -> Vec<&'a str>
where
    F: Fn(&str) -> bool,
{
    contents.lines().filter(|&line| predicate(line)).collect()
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
/// * `Vec<&str>` - A vector of lines that contain the query
///
/// # Examples
///
/// ```
/// use minigrep::search::search;
///
/// let query = "duct";
/// let contents = "Rust:\nsafe, fast, productive.\nPick three.";
///
/// assert_eq!(vec!["safe, fast, productive."], search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
/// * `Vec<&str>` - A vector of lines that contain the query (ignoring case)
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
///     vec!["Rust:", "Trust me."],
///     search_case_insensitive(query, contents)
/// );
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lower = query.to_lowercase();
    search_with(contents, |line| {
        line.to_lowercase().contains(&query_lower)
    })
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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_case_sensitive_no_match() {
        let query = "DUCT"; // Uppercase won't match in case-sensitive mode
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn test_case_sensitive_multiple_matches() {
        let query = "the";
        let contents = "\
The quick brown fox
jumps over the lazy dog.
The end.";

        assert_eq!(vec!["jumps over the lazy dog."], search(query, contents));
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
            vec!["Rust:", "Trust me."],
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
            vec!["The quick brown fox", "jumps over the lazy dog.", "The end."],
            search_case_insensitive(query, contents)
        );
    }

    #[test]
    fn test_empty_query() {
        let query = "";
        let contents = "Some content";

        // An empty query should match all lines
        assert_eq!(vec!["Some content"], search(query, contents));
    }

    #[test]
    fn test_empty_contents() {
        let query = "test";
        let contents = "";

        // Empty contents should return an empty vector
        assert_eq!(Vec::<&str>::new(), search(query, contents));
    }

    #[test]
    fn test_multiline_content() {
        let query = "line";
        let contents = "First line\nSecond line\nThird line";

        assert_eq!(
            vec!["First line", "Second line", "Third line"],
            search(query, contents)
        );
    }

    #[test]
    fn test_special_characters() {
        let query = ".*";
        let contents = "Regex .* wildcards\nNormal text";

        assert_eq!(vec!["Regex .* wildcards"], search(query, contents));
    }

    #[test]
    fn test_unicode_characters() {
        let query = "こんにちは";
        let contents = "Hello World\nこんにちは世界\nGoodbye";

        assert_eq!(vec!["こんにちは世界"], search(query, contents));
    }
}
