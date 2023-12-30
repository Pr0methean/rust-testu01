use std::env;
use std::env::VarError;

pub fn main() -> Result<(), VarError> {
    let os = env::var("CARGO_CFG_TARGET_OS")?;
    match os.as_str() {
        "macos" => {
            // WARNING: Tested on a Mac where testu01 was built from source.
            println!("cargo:rustc-link-search=all=/usr/local/lib/");
            println!("cargo:rustc-link-lib=static=probdist");
            println!("cargo:rustc-link-lib=static=mylib");
            println!("cargo:rustc-link-lib=static=testu01");
        }
        "linux" => {
            // WARNING: Only tested on Ubuntu
            println!("cargo:rustc-link-search=all=/usr/lib/x86_64-linux-gnu/");
            println!("cargo:rustc-link-lib=static=testu01probdist");
            println!("cargo:rustc-link-lib=static=testu01mylib");
            println!("cargo:rustc-link-lib=static=testu01");
        }
        _ => panic!("Don't know how to link testu01 statically on this platform")
    }
    Ok(())
}