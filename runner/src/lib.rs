use std::path::Path;
use bootloader::{BootConfig, DiskImageBuilder};

pub fn run() {
    let kernel_binary = Path::new(env!("CARGO_BIN_FILE_AETHEROS_KERNEL"));
    let boot_config = BootConfig::default();

    let disk_image_builder = DiskImageBuilder::new(kernel_binary.to_path_buf());
    let disk_image = disk_image_builder.create_disk_image(&boot_config).unwrap();

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    cmd.arg("-drive").arg(format!("format=raw,file={}", disk_image.disk_image_path().display()));
    let exit_status = cmd.status().unwrap();
    if !exit_status.success() {
        std::process::exit(exit_status.code().unwrap_or(1));
    }
}
