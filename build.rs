fn main() {
    println!("cargo:rustc-link-arg=-L $NIX_LD_LIBRARY_PATH");
    // println!("cargo:rustc-link-arg=-L ./");
    // println!("cargo:rustc-link-lib=static=tinyalsa");
}
