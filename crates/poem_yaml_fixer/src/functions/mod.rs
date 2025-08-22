pub mod extract_front_matter;
pub mod parse_front_matter_fields;
pub mod process_memes_with_workflow;
pub mod extract_words_from_text;
pub mod save_word_index;
pub mod process_poem_file;

pub mod types;
pub mod error_handling;

pub mod callbacks; // Add this line
pub mod parse_poem_file_direct;
pub mod test_save_word_index;
pub mod test_process_memes_with_workflow;
pub mod test_handle_unmatched_regex_error;
pub mod process_single_poem_file_for_report;
pub mod archeology_parser;
//pub mod parse_front_matter_with_regex;
pub mod utils; // New line
pub mod report_generator;
pub mod regex_patterns;
pub mod load_regex_config;
pub mod report_processing;
pub mod process_document_with_regex;
pub mod report_printer;
