# Audit: `kernel/src/security/crypto.rs`

Summary:
- Provides a PQC-capable API surface (Kyber / Dilithium levels) but currently implements simulated ciphertexts/signatures and mock key material.

Strengths:
- Well-defined API (`QuantumSecurity`) and selection of security levels.
- Logging/audit hooks exist to track crypto operations.

Risks/Findings:
- Core cryptographic operations are simulated (fixed-length vectors and mocked logic). This is explicitly marked in comments and code.
- `verify()` uses message[0] == signature[0] for simulation — not cryptographically valid.
- Using simulated crypto in production images would be catastrophic; tests based on these mocks are insufficient to prove cryptographic correctness.

Recommendations:
1. Integrate a vetted no_std PQC implementation (link or vendor `kyber-no-std` / `pqcrypto` crates where available) and provide build flags for simulation vs production.
2. Add unit tests with known answer test vectors (KATs) and interoperability tests against reference implementations.
3. Ensure private key material is stored/handled with secure memory (zeroize on drop) and add protections for side-channel leakage on supported platforms.
4. Document which code paths are simulation-only and gate them behind a `#[cfg(feature = "simulated-crypto")]` flag so release builds cannot include mocks.

References:
- File: `kernel/src/security/crypto.rs`
