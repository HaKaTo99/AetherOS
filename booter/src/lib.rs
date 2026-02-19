pub fn boot() {
    let kernel_path = Path::new(env!("CARGO_BIN_FILE_AETHEROS_KERNEL"));
    let mut builder = DiskImageBuilder::new(kernel_path.to_path_buf());
    
    let out_dir = Path::new("target").join("aetheros_boot");
    std::fs::create_dir_all(&out_dir).unwrap();
    let out_path = out_dir.join("aetheros.img");
    
    println!("[Booter] Creating bootable BIOS image: {:?}", out_path);
    builder.create_bios_image(&out_path).unwrap();

    // 3. Launch QEMU
    println!("[Booter] Launching AetherOS v10.0 in QEMU...");
    let mut qemu = Command::new("qemu-system-x86_64");
    qemu.arg("-drive").arg(format!("format=raw,file={}", out_path.display()));
    qemu.arg("-m").arg("512M");
    qemu.arg("-serial").arg("stdio");
    qemu.arg("-display").arg("none"); // Headless for terminal output

    let mut child = qemu.spawn().expect("failed to launch QEMU");
    child.wait().expect("QEMU failed");
}
