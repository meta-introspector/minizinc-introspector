use gemini_utils::gemini_eprintln;

pub fn print_header(message: &str) {
    gemini_eprintln!("-- HEADER --");
}

pub fn print_footer() {
    gemini_eprintln!("---------------------------------------------------------");
}

pub fn print_success(message: &str) {
    gemini_eprintln!("-- SUCCESS --");
}
