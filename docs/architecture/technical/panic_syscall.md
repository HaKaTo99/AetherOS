# Audit: `kernel/src/panic.rs` & `kernel/src/syscall/mod.rs`

Summary:
- `panic.rs` implements a panic handler that prints panic info to platform serial and halts the CPU.
- `syscall/mod.rs` exposes a small syscall shim with `write`, `exit`, and a custom `sys_ai_sync` call.

Strengths:
- Panic handler attempts to use platform serial output to record panic details and halts safely.
- Syscall layer checks RBAC for `write` and logs audits for large writes; integrates `AI` intent parsing for telemetry.

Risks/Findings:
- Panic handler runs in constrained context; calling into HAL may rely on global state that could be corrupted during panic — take care not to increase panic handler complexity.
- `sys_exit` halts the thread via infinite loop; there is no process isolation or scheduler-driven teardown for processes (expected for early kernel).
- `sys_write` constructs slice from raw pointer `buf_ptr` with no bounds validation beyond `count` — callers must ensure pointer validity; consider validating against process memory maps.

Recommendations:
1. Keep panic handler minimal; prefer writing minimal CPU state and return to firmware where possible. Consider storing crash dump to reserved storage before complex IO.
2. Add pointer and bounds validation in syscall layer using process address space checks (when MMU/user space present).
3. For `sys_exit`, integrate with scheduler to cleanly free resources if process model exists; avoid infinite loop in kernel-mode exit.

References:
- `kernel/src/panic.rs`
- `kernel/src/syscall/mod.rs`
