use gemini_utils::gemini_eprintln;

pub fn print_success(message: &str) {
    gemini_eprintln!("-- :message: --::sparkles::", message = message);
}

pub fn print_footer() {
    gemini_eprintln!("---------------------------------------------------------");
}

pub fn print_info(message: &str) {
    gemini_eprintln!("FIXME")
}
