# Security Policy

## Supported Versions

We take security seriously. The following versions are currently supported with security updates:

| Version | Supported          | Status |
| ------- | ------------------ | ------ |
| 2.0.x   | :white_check_mark: | **Current Production Release** |
| 1.9.x   | :white_check_mark: | Maintenance (until March 2026) |
| 1.8.x   | :x:                | End of Life |
| < 1.8   | :x:                | No longer supported |

**Recommendation**: Always use the latest stable release (v2.0.0+) for the best security.

---

## Reporting a Vulnerability

If you discover a security vulnerability in AetherOS, please follow responsible disclosure:

### 🔒 Private Disclosure (Preferred)

1. **Do NOT create a public GitHub issue**
2. **Email**: `security@aetheros.org` (Placeholder - will be active soon)
3. **Include**:
   - Detailed description of the vulnerability
   - Steps to reproduce
   - Potential impact and severity assessment
   - Suggested fix (if available)

### Response Timeline

- **Initial Response**: Within 48 hours
- **Triage & Assessment**: Within 5 business days
- **Patch Development**: Depends on severity (critical: 7 days, high: 14 days)
- **Public Disclosure**: After patch release + 7-day grace period

---

## Security Features in AetherOS v2.0

### Current Protections

- **Memory Safety**: Rust's ownership system prevents common vulnerabilities
- **Capability System**: Token-based access control
- **Guard Pages**: Stack overflow protection
- **Code Signing**: Secure boot framework (stub implementation)
- **Minimal Attack Surface**: Microkernel architecture

### Known Limitations

- **Network Stack**: Currently limited to loopback (physical network support in development)
- **Input Validation**: Not all RPC message deserialization paths are fuzz-tested
- **Cryptography**: Uses standard Rust crypto libraries (not yet quantum-resistant)

---

## CVE Response Process

When a vulnerability is confirmed:

### 1. Triage
- Verify the vulnerability
- Assess severity using CVSS v3 scoring
- Assign internal priority (P0: Critical, P1: High, P2: Medium, P3: Low)

### 2. Patch Development
- Develop fix in a private security branch
- Write comprehensive tests
- Perform internal code review

### 3. Security Advisory
- Draft advisory with CVE details
- Prepare patch release notes
- Notify key stakeholders (if applicable)

### 4. Release
- Publish patched version (e.g., v2.0.1)
- Release security advisory
- Update this SECURITY.md

### 5. Public Disclosure
- Wait 7 days after patch release
- Publish full details in GitHub Security Advisories
- Credit reporter (if desired)

---

## Security Best Practices for Users

If you're deploying AetherOS:

1. **Keep Updated**: Always use the latest stable version
2. **Secure Boot**: Enable if available on your platform
3. **Network Isolation**: Until full network stack is audited, isolate AetherOS devices
4. **Physical Security**: Protect UART/serial console access
5. **Monitor Logs**: Watch for unusual kernel panics or behavior

---

## Security Hardening Roadmap

Future security improvements planned:

- **Fuzzing**: Comprehensive fuzzing of RPC deserialization (cargo-fuzz)
- **Formal Verification**: Critical components (memory allocator, scheduler)
- **Network Security**: TLS support for Quantum Bus RPC
- **Quantum-Resistant Crypto**: Post-quantum cryptography integration
- **Hardware Security**: TPM support, secure enclaves

---

## Credit

We appreciate responsible security researchers. If you report a vulnerability:

- We will credit you in the security advisory (unless you prefer anonymity)
- We may offer a token of appreciation (project swag, acknowledgment)

---

## Security Contact

- **Email**: h.krisnanto@gmail.com
- **PGP Key**: Available upon request
- **GitHub Security Advisories**: [View Advisories](https://github.com/HaKaTo99/AetherOS/security/advisories)

---

## Scope

**In Scope**:
- Memory corruption vulnerabilities
- Privilege escalation
- Information disclosure
- Cryptographic weaknesses
- Denial of service (kernel panics)

**Out of Scope**:
- Social engineering attacks
- Physical attacks requiring hardware modifications
- Issues in third-party dependencies (report to upstream)

---

**Last Updated**: February 1, 2026 (v2.0.0 release)
