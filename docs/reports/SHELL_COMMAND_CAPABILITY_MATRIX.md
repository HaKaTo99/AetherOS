# AetherOS Shell Command Capability Matrix

Date: 2026-03-31
Source of truth: kernel/src/enterprise/shell.rs

## 1. Effective Command Surface (Current)

### 1.1 Active commands
- help: prints explicit capability profile (active vs disabled command list).
- calc: prints calculator demo message (non-interactive simulation text).
- clear: soft clear by printing multiple CRLF lines.
- exit: exits shell loop and calls platform shutdown.
- meshstatus: prints mesh node and peer status from GLOBAL_MESH.

### 1.2 Disabled bridge commands (all route to bridge_disabled)
- omni
- python
- node
- java
- rustc
- php
- linux
- unix
- windows
- mac
- harmony
- symbian
- webos
- blender
- vlc
- apk
- intent
- identity
- evolve
- tactical
- captrade
- onemind
- bci

Behavior for disabled bridge commands:
- Output prefix: [BRIDGE DISABLED]
- Policy note in output: production shell blocks bridge execution in current stage lane.
- Deny audit: warning-level security audit is emitted with throttle to avoid log flood under noisy/hostile input.

## 2. Parser/Resolution Behavior

The shell resolver is intentionally tolerant and deterministic:
- Lower/upper case command matching accepted (prefix compare is case-insensitive).
- Leading whitespace ignored.
- First token only is parsed (arguments are effectively ignored for command routing).
- Trailing punctuation ; : , is trimmed.
- Numeric shortcut mapping:
  - 1 -> help
  - 2 -> calc
  - 3 -> clear
  - 0 -> exit
- Noise-handling fallback:
  - extracts alpha-only stream and applies heuristic matching.
  - first-letter fallback for h/e/c when noisy streams occur.

Practical consequence:
- command parser is robust for serial/PS2 noisy input,
- but argument-level routing for bridge subcommands is currently not active.

## 3. Reachability vs Advertisement

Help output is now aligned with dispatch behavior. It no longer implies bridge executability and instead prints explicit status labels.

Operational truth table:
- advertised in help: explicit status profile (active core + disabled bridge list).
- reachable behavior now: 5 active commands + disabled bridge message for 23 bridge commands.

## 4. Risk and Validation Notes

- Positive: stable, low-allocation parser path reduces shell lockups in unstable input conditions.
- Positive: deterministic policy classification layer (`Active`/`BridgeDenied`/`Unknown`) reduces dispatch drift risk.
- Risk: bridge commands are intentionally blocked, so users may still expect activation in later stages.
- Recommended validation set:
  - smoke: 1,2,3,0 shortcuts.
  - direct: help/calc/clear/exit.
  - bridge sample: omni/python/windows should all return bridge-disabled policy message.
  - malformed/noisy input should not panic and should return unknown command debug trace.
  - anomaly: repeated unknown commands should remain responsive while audit entries are throttled.
  - startup policy integrity: shell prints `[POLICY] shell-command-policy: PASS` under normal conditions.
    - startup policy integrity now also covers duplicate-entry detection and lowercase prefix assumptions.

## 5. Suggested Next Action

If bridge activation is desired in future stages:
1. keep resolver unchanged (already robust),
2. replace bridge_disabled arms one-by-one with guarded handlers,
3. add per-command stage gate and explicit audit log on deny/allow,
4. keep help profile derived from the same command policy table used by dispatch.

## 6. Implementation Delta (2026-03-31)

- `print_help` updated to show capability profile with `[active]` and `[disabled]` markers.
- `meshstatus` now explicitly shown in help active command list.
- `handle_command` now mirrors interactive dispatch policy for meshstatus and bridge-disabled commands.
- Added shared command executor path so interactive loop and non-interactive handler use the same dispatch policy.
- Added single-source command tables (`ACTIVE_COMMANDS` and `BRIDGE_COMMANDS`) to reduce future drift between help and runtime behavior.
- Added centralized stage policy label in shell output for consistent operator-facing policy text.
- Added throttled military-grade deny audit for bridge commands (`AuditSeverity::Warning`) to preserve traceability without compromising responsiveness.
- Added deterministic pre-dispatch policy classifier (`classify_command_policy`) so executor behavior remains consistent and auditable under future changes.
- Promoted resolver prefix mapping to a global single-source table (`COMMAND_PREFIXES`) to reduce hidden divergence.
- Added invariant tests to enforce table disjointness (`ACTIVE_COMMANDS` vs `BRIDGE_COMMANDS`) and prefix coverage for all declared commands.
- Added resolver regression tests for shortcut mapping, whitespace/punctuation trimming, and resolver-policy consistency across all declared commands.
- Added invariants to reject duplicate command entries across command tables and resolver source table.
- Added invariants to enforce lowercase-ascii resolver prefixes, preserving deterministic case-insensitive matching assumptions.
- Added throttled warning audit for unknown non-empty commands to improve operational traceability under fuzz/noise conditions.
- Added exact set-equivalence invariant between policy tables and resolver prefix table to prevent hidden command drift.
- Added runtime policy self-check banner at shell startup with non-disruptive PASS/FAIL output.
- Added critical audit escalation when policy self-check fails (no panic, shell remains operational for resilience).
- Expanded runtime self-check to include duplicate-command detection and resolver lowercase-prefix integrity.
