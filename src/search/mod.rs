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
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
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
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
}
