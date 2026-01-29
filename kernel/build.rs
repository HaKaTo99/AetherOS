use std::env;
use std::path::PathBuf;

fn main() {
    // Set the linker script to use
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let link_script = PathBuf::from(manifest_dir).join("link.ld");

    // Tell cargo to search the current directory for the linker script
    println!("cargo:rustc-link-search={}", env::current_dir().unwrap().display());
    
    // Tell cargo to rerun if link.ld changes
    println!("cargo:rerun-if-changed={}", link_script.display());
    println!("cargo:rerun-if-changed=link.ld");
}
