use std::process::Command;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    println!("Running cargo test for poem_yaml_fixer...");

    let output = Command::new("cargo")
        .arg("test")
        .arg("--package")
        .arg("poem_yaml_fixer")
        .output()?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    if output.status.success() {
        println!("\nCargo test completed successfully.");
    } else {
        println!("\nCargo test failed with status: {}", output.status);
    }

    Ok(())
}