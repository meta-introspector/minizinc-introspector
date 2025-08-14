fn main() {
    println!("cargo:rustc-link-search=/data/data/com.termux/files/home/storage/github/libminizinc/tools/minizinc_c_wrapper");
    println!("cargo:rustc-link-lib=minizinc_c_wrapper");
}