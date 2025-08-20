pub fn sanitize_filename_char(c: char) -> String {
    if c.is_ascii_alphanumeric() {
        c.to_string()
    } else {
        format!("U{:04X}", c as u32) // Format as UXXXX (hex Unicode codepoint)
    }
}

pub fn sanitize_filename(s: &str) -> String {
    s.chars().flat_map(|c| sanitize_filename_char(c).chars()).collect()
}