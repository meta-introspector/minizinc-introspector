pub fn format_pair(w1:&String, w2:&String) -> String {
    return format!("\"{}\" , \"{}\"", w1, w2)
}

pub fn format_triple(w1:&String, w2:&String, w3:&String) -> String {
    return format!("\"{}\",\"{}\",\"{}\"", w1, w2, w3)
}