use clang::*;
use std::path::Path;

fn main() {
    // Replace with the actual path to a MiniZinc C++ source file
    let cpp_file_path = Path::new("/data/data/com.termux/files/home/storage/github/libminizinc/lib/parser.cpp");

    // Create a new Clang instance
    let clang = Clang::new().unwrap();

    // Create a new Clang index
    let index = Index::new(&clang, false, true);

    // Common include paths for MiniZinc
    let include_paths = [
        "-I/data/data/com.termux/files/home/storage/github/libminizinc/include",
        "-I/data/data/com.termux/files/home/storage/github/libminizinc/lib", // Might contain some headers
        "-I/data/data/com.termux/files/usr/include/c++/v1",
        "-I/data/data/com.termux/files/usr/include",
    ];

    // Common C++ standard and other flags
    let compiler_args = [
        "-std=c++17", // Or c++11, c++14, c++20 depending on MiniZinc's codebase
        "-x", "c++", // Treat input as C++
    ];

    let mut all_args: Vec<&str> = Vec::new();
    all_args.extend_from_slice(&include_paths);
    all_args.extend_from_slice(&compiler_args);

    let mut parser = index.parser(cpp_file_path);
    parser.arguments(&all_args);

    // Parse the C++ file
    let tu = match parser.parse() {
        Ok(tu) => tu,
        Err(e) => {
            eprintln!("Error parsing C++ file {:?}: {:?}", cpp_file_path, e);
            // Attempt to get diagnostics for more info
            // Note: Diagnostics are usually part of the TranslationUnit, even on error.
            // We need to get them from the returned TranslationUnitError if available.
            // For now, we'll just print the error.
            return;
        }
    };

    println!("Successfully parsed C++ file: {}", cpp_file_path.display());
    println!("Functions found:");

    // Iterate over the AST to find function declarations
    tu.get_entity().visit_children(|entity, _| {
        if entity.get_kind() == EntityKind::FunctionDecl {
            if let Some(name) = entity.get_display_name() {
                println!("  - {}", name);
            }
        }
        EntityVisitResult::Continue // Continue visiting children
    });
}