# Contributing to AetherOS

Thank you for your interest in contributing to AetherOS! This document provides guidelines for contributing to the project.

## Code of Conduct

Be respectful, inclusive, and professional in all interactions.

## How to Contribute

### Reporting Bugs
1. Check existing issues first
2. Use the bug report template
3. Include reproduction steps
4. Provide system information

### Suggesting Features
1. Check existing feature requests
2. Use the feature request template
3. Explain the use case
4. Consider implementation complexity

### Pull Requests
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`make test`)
6. Commit with clear messages
7. Push to your fork
8. Open a Pull Request

## Development Setup

```bash
# Clone repository
git clone https://github.com/AetherOS-Project/aetheros
cd aetheros

# Install dependencies
make docker  # Or install Rust, LLVM, etc. manually

# Build
make all

# Run tests
make test
```

## Coding Standards

### Rust Code
- Follow Rust style guide
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Add documentation comments for public APIs

### AetherScript Code
- Use clear, descriptive names
- Add comments for complex logic
- Follow resource-aware programming paradigm

## Testing

- Write unit tests for new functionality
- Ensure integration tests pass
- Test on target hardware when possible

## Documentation

- Update README.md for user-facing changes
- Add API documentation for new functions
- Update CHANGELOG.md

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

Join our [Discord](https://discord.gg/aetheros) or open a discussion on GitHub.
