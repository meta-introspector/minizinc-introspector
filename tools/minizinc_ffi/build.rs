fn main() {
    // Tell cargo to link the `minizinc_c_wrapper` library with its absolute path
    println!("cargo:rustc-link-arg=/data/data/com.termux/files/home/storage/github/libminizinc/build/libminizinc_c_wrapper.so");
}