pub fn main() {
    println!("cargo:rustc-link-search=all=/usr/local/lib");
    println!("cargo:rustc-link-lib=static=probdist");
    println!("cargo:rustc-link-lib=static=mylib");
    println!("cargo:rustc-link-lib=static=testu01");
}