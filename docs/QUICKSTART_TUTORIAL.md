# QUICKSTART TUTORIAL — Developer Guide

## 5-Minute Quick Start

### Step 1: Installation

```bash
# macOS / Linux
curl -sSL https://get.aetheros.dev | bash

# Windows (WSL2)
wsl --install Ubuntu
curl -sSL https://get.aetheros.dev | bash

# Docker (any platform)
docker run -it aetheros/dev:latest
```

### Step 2: Verify Installation

```bash
# Check installation
aether --version
# Output: AetherOS SDK v1.0

# List available commands
aether --help

# Check system requirements
aether doctor
```

### Step 3: Create First App

```bash
# Create new project
aether new hello-world --template=distributed

# Explore generated structure
cd hello-world
tree .
```

## Your First AetherScript App

File: `src/main.aethersrc`

```swift
// Simple distributed calculator
app DistributedCalculator {
    @memory(budget: 16.mb, distributed: true)
    @compute(min: 0.1.tflops)
    @security(level: .standard)
    
    distributed func main() {
        print("🧮 Distributed Calculator Starting...")
        
        // Input from user
        let numbers = getInput("Enter numbers (comma separated):")
            .split(",")
            .map { $0.trim().toFloat() }
        
        print("Processing \(numbers.count) numbers...")
        
        // Distribute computation automatically
        let results = computeDistributed(numbers: numbers) {
            // This block will run on optimal device(s)
            numbers.map { n in
                // Heavy computation example
                fibonacci(n.truncate())
            }
        }
        
        print("Results: \(results)")
        print("Computation complete! 🎉")
    }
    
    // Helper function with manual memory control
    manual func fibonacci(n: Int) -> BigInt {
        if n <= 1 { return BigInt(n) }
        
        raw memory fibCache {
            var cache = allocate(size: (n + 1) * 8, aligned: 8)
            
            // Initialize cache
            cache[0] = BigInt(0)
            cache[1] = BigInt(1)
            
            // Compute with manual optimization
            for i in 2...n {
                cache[i] = cache[i-1] + cache[i-2]
            }
            
            let result = cache[n]
            deallocate(cache)
            return result
        }
    }
}
```

## Build & Run

### Local Execution

```bash
# Build for current machine
aether build

# Run locally
aether run

# Debug mode
aether run --debug

# Performance profile
aether run --profile
```

### Distributed Execution

```bash
# Discover available devices
aether devices discover

# Run on specific device
aether run --device=raspberry-pi-4

# Run distributed (auto-optimized)
aether run --distributed

# Specify compute requirements
aether run --require-gpu --require-npu --memory=2gb
```

(Additional developer workflow, packaging, and advanced features omitted for brevity — full guide in docs.)
