fn main() {
    println!("cargo:rustc-link-search=/data/data/com.termux/files/home/storage/github/libminizinc/tools/minizinc_c_wrapper");
    println!("cargo:rustc-link-search=/data/data/com.termux/files/home/storage/github/libminizinc/install/lib");
    println!("cargo:rustc-link-lib=minizinc_c_wrapper");
    println!("cargo:rustc-link-lib=mzn"); // Assuming the library name is 'mzn'
    // Set rpath for runtime linking
    println!("cargo:rustc-link-arg=-Wl,-rpath,/data/data/com.termux/files/home/storage/github/libminizinc/tools/minizinc_c_wrapper");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/data/data/com.termux/files/home/storage/github/libminizinc/install/lib");
}