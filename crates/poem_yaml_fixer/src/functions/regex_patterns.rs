use lazy_static::lazy_static;
use regex::Regex;

// Define regex patterns as constants
pub const KEYWORDS_REGEX_PATTERN: &str = "keywords: (.*)";
pub const MALFORMED_MEME_REGEX_PATTERN: &str = r#"^- "([^"]+)" \(([^)]+)\)"#;
pub const UNQUOTED_COLON_REGEX_PATTERN: &str = "description: (.*:.*)";
pub const DOCUMENT_SEPARATOR_REGEX_PATTERN: &str = "^---$";

// New regex patterns from regex_patterns.toml
pub const OLD_MEME_REGEX_PATTERN: &str = r"^\s*-\s*(.*?)\s*\((.*?)\)";
pub const NEW_MEME_DESC_REGEX_PATTERN: &str = r"^\s*-\s*description:\s*(.*?)";
pub const NEW_MEME_TEMPLATE_REGEX_PATTERN: &str = r"^\s*template:\s*(.*?)";
pub const TITLE_REGEX_PATTERN_FROM_TOML: &str = r"^title:\s*(.*)";
pub const SUMMARY_REGEX_PATTERN: &str = r"^summary:\s*(.*)";
pub const KEYWORDS_REGEX_PATTERN_FROM_TOML: &str = r"^keywords:\s*(.*)";
pub const EMOJIS_REGEX_PATTERN: &str = r"^emojis:\s*(.*)";
pub const ART_GENERATOR_INSTRUCTIONS_REGEX_PATTERN: &str = r"^art_generator_instructions:\s*(.*)";
pub const POEM_BODY_START_REGEX_PATTERN: &str = r"^poem_body:\s*\|";


// Compile regexes once at runtime into static Regex instances
lazy_static! {
    pub static ref KEYWORDS_REGEX: Regex = Regex::new(KEYWORDS_REGEX_PATTERN).unwrap();
    pub static ref MALFORMED_MEME_REGEX: Regex = Regex::new(MALFORMED_MEME_REGEX_PATTERN).unwrap();
    pub static ref UNQUOTED_COLON_REGEX: Regex = Regex::new(UNQUOTED_COLON_REGEX_PATTERN).unwrap();
    pub static ref DOCUMENT_SEPARATOR_REGEX: Regex = Regex::new(DOCUMENT_SEPARATOR_REGEX_PATTERN).unwrap();

    // New compiled regexes from regex_patterns.toml
    pub static ref OLD_MEME_REGEX: Regex = Regex::new(OLD_MEME_REGEX_PATTERN).unwrap();
    pub static ref NEW_MEME_DESC_REGEX: Regex = Regex::new(NEW_MEME_DESC_REGEX_PATTERN).unwrap();
    pub static ref NEW_MEME_TEMPLATE_REGEX: Regex = Regex::new(NEW_MEME_TEMPLATE_REGEX_PATTERN).unwrap();
    pub static ref TITLE_REGEX: Regex = Regex::new(TITLE_REGEX_PATTERN_FROM_TOML).unwrap();
    pub static ref SUMMARY_REGEX: Regex = Regex::new(SUMMARY_REGEX_PATTERN).unwrap();
    pub static ref KEYWORDS_REGEX_FROM_TOML: Regex = Regex::new(KEYWORDS_REGEX_PATTERN_FROM_TOML).unwrap();
    pub static ref EMOJIS_REGEX: Regex = Regex::new(EMOJIS_REGEX_PATTERN).unwrap();
    pub static ref ART_GENERATOR_INSTRUCTIONS_REGEX: Regex = Regex::new(ART_GENERATOR_INSTRUCTIONS_REGEX_PATTERN).unwrap();
    pub static ref POEM_BODY_START_REGEX: Regex = Regex::new(POEM_BODY_START_REGEX_PATTERN).unwrap();
}