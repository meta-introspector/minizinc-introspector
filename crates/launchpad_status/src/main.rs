use chrono::Local;

fn main() {
    let now = Local::now();
    println!("Hello world! Current time: {}", now.format("%Y-%m-%d %H:%M:%S"));
}