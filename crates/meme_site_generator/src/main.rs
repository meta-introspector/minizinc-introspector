use std::{fs, path::PathBuf};
use serde::{Deserialize, Serialize};
use anyhow::{Result, anyhow};
use walkdir::WalkDir;
use tera::{Tera, Context};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PoemFrontMatter {
    title: String,
    summary: String,
    keywords: String,
    emojis: String,
    art_generator_instructions: String,
    memes: Vec<String>,
    poem_body: Option<String>, // Now guaranteed to be Some(String) after poem_formatter
}

fn main() -> Result<()> {
    let poems_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/docs/poems");
    let site_output_dir = PathBuf::from("/data/data/com.termux/files/home/storage/github/libminizinc/site");

    // Create output directory if it doesn't exist
    fs::create_dir_all(&site_output_dir)?;

    // Initialize Tera (templating engine)
    // We'll need to define our templates later, for now, let's assume basic ones
    let mut tera = Tera::default();
    tera.add_raw_template("poem.html", r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>{{ poem.title }}</title>
            <meta name="description" content="{{ poem.summary }}">
            <meta name="keywords" content="{{ poem.keywords }}">
            <meta name="emojis" content="{{ poem.emojis }}">
        </head>
        <body>
            <h1>{{ poem.title }}</h1>
            <p>{{ poem.summary }}</p>
            <pre>{{ poem.poem_body }}</pre>
            <h2>Memes:</h2>
            <ul>
                {% for meme in poem.memes %}
                <li>{{ meme }}</li>
                {% endfor %}
            </ul>
        </body>
        </html>
    "#)?;
    tera.add_raw_template("index.html", r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Poem Index</title>
        </head>
        <body>
            <h1>Poem Index</h1>
            <ul>
                {% for poem in poems %}
                <li><a href="{{ poem.filename | replace(from=".md", to=".html") }}">{{ poem.title }}</a></li>
                {% endfor %}
            </ul>
        </body>
        </html>
    "#)?;


    let mut all_poems = Vec::new();

    for entry in WalkDir::new(&poems_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            let filename = path.file_name().unwrap().to_str().unwrap().to_string();
            if filename == "index.md" {
                continue; // Skip index.md for individual poem pages
            }

            println!("Generating page for: {path:?}");
            let content = fs::read_to_string(path)?;
            let parts: Vec<&str> = content.split("---").collect();

            if parts.len() < 3 {
                return Err(anyhow!("Invalid Markdown file format: {:?}", path));
            }

            let front_matter_str = parts[1].trim();
            let poem_data: PoemFrontMatter = serde_yaml::from_str(front_matter_str)?;

            let mut context = Context::new();
            context.insert("poem", &poem_data);

            let rendered = tera.render("poem.html", &context)?;
            let output_path = site_output_dir.join(filename.replace(".md", ".html"));
            fs::write(&output_path, rendered)?;

            all_poems.push(poem_data);
        }
    }

    // Generate index.html
    let mut context = Context::new();
    context.insert("poems", &all_poems);
    let rendered_index = tera.render("index.html", &context)?;
    fs::write(site_output_dir.join("index.html"), rendered_index)?;

    println!("Static site generated successfully in {site_output_dir:?}");

    Ok(())
}