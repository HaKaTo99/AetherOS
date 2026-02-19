use std::path::PathBuf;
use std::process::Command;

fn main() {
    let kernel_path = build_kernel();
    let bootloader_path = build_bootloader();

    let mut cmd = Command::new("qemu-system-x86_64");
    cmd.arg("-drive").arg(format!("format=raw,file={}", create_boot_image(&kernel_path, &bootloader_path).display()));
    let status = cmd.status().unwrap();
    if !status.success() {
        panic!("qemu failed with status: {}", status);
    }
}

fn build_kernel() -> PathBuf {
    let kernel_path = PathBuf::from("../kernel");
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&kernel_path);
    cmd.arg("build").arg("--release");
    let status = cmd.status().unwrap();
    if !status.success() {
        panic!("kernel build failed");
    }
    kernel_path.join("target/x86_64-unknown-none/release/aetheros-kernel")
}

fn build_bootloader() -> PathBuf {
    let bootloader_path = PathBuf::from("bootloader");
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&bootloader_path);
    cmd.arg("build").arg("--release");
    let status = cmd.status().unwrap();
    if !status.success() {
        panic!("bootloader build failed");
    }
    bootloader_path.join("target/x86_64-bootloader/release/bootloader")
}

fn create_boot_image(kernel_path: &PathBuf, bootloader_path: &PathBuf) -> PathBuf {
    let bootloader_elf = std::fs::read(bootloader_path).unwrap();
    let kernel_elf = std::fs::read(kernel_path).unwrap();
    let mut boot_image = bootloader::BootImage::new(&bootloader_elf, &kernel_elf).unwrap();
    boot_image.set_boot_config(&bootloader::BootConfig::default());
    let path = PathBuf::from("boot.img");
    boot_image.write_to_disk(&path).unwrap();
    path
}
