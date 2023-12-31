pub fn main() {
    // Tested on MacOS & Ubuntu with testu01 built from source. Dependency names are different when
    // installing it from the Ubuntu repos (testu01probdist and testu01mylib).
    println!("cargo:rustc-link-search=all=/usr/local/lib/");
    println!("cargo:rustc-link-lib=static=testu01");
}