# Contributing to AetherOS

Thank you for your interest in contributing to AetherOS v2.0! This document provides guidelines and information for contributors.

---

##  Code of Conduct

- Be respectful, inclusive, and professional in all interactions
- Provide constructive feedback
- Focus on what is best for the community
- Show empathy towards other community members

---

##  Reporting Bugs

Before submitting a bug report:

1. **Check existing issues** to avoid duplicates
2. **Use the latest version** (v2.0.0)
3. **Provide clear reproduction steps**

**Bug Report Template**:
```markdown
**Environment**:
- AetherOS Version: v2.0.0
- Platform: x86_64 / aarch64
- Host OS: Windows/Linux/macOS

**Description**: Clear description of the bug

**Steps to Reproduce**:
1. Step one
2. Step two
3. Expected vs actual behavior

**Logs**: Paste kernel logs or panic output
```

---

##  Suggesting Features

Feature requests are welcome! Please:

1. **Check existing feature requests** first
2. **Explain the use case** clearly
3. **Consider implementation complexity**
4. **Align with project vision** (distributed computing, multi-platform)

---

##  Pull Requests

### Workflow

1. **Fork** the repository
2. **Create a feature branch**:
   ```bash
   git checkout -b feature/amazing-feature
   ```
3. **Make your changes**
4. **Add tests** for new functionality
5. **Run verification**:
   ```bash
   cargo fmt
   cargo clippy --all-targets -- -D warnings
   cargo test
   cargo build --release --target x86_64-unknown-none
   ```
6. **Commit** with clear, descriptive messages
7. **Push** to your fork
8. **Open a Pull Request** with a clear description

### PR Requirements

-  All tests pass
-  Code formatted (`cargo fmt`)
-  No clippy warnings
-  Documentation updated (if applicable)
-  CHANGELOG entry (for user-facing changes)

---

##  Development Setup

### Clone Repository

```bash
git clone https://github.com/HaKaTo99/AetherOS.git
cd AetherOS
```

### Install Dependencies

```bash
# Rust nightly
rustup default nightly

# Cross-compilation targets
rustup target add x86_64-unknown-none
rustup target add aarch64-unknown-none

# QEMU (for testing)
# Linux: sudo apt install qemu-system-x86 qemu-system-aarch64
# macOS: brew install qemu
# Windows: Download from qemu.org
```

### Build

```bash
cd kernel
cargo build --release --target x86_64-unknown-none
```

### Test

```bash
# Unit tests
cargo test

# Run in QEMU
qemu-system-x86_64 \
  -kernel target/x86_64-unknown-none/release/aetheros-kernel \
  -serial stdio
```

---

##  Coding Standards

### Rust Code

- **Follow Rust Style Guide**: Use `cargo fmt`
- **Fix Clippy Warnings**: Run `cargo clippy` before committing
- **Document Public APIs**: Add `///` doc comments with examples
- **Safety First**: Document all `unsafe` blocks with safety requirements

**Example**:
```rust
/// Allocates memory from the SMME pool.
///
/// # Safety
///
/// Caller must ensure the allocated memory is properly initialized
/// before use and deallocated with the correct size.
///
/// # Example
///
/// ```no_run
/// let ptr = unsafe { SMME.allocate(4096).expect("Out of memory") };
/// // Use memory...
/// unsafe { SMME.deallocate(ptr, 4096); }
/// ```
pub unsafe fn allocate(&self, size: usize) -> Result<usize, &'static str> {
    // Implementation...
}
```

### Commit Messages

Use conventional commits format:

```
feat: Add distributed KV store replication
fix: Correct scheduler priority inversion
docs: Update DEVELOPER_GUIDE with QEMU instructions
test: Add unit tests for SMME allocator
refactor: Simplify Quantum Bus RPC serialization
```

---

##  Testing

### Unit Tests

Write tests for all new functionality:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smme_allocation() {
        // Test implementation
    }
}
```

### Integration Tests

- Test cross-module interactions
- Verify distributed computing features
- Test on real hardware when possible (RPi4, x86_64 PC)

---

##  Documentation

Update documentation for any changes:

- **README.md**: User-facing features
- **DEVELOPER_GUIDE.md**: Architecture changes
- **API_REFERENCE.md**: New public APIs
- **CHANGELOG.md**: All user-visible changes
- **Rustdoc**: Inline code documentation

---

##  Architecture Guidelines

When contributing, keep in mind AetherOS design principles:

1. **Stability First**: No experimental features in main branch
2. **Symbian DNA**: Prefer Active Objects over threads
3. **Distributed-First**: Design for multi-device scenarios
4. **Resource Awareness**: Optimize for embedded devices
5. **Safety**: Minimize `unsafe`, document all invariants

---

##  Areas for Contribution

We welcome contributions in these areas:

- **Drivers**: Network cards, GPU, USB
- **Distributed Features**: Improved task migration, consensus algorithms
- **UI Framework**: New widgets, animations
- **Platform Support**: New architectures, device support
- **Documentation**: Tutorials, examples, translations
- **Testing**: More unit tests, fuzzing, hardware testing

---

##  License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

##  Questions?

- **GitHub Discussions**: Ask general questions
- **GitHub Issues**: Report bugs or request features
- **Email**: security@aetheros.org (for security issues only)

---

##  Thank You!

Your contributions make AetherOS better for everyone. We appreciate your time and effort!

---

**Happy Coding!** 
