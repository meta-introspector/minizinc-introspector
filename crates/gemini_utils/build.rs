extern crate phf_codegen;

use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join("emojis.rs");
    let mut file = BufWriter::new(File::create(&path).unwrap());

    writeln!(&mut file, "pub static TEST_EMOJI: &'static str = \"\u{23ce}\";").unwrap();
}
