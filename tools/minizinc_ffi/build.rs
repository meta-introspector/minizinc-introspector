fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=native=/data/data/com.termux/files/home/storage/github/libminizinc/build");

    // Tell cargo to link the `minizinc_c_wrapper` library
    println!("cargo:rustc-link-lib=minizinc_c_wrapper");

    // Add rpath for Android to find the shared library at runtime
    #[cfg(target_os = "android")]
    println!("cargo:rustc-link-arg=-Wl,-rpath=/data/data/com.termux/files/home/storage/github/libminizinc/build");
}
