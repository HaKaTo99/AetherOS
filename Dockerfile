FROM ubuntu:22.04

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    curl \
    git \
    clang \
    llvm \
    qemu-system-arm \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Add ARM64 target
RUN rustup target add aarch64-unknown-none

# Set working directory
WORKDIR /aetheros

# Copy project files
COPY . .

# Default command
CMD ["/bin/bash"]
