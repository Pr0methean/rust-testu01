pub fn main() {
    println!("cargo:rustc-link-lib=static=testu01");
    println!("cargo:rustc-link-search=native=/usr/local/lib");
}