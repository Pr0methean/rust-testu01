pub fn main() {
    println!("cargo:rustc-link-search=all=/usr/lib/x86_64-linux-gnu/");
    println!("cargo:rustc-link-lib=static=testu01probdist");
    println!("cargo:rustc-link-lib=static=testu01mylib");
    println!("cargo:rustc-link-lib=static=testu01");
}