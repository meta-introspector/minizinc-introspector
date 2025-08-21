use std::{fs
	  //, path::PathBuf
};
use anyhow::{Result,
	     //anyhow
};
use walkdir::WalkDir;

use poem_meme_formatter::parser;

fn main() -> Result<()> {
    let current_dir = std::env::current_dir()?;
    let poems_dir = current_dir.join("docs").join("poems");

    for entry in WalkDir::new(&poems_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
            // Skip index.md for this specific processing
            if path.file_name().map_or(false, |name| name == "index.md") {
                continue;
            }

            println!("Processing: {:?}\n", path);
            let content = fs::read_to_string(path)?;
            match parser::parse_poem_document(&content) {
                Ok(poem_doc) => {
                    println!("Successfully parsed: {}\n", poem_doc.title);
                    // Here you would typically re-serialize the poem_doc back to a file
                    // or use it to generate a static site.
                    // For now, just print for verification.
                    // println!("{:?}\n", poem_doc);
                },
                Err(e) => eprintln!("Error parsing {:?}: {}\n", path, e),
            }
        }
    }

    Ok(()) // Explicitly return Ok(())
}
