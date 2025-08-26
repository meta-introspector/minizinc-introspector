use gemini_utils::gemini_eprintln;

#[test]
fn test_kantspel_characters() {
    // Test literal backslash and curly braces, which should be handled by kantspel
    gemini_eprintln!("Path: C:\\Users\\Test\\File.txt. Data: {{ \"key\": \"value\" }}");
    gemini_eprintln!("This is a backslash: \\ and an open curly brace: {{ and a close curly brace: }}");
}

