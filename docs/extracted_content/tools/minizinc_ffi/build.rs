extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("wrapper.h")
        // Tell cargo to rerun this build script if the generated bindings are changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Add MiniZinc include path
        .clang_arg("-I/data/data/com.termux/files/home/storage/github/libminizinc/include")
        .clang_arg("-I/data/data/com.termux/files/home/storage/github/libminizinc/build/include")
        .clang_arg("-I/data/data/com.termux/files/usr/include/c++/v1")
        .clang_arg("-I/data/data/com.termux/files/usr/include")
        .clang_arg("-std=c++17")
        .clang_arg("-x c++")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Tell cargo to link against libmzn.so
    println!("cargo:rustc-link-search=/data/data/com.termux/files/home/storage/github/libminizinc/build");
    println!("cargo:rustc-link-lib=mzn");
}