# iPhone (iOS) Porting Notes

Supporting iPhone (iOS) devices presents significant technical and legal challenges. This document records constraints, development approaches, and realistic goals.

## Constraints
- Apple enforces signed firmware and locked boot chains; installing a custom OS on non-jailbroken devices is not possible without Apple's cooperation.
- Drivers and hardware access are proprietary; reverse engineering risks legal issues.

## Practical Approaches
1. Research & Development on Jailbroken Devices
   - Use jailbroken phones for development, debugging, and proof-of-concept only.
2. OEM Partnership
   - Pursue official partnerships with Apple or contract manufacturers for certified support (long-term, high-effort).
3. Alternate Paths
   - Provide remote runtime (streaming) solutions where the iPhone acts as a thin client to AetherOS running elsewhere.

## Recommendations
- Do not prioritize general iPhone support unless a strategic partnership is established.
- Focus on cross-platform compatibility (WebAssembly, cloud/offload) to support iPhone clients indirectly.
