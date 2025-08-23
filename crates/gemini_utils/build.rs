use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

// This data is copied from emojicons/src/lib.rs and build.rs
// It's a subset for the emojis we need.
const EMOJI_DATA: &[(&str, &str)] = &[
    ("return", "⏎"),
    ("sparkles", "✨"),
    ("brick", "🧱"),
];

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("emojis.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    writeln!(&mut file, "static EMOJIS: phf::Map<&'static str, &'static str> = {};",
             phf_codegen::Map::new(EMOJI_DATA)
             .build()).unwrap();
}
