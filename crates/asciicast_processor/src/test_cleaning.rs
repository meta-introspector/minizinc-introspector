//use std::fs;
use std::io::{self,
	      //Read
};
//use serde_yaml;

// Define a struct to hold the extracted message (optional, for future extensibility)
#[derive(serde::Deserialize, Debug)]
struct Message {
    content: String,
}

// Function to separate content (printable characters) from background (non-printable)
fn separate_content_from_background(bytes: &[u8]) -> (String, Vec<u8>) {
    let mut content = String::new();
    let mut background = Vec::new();

    for &byte in bytes {
        if byte.is_ascii_graphic() || byte == b'\n' || byte == b' ' || byte == b'\t' {
            // Printable ASCII characters, newline, space, or tab are considered content
            content.push(byte as char);
        } else {
            // Non-printable bytes (control characters, extended ASCII, etc.) are background
            background.push(byte);
        }
    }

    (content, background)
}

// Function to extract and clean the message
fn extract_message(content: String) -> String {
    // Simple cleaning: remove leading/trailing whitespace and join lines
    content
        .lines()
        .filter(|line| !line.trim().is_empty()) // Remove empty lines
        .collect::<Vec<&str>>()
        .join(" ")
        .trim()
        .to_string()
}

// Function to remove non-ASCII characters
fn clean_non_ascii(input: &str) -> String {
    input.chars().filter(|c| c.is_ascii()).collect()
}

// Main function to process bytes from a file or hardcoded input
fn main() -> io::Result<()> {
    // Example: Hardcode the byte sequence you provided for testing
    let input_bytes: Vec<u8> = vec![
        226, 160, 153, 32, 73, 39, 108, 108, 32, 99, 114, 101, 97, 116, 101, 32, 97, 32, 109, 105,
        110, 105, 109, 97, 108, 32, 82, 117, 115, 116, 32, 112, 111, 114, 116, 32, 111, 102, 32, 97,
        99, 116, 32, 116, 111, 32, 114, 117, 110, 32, 111, 117, 114, 32, 71, 105, 116, 72, 117, 98,
        32, 65, 99, 116, 105, 111, 110, 115, 32, 119, 111, 114, 107, 102, 108, 111, 119, 32, 108,
        111, 99, 97, 108, 108, 121, 44, 32, 97, 118, 111, 105, 100, 105, 110, 103, 32, 71, 111, 46,
        32, 70, 105, 114, 115, 116, 44, 10, 32, 32, 73, 39, 108, 108, 32, 97, 110, 97, 108, 121,
        122, 101, 32, 97, 99, 116, 39, 115, 32, 109, 97, 105, 110, 46, 103, 111, 32, 116, 111, 32,
        117, 110, 100, 101, 114, 115, 116, 97, 110, 100, 32, 105, 116, 115, 32, 99, 111, 114, 101,
        32, 108, 111, 103, 105, 99, 46, 32, 84, 104, 101, 110, 44, 32, 73, 39, 108, 108, 32, 99,
        114, 101, 97, 116, 101, 32, 97, 32, 110, 101, 119, 32, 82, 117, 115, 116, 32, 112, 114, 111,
        106, 101, 99, 116, 32, 110, 97, 109, 101, 100, 10, 32, 32, 109, 105, 110, 105, 45, 97, 99,
        99, 116, 32, 105, 110, 32, 116, 104, 101, 32, 99, 114, 97, 116, 101, 115, 32, 100, 105, 114, 115,
        101, 99, 116, 111, 114, 121, 46, 32, 73, 39, 108, 108, 32, 105, 109, 112, 108, 101, 109, 101, 110, 116, 32, 97, 32, 89, 65, 77, 76, 32, 112, 97, 114, 115, 101, 114, 32, 117, 115, 105, 110, 103, 32, 115, 101, 114, 100, 101, 95, 121, 97, 109, 108, 32, 97, 110, 100, 32, 97, 32, 115, 105, 109, 112, 108, 101, 32, 114, 117, 110, 110, 101, 114, 32, 116, 111, 10, 32, 32, 101, 120, 101, 99, 117, 116, 101, 32, 116, 104, 101, 32, 99, 104, 101, 99, 107, 95, 97, 110, 100, 95, 98, 117, 105, 108, 100, 32, 106, 111, 98, 39, 115, 32, 115, 116, 101, 112, 115, 32, 115, 101, 113, 117, 101, 110, 116, 105, 97, 108, 108, 121, 46, 32, 68, 111, 99, 107, 101, 114, 32, 105, 110, 116, 101, 103, 114, 97, 116, 105, 111, 110, 32, 105, 115, 32, 100, 101, 102, 101, 114, 114, 101, 100, 32, 97, 115, 32, 105, 116, 39, 115, 32, 110, 111, 116, 10, 32, 32, 99, 117, 114, 114, 101, 110, 116, 108, 121, 32, 110, 101, 101, 100, 101, 100, 46, 32, 73, 39, 108, 108, 32, 98, 101, 103, 105, 110, 32, 98, 121, 32, 101, 120, 97, 109, 105, 110, 105, 110, 103, 32, 109, 97, 105, 110, 46, 103, 111, 46, 10, 226, 160, 153, 32, 68, 105, 115, 115, 101, 99, 116, 105, 110, 103, 32, 96, 97, 99, 116, 96, 39, 115, 32, 65, 114, 99, 104, 105, 116, 101, 99, 116, 117, 114, 101, 32, 40, 101, 115, 99, 32, 116, 111, 32, 99, 97, 110, 99, 101, 108, 44, 32, 55, 115, 41, 10, 10, 85, 115, 105, 110, 103, 58, 32, 50, 32, 71, 69, 77, 73, 78, 73, 46, 109, 100, 32, 102, 105, 108, 101, 115, 10, 126, 47, 115, 116, 111, 114, 97, 103, 101, 47, 103, 105, 116, 104, 117, 98, 47, 108, 105, 98, 109, 105, 110, 105, 122, 105, 110, 99, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 32, 110, 111, 32, 115, 97, 110, 100, 98, 111, 120, 32, 40, 115, 101, 101, 32, 32, 32, 103, 101, 109, 105, 110, 105, 45, 50, 46, 53, 45, 112, 114, 111, 32, 40, 57, 55, 37, 32, 32, 32, 32, 32, 124, 32, 226, 156, 150, 32
    ];

    let (content, background) = separate_content_from_background(&input_bytes);

    println!("Original Bytes: {:?}", input_bytes);
    println!("Separated Content: {:?}", content);
    println!("Separated Background: {:?}", background);

    let cleaned_message = extract_message(clean_non_ascii(&content)); // Apply clean_non_ascii here
    println!("Cleaned Message: {:?}", cleaned_message);

    Ok(())
}
