# xAetherOS Developer Guide
**Version**: v5.1 "Foundation"  
**Target Audience**: Systems Engineers, AI Developers, Distributed Systems Architects

---

## 🚀 1. Getting Started

### Prerequisites
*   **Operating System**: Linux (Ubuntu 22.04+), macOS, or Windows (WSL2 recommended).
*   **Rust Toolchain**: Nightly channel required.
    ```bash
    rustup toolchain install nightly
    rustup target add aarch64-unknown-none-softfloat
    rustup component add rust-src llvm-tools-preview
    ```
*   **QEMU**: For emulation.
    ```bash
    # Ubuntu
    sudo apt install qemu-system-aarch64 qemu-system-x86
    ```

### Building the Kernel
Clone the repository and build for your target architecture.

**Raspberry Pi 4 (aarch64)**
```bash
cargo build --release --target aarch64-unknown-none-softfloat
./build_rpi4.ps1 # Powershell script to generate SD card image
```

**x86_64 PC (UEFI)**
```bash
cargo build --release --target x86_64-unknown-uefi
```

### Running in Emulator
```bash
# Run aarch64 (RPi4 emulation)
qemu-system-aarch64 -M raspi4b -kernel target/aarch64.../kernel8.img -serial stdio

# Run x86_64
cargo run --target x86_64-unknown-uefi
```

---

## 🏛️ 2. Architecture Overview

xAetherOS is not just an OS; it's a **Distributed Intelligence Fabric**.

### The 3 Pillars
1.  **AI-Native Kernel**: The `Oracle Engine` inside the kernel predicts resource usage and orchestrates tasks based on intent, not just PID.
2.  **Quantum-Secure**: All communications via the `Quantum Bus` are encrypted with Post-Quantum Cryptography (Kyber/Dilithium).
3.  **Global Mesh**: Devices automatically discover peers and form a self-healing mesh. Resources (CPU, NPU, Storage) are traded in a global market.

---

## 💻 3. Developing Applications

xAetherOS supports multiple runtimes. Choose the best one for your needs.

### A. Universal Runtime (QuickJS)
Write apps in standard JavaScript/TypeScript.

```javascript
// hello.js
console.log("Hello from xAetherOS!");

// Using the AI Agent API
const agent = new AiAgent("Llama-7B");
const response = agent.chat("Explain quantum entanglement");
console.log(response);
```

### B. Rust Native Apps
For maximum performance and direct kernel access.

```rust
#![no_std]
use aether_sdk::{scheduler, net, quantum};

fn main() {
    // 1. Create a distributed task
    let task_id = scheduler::spawn(|| {
        loop {
            // 2. Use Quantum Bus for secure messaging
            let msg = quantum::encrypt("Secret Data");
            net::broadcast(msg);
        }
    });
}
```

### C. AI Agent (WASM)
Deploy autonomous agents as WebAssembly modules.

```rust
// agent.rs (compile to wasm32-wasi)
#[no_mangle]
pub fn run_inference(input: &str) -> String {
    // Access NPU via syscall
    let tensor = tensor::from_str(input);
    let result = npu::compute(tensor);
    result.to_string()
}
```

---

## 🌐 4. Distributed API (Quantum Bus)

The most powerful feature of xAetherOS is the Ability Market.

### Publishing a Capability
If your device has a powerful GPU, you can farm it out.

```rust
use aether_sdk::distributed::market;

fn publish_gpu() {
    market::place_ask(
        resource_id: "GPU-RTX4090",
        amount: 50, // 50 AetherTokens per hour
        capability: Capability::Compute(100_000) // 100 TFLOPS
    );
}
```

### Consuming a Capability
Offload heavy tasks to the mesh.

```rust
fn render_scene() {
    // Find best provider for 3D rendering
    let node = market::find_provider(Capability::Render3D);
    
    // Migrate task to that node
    migration::move_task(current_task(), node);
}
```

---

## 🤝 5. Contributing

Please read `CONTRIBUTING.md` before submitting PRs.
*   **Style**: Run `cargo fmt` and `cargo clippy`.
*   **Docs**: Ensure all public APIs have examples.
*   **Tests**: Add unit tests in `kernel/src/tests`.

---

**Happy Hacking!** 🚀
