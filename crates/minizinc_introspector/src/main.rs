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
    let all_args = [
        // MiniZinc specific includes
        "-I/data/data/com.termux/files/home/storage/github/libminizinc/include",
        "-I/data/data/com.termux/files/home/storage/github/libminizinc/lib",

        // System includes from clang++ -E -v
        "-I/data/data/com.termux/files/usr/include/c++/v1",
        "-I/data/data/com.termux/files/usr/lib/clang/20/include",
        "-I/data/data/com.termux/files/usr/include/aarch64-linux-android",
        "-I/data/data/com.termux/files/usr/include",
        // Ignoring nonexistent: "-I/data/data/com.termux/files/usr/local/include",
        // Ignoring nonexistent: "-I/data/data/com.termux/files/include",

        // Compiler arguments from clang++ -E -v
        "-std=c++17", // Keep this, as it's a common standard
        "-x", "c++",
        "-fdeprecated-macro",
        "-ferror-limit", "19", // Note: this might stop parsing early if too many errors
        "-fno-signed-char",
        "-fgnuc-version=4.2.1",
        "-fskip-odr-check-in-gmf",
        "-fcxx-exceptions",
        "-fexceptions",
        "-target-feature", "+outline-atomics",
        "-D__GCC_HAVE_DWARF2_CFI_ASM=1",
        "-Xclang", "-fno-pch-reuse",
    ];

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