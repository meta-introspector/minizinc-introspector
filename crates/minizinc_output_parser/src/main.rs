use std::io::{self, Read};
use regex::Regex;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let embeddings_regex = Regex::new(r"embeddings = array2d\(1..(\d+), 1..8, \[(.*?)\]\);").unwrap();

    if let Some(captures) = embeddings_regex.captures(&buffer) {
        let num_words_str = captures.get(1).unwrap().as_str();
        let values_str = captures.get(2).unwrap().as_str();

        let num_words: usize = num_words_str.parse().unwrap();
        let values: Vec<f64> = values_str
            .split(',')
            .map(|s| s.trim().parse().unwrap())
            .collect();

        // Output in .dzn format
        println!("num_fixed_words = {};", num_words);
        print!("fixed_word_indices = [");
        for i in 1..=num_words {
            print!("{}", i);
            if i < num_words {
                print!(", ");
            }
        }
        println!("];");

        print!("fixed_word_embeddings = array2d(1..{}, 1..8, [", num_words);
        for (i, val) in values.iter().enumerate() {
            print!("{}", val);
            if i < values.len() - 1 {
                print!(", ");
            }
        }
        println!("]);");
    } else {
        eprintln!("Could not find 'embeddings' in the MiniZinc output.");
    }

    Ok(())
}
