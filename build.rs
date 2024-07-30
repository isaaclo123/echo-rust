fn main() {
    println!("cargo:rustc-link-arg=-L $NIX_LD_LIBRARY_PATH");
}
