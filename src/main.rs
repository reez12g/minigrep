use std::env;
use std::process;

use minigrep::config::Config;

/// The main entry point for the minigrep application
fn main() {
    // Parse command line arguments
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        eprintln!("Usage: minigrep <query> <filename>");
        process::exit(1);
    });

    // Display search parameters
    println!("Searching for '{}' in '{}'", config.query, config.filename);
    println!("Case sensitive: {}", config.case_sensitive);

    // Run the application
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
