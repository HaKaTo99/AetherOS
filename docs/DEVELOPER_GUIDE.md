# xAetherOS Developer Guide
**Version**: v10.2 "The Fabric" (Supreme Grade - Sovereign Framework)  
**Target Audience**: Systems Engineers, AI Developers, Distributed Systems Architects

---

## 🌌 1. Fast Track (Quick Start v10.0)
Gunakan langkah ini untuk menjalankan AetherOS dalam 5 menit.

### Prerequisites (Rust Nightly)
```bash
rustup default nightly
rustup target add x86_64-unknown-none
cargo install bootimage
```

### Build & Launch (QEMU)
```powershell
cd kernel
cargo run --release
```

---

## 🚀 2. Getting Started (Full Setup)

### Operating System
*   **Linux (Ubuntu 22.04+)**, **macOS**, or **Windows (WSL2/PowerShell)**.

### Target Architectures
xAetherOS mendukung platform berikut secara native:
- **x86_64 PC (UEFI/BIOS)**: Target utama untuk v10.2 (Sovereign Framework).
- **Raspberry Pi 4 (aarch64)**: Target IoT dengan GICv2 Hardening.
- **Universal Bridges**: Mendukung binari POSIX, Win32, Darwin, HarmonyOS, Symbian, dan webOS secara native.

---

## 🏛️ 2. Architecture Overview

xAetherOS is not just an OS; it's a **Distributed Intelligence Fabric**.

### The 3 Pillars
1.  **AI-Native Kernel**: The `Oracle Engine` inside the kernel predicts resource usage and orchestrates tasks based on intent, not just PID.
2.  **Quantum-Secure**: All communications via the `Quantum Bus` are encrypted with Post-Quantum Cryptography (Kyber/Dilithium).
3.  **Global Mesh**: Devices automatically discover peers and form a self-healing mesh. Resources (CPU, NPU, Storage) are traded in a global market.

---

## 💻 3. Developing Applications

AetherOS mendukung beragam runtime berdaulat. Gunakan `AetherShell` untuk memverifikasi kesiapan toolchain.

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
