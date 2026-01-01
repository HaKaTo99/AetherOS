# AetherOS Build System
# Builds kernel, compiler, and creates bootable images

.PHONY: all kernel compiler examples clean test qemu raspberry-pi docker help

# Default target
all: kernel compiler

# Help target
help:
	@echo "AetherOS Build System v1.0"
	@echo ""
	@echo "Targets:"
	@echo "  all          - Build kernel and compiler (default)"
	@echo "  kernel       - Build Quantum Microkernel"
	@echo "  compiler     - Build AetherScript Compiler"
	@echo "  examples     - Compile example applications"
	@echo "  test         - Run all tests"
	@echo "  qemu         - Run in QEMU emulator"
	@echo "  raspberry-pi - Create Raspberry Pi SD image"
	@echo "  clean        - Clean all build artifacts"
	@echo "  docker       - Build Docker development environment"

# Kernel build
kernel:
	@echo "=== Building Quantum Microkernel ==="
	cd kernel && cargo build --release
	@echo "✓ Kernel built successfully"

# Compiler build
compiler:
	@echo "=== Building AetherScript Compiler ==="
	cd compiler && cargo build --release
	@echo "✓ Compiler built successfully"

# Example applications
examples: compiler
	@echo "=== Compiling Examples ==="
	./compiler/target/release/aetherc examples/hello_distributed.aethersrc --output examples/hello.rs --verbose
	@echo "✓ Examples compiled"

# Run tests
test:
	@echo "=== Running Tests ==="
	cargo test --workspace --verbose
	@echo "✓ All tests passed"

# QEMU emulation
qemu: kernel
	@echo "=== Starting QEMU Emulation ==="
	qemu-system-aarch64 \
		-M virt \
		-cpu cortex-a72 \
		-smp 4 \
		-m 2G \
		-kernel kernel/target/aarch64-unknown-none/release/aetheros-kernel \
		-nographic \
		-serial mon:stdio

# Raspberry Pi image
raspberry-pi: kernel
	@echo "=== Creating Raspberry Pi SD Image ==="
	@echo "Not yet implemented - requires bootloader integration"

# Docker development environment
docker:
	@echo "=== Building Docker Image ==="
	docker build -t aetheros-dev -f Dockerfile .
	@echo "✓ Docker image built"
	@echo "Run: docker run -it --rm -v $$(pwd):/aetheros aetheros-dev"

# Clean
clean:
	@echo "=== Cleaning Build Artifacts ==="
	cd kernel && cargo clean
	cd compiler && cargo clean
	rm -f examples/*.rs
	@echo "✓ Clean complete"

# Development shortcuts
dev-kernel:
	cd kernel && cargo build

dev-compiler:
	cd compiler && cargo build

dev-test:
	cargo test --workspace

# Continuous integration
ci: test
	@echo "=== CI Build Complete ==="
