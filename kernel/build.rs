use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    let link_script = if arch == "x86_64" {
        PathBuf::from(manifest_dir).join("src/arch/x86_64/linker.ld")
    } else {
        // Default to aarch64 / RPi4
        PathBuf::from(manifest_dir).join("link.ld")
    };

    // Tell cargo to search directory containing link script
    if let Some(parent) = link_script.parent() {
        println!("cargo:rustc-link-search={}", parent.display());
    }
    
    // Tell cargo to rerun if link.ld changes
    println!("cargo:rerun-if-changed={}", link_script.display());
    println!("cargo:rustc-link-arg=-T{}", link_script.file_name().unwrap().to_str().unwrap());
}
