use lazy_static::lazy_static;
use regex::Regex;

// Define regex patterns as constants
pub const KEYWORDS_REGEX_PATTERN: &str = "keywords: (.*)";
pub const MALFORMED_MEME_REGEX_PATTERN: &str = r#"^- "([^"]+)" \(([^)]+)\)"#;
pub const UNQUOTED_COLON_REGEX_PATTERN: &str = "description: (.*:.*)";
pub const DOCUMENT_SEPARATOR_REGEX_PATTERN: &str = "^---$";

// Compile regexes once at runtime into static Regex instances
lazy_static! {
    pub static ref KEYWORDS_REGEX: Regex = Regex::new(KEYWORDS_REGEX_PATTERN).unwrap();
    pub static ref MALFORMED_MEME_REGEX: Regex = Regex::new(MALFORMED_MEME_REGEX_PATTERN).unwrap();
    pub static ref UNQUOTED_COLON_REGEX: Regex = Regex::new(UNQUOTED_COLON_REGEX_PATTERN).unwrap();
    pub static ref DOCUMENT_SEPARATOR_REGEX: Regex = Regex::new(DOCUMENT_SEPARATOR_REGEX_PATTERN).unwrap();
}
