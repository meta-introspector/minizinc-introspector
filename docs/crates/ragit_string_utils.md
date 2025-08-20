# `ragit-string-utils` Crate Documentation

This crate provides utility functions for string manipulation, primarily focused on parsing key-value pairs from text.

## `parse_key_value_pairs` Function

*   **Description:** Parses a given string (`hay`) to extract key-value pairs based on predefined regular expression patterns. It supports various formats for key-value pairs, including `key=value`, `key="value"`, `key='value'`, and `key: value`.

*   **Signature:**
    ```rust
pub fn parse_key_value_pairs(hay: &str) -> Result<Vec<(&str, &str)>, Box<dyn std::error::Error>>
    ```

*   **Parameters:**
    *   `hay`: A string slice (`&str`) representing the input text to be parsed.

*   **Returns:**
    *   `Result<Vec<(&str, &str)>, Box<dyn std::error::Error>>`: On success, returns a `Vec` of tuples, where each tuple contains a `(&str, &str)` representing a key-value pair. On failure, returns a `Box<dyn std::error::Error>`.

*   **Error Handling:**
    *   Returns an error if the internal regular expressions fail to compile.
    *   Returns an error if a key or value capture group is not found for a matched pattern.

*   **Example Usage:**

    ```rust
    let hay = r#"\n    best_album=\"Blow Your Face Out\"\n    best_quote=\"\\\"then as it was, then again it will be\\\"\"\"\n    best_year=1973\n    best_simpsons_episode: HOMR\n    "#;
    let kvs = ragit_string_utils::parse_key_value_pairs(hay).unwrap();
    assert_eq!(kvs, vec![
        ("best_album", "Blow Your Face Out"),
        ("best_quote", "\"then as it was, then again it will be\""),
        ("best_year", "1973"),
        ("best_simpsons_episode", "HOMR"),
    ]);
    ```

## Internal Implementation Details

The function uses the `regex_automata` crate to define multiple regular expression patterns for matching different key-value pair formats. It iterates through the captures found by these regexes and extracts the `key` and `val` named capture groups.

```
