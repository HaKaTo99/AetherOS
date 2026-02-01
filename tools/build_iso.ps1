# Build Bootable ISO for AetherOS (x86_64)
# Requires: cargo install bootimage

$KernelPath = "kernel"
$Target = "x86_64-unknown-none"

Write-Host "Building Kernel..."
cargo build --manifest-path "$KernelPath/Cargo.toml" --release --target $Target

if ($LASTEXITCODE -ne 0) {
    Write-Error "Kernel build failed!"
    exit 1
}

Write-Host "Creating Bootable Disk Image..."
# Assuming bootimage is installed
# If not: cargo install bootimage
cargo bootimage --manifest-path "$KernelPath/Cargo.toml" --release --target $Target

if ($LASTEXITCODE -ne 0) {
    Write-Error "Bootimage generation failed. Ensure 'cargo install bootimage' is run."
    exit 1
}

Write-Host "Success! Image available in target/x86_64-unknown-none/release/bootimage-aetheros-kernel.bin"
