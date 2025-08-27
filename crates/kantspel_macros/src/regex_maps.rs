use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref REGEX_EMOJIS: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        // General
        map.insert("💨", r"\u{1b}"); // Escape character
        map.insert("🤸", r"\s"); // Whitespace
        map.insert("🔢", r"\d"); // Digit
        map.insert("✍️", r"\w"); // Word character
        map.insert(".", ".");   // Any character
        map.insert("❓", ".");   // Any character (alternative)

        // Quantifiers
        map.insert("*", "*");   // Zero or more
        map.insert("+", "+");   // One or more
        map.insert("?", "?");   // Zero or one

        // Anchors
        map.insert("^", "^");   // Start of string
        map.insert("$", "$");   // End of string

        // Grouping and Alternation
        map.insert("(", "(");
        map.insert(")", ")");
        map.insert("[", "[");
        map.insert("]", "]");
        map.insert("{ ", "{ ");
        map.insert("} ", "} ");
        map.insert("|", "|");

        // Escaped characters
        map.insert(r"\\", r"\\"); // Literal backslash
        map.insert(r"\.", r"\.");
        map.insert(r"\*", r"\*");
        map.insert(r"\+", r"\+");
        map.insert(r"\?", r"\?");
        map.insert(r"\(", r"\");
        map.insert(r"\)", r"\");
        map.insert(r"\[", r"\");
        map.insert(r"\]", r"\");
        map.insert(r"\{{", r"\");
        map.insert(r"\}}", r"\");
        map.insert(r"\^", r"\");
        map.insert(r"\$", r"\");
        map.insert(r"\|", r"\");

        map
    };
    pub static ref REGEX_ALIASES: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert("digit", r"\d");
        map.insert("word", r"\w");
        map.insert("space", r"\s");
        map
    };
}
