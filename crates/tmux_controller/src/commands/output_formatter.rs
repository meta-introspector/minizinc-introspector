pub fn print_header(message: &str) {
    println!("-- {} --", message);
}

pub fn print_footer(message: &str) {
    println!("---------------------------------------------------------\
");
}

pub fn print_info(message: &str) {
    println!("{}", message);
}

pub fn print_success(message: &str) {
    println!("-- {} --\
", message);
}

pub fn print_error(message: &str) {
    eprintln!("Error: {}\
", message);
}

