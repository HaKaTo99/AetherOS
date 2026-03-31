//! Aether Interactive Shell (no-alloc stable parser)

use crate::hal;
use core::sync::atomic::{AtomicUsize, Ordering};

const SOFT_CLEAR_LINES: usize = 24;
const INPUT_MAX: usize = 64;
const LETTERS_MAX: usize = 64;
const ACTIVE_COMMANDS: [&str; 5] = ["help", "calc", "clear", "exit", "meshstatus"];
const SHELL_POLICY_STAGE_LABEL: &str = "Stage-7 lockdown";
const BRIDGE_AUDIT_THROTTLE_TICKS: usize = 16;
const UNKNOWN_AUDIT_THROTTLE_TICKS: usize = 64;
const POLICY_SELF_CHECK_TAG: &str = "shell-command-policy";
const BRIDGE_COMMANDS: [&str; 23] = [
    "omni", "python", "node", "java", "rustc", "php",
    "linux", "unix", "windows", "mac", "harmony", "symbian", "webos",
    "blender", "vlc", "apk",
    "intent", "identity", "evolve", "tactical", "captrade", "onemind", "bci",
];
const COMMAND_PREFIXES: &[(&[u8], &str)] = &[
    (b"help", "help"),
    (b"exit", "exit"),
    (b"clear", "clear"),
    (b"calc", "calc"),
    (b"meshstatus", "meshstatus"),
    (b"omni", "omni"),
    (b"python", "python"),
    (b"node", "node"),
    (b"java", "java"),
    (b"rustc", "rustc"),
    (b"php", "php"),
    (b"linux", "linux"),
    (b"unix", "unix"),
    (b"windows", "windows"),
    (b"mac", "mac"),
    (b"harmony", "harmony"),
    (b"symbian", "symbian"),
    (b"webos", "webos"),
    (b"blender", "blender"),
    (b"vlc", "vlc"),
    (b"apk", "apk"),
    (b"intent", "intent"),
    (b"identity", "identity"),
    (b"evolve", "evolve"),
    (b"tactical", "tactical"),
    (b"captrade", "captrade"),
    (b"onemind", "onemind"),
    (b"bci", "bci"),
];

pub struct AetherShell;

static LAST_BRIDGE_AUDIT_TICK: AtomicUsize = AtomicUsize::new(0);
static LAST_UNKNOWN_AUDIT_TICK: AtomicUsize = AtomicUsize::new(0);

enum CommandExec {
    Handled,
    Exit,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CommandPolicy {
    Active,
    BridgeDenied,
    Unknown,
}

struct LineInput {
    buf: [u8; INPUT_MAX],
    len: usize,
}

impl LineInput {
    const fn new() -> Self {
        Self { buf: [0; INPUT_MAX], len: 0 }
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn as_slice(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

fn build_marker() -> &'static str {
    option_env!("AETHER_BUILD_ID").unwrap_or("INPUT-STABLE-UNSTAMPED")
}

pub fn self_test_core_commands() {
    let platform = hal::get_platform();
    let cases: [(&[u8], &str); 8] = [
        (b"1", "help"),
        (b"2", "calc"),
        (b"3", "clear"),
        (b"0", "exit"),
        (b"help", "help"),
        (b"calc", "calc"),
        (b"clear", "clear"),
        (b"exit", "exit"),
    ];

    let mut passed = 0usize;
    for (input, expected) in cases {
        if resolve_core_command(input) == expected {
            passed += 1;
        }
    }

    platform.puts("[SMOKE] shell-core ");
    if passed == cases.len() {
        platform.puts("PASS\r\n");
    } else {
        platform.puts("FAIL\r\n");
    }
}

impl AetherShell {
    pub fn start() {
        let platform = hal::get_platform();

        platform.puts("--- AetherOS v10.1 Sovereign Shell ---\r\n");
        platform.puts("[BUILD] ");
        platform.puts(build_marker());
        platform.puts("\r\n");
        platform.puts("[AUTHORITY] Welcome, Architect Herman Krisnanto.\r\n");
        print_policy_self_check(platform);
        platform.puts("Type 'help' (or 1) for commands.\r\n");
        platform.puts("Shortcuts: 1=help, 2=calc, 3=clear, 0=exit\r\n");

        loop {
            platform.puts("\r\nAetherShell> ");
            let line = read_line(platform);
            if line.is_empty() {
                continue;
            }

            let core = resolve_core_command(line.as_slice());
            match execute_command(platform, core) {
                CommandExec::Handled => {}
                CommandExec::Exit => break,
                CommandExec::Unknown => {
                    platform.puts("\r\nUnknown command. Type 'help'.\r\n");
                    platform.puts("[DEBUG] Raw input: ");
                    for b in line.as_slice() {
                        platform.puts(&format!("{:02X} ", b));
                    }
                    platform.puts(" | ");
                    for b in line.as_slice() {
                        let c = *b as char;
                        if c.is_ascii_graphic() || c == ' ' {
                            platform.puts(&format!("{}", c));
                        } else {
                            platform.puts(".");
                        }
                    }
                    platform.puts("\r\n");
                }
            }
        }

        platform.puts("\r\nShutting down...\r\n");
        platform.shutdown();
    }

    pub fn handle_command(cmd: &str) {
        let platform = hal::get_platform();
        match execute_command(platform, resolve_core_command(cmd.as_bytes())) {
            CommandExec::Exit => {
                platform.puts("\r\nShutting down...\r\n");
                platform.shutdown();
            }
            CommandExec::Handled => {}
            CommandExec::Unknown => platform.puts("\r\nUnknown command. Type 'help'.\r\n"),
        }
    }
}

fn is_bridge_command(cmd: &str) -> bool {
    BRIDGE_COMMANDS.iter().any(|name| *name == cmd)
}

fn classify_command_policy(cmd: &str) -> CommandPolicy {
    if ACTIVE_COMMANDS.iter().any(|name| *name == cmd) {
        CommandPolicy::Active
    } else if is_bridge_command(cmd) {
        CommandPolicy::BridgeDenied
    } else {
        CommandPolicy::Unknown
    }
}

fn has_duplicate_entries(entries: &[&str]) -> bool {
    for (i, lhs) in entries.iter().enumerate() {
        for rhs in entries.iter().skip(i + 1) {
            if lhs == rhs {
                return true;
            }
        }
    }
    false
}

fn prefixes_are_lowercase_ascii() -> bool {
    for (prefix, _) in COMMAND_PREFIXES {
        for b in *prefix {
            if !b.is_ascii_lowercase() {
                return false;
            }
        }
    }
    true
}

fn prefix_command_names_have_duplicates() -> bool {
    for (i, (_, lhs)) in COMMAND_PREFIXES.iter().enumerate() {
        for (_, rhs) in COMMAND_PREFIXES.iter().skip(i + 1) {
            if lhs == rhs {
                return true;
            }
        }
    }
    false
}

fn policy_tables_are_consistent() -> bool {
    // 0) no duplicates in command declaration tables
    if has_duplicate_entries(&ACTIVE_COMMANDS) || has_duplicate_entries(&BRIDGE_COMMANDS) {
        return false;
    }
    if prefix_command_names_have_duplicates() {
        return false;
    }

    // 0.1) resolver prefix assumptions
    if !prefixes_are_lowercase_ascii() {
        return false;
    }

    // 1) active and bridge tables must be disjoint
    for active in ACTIVE_COMMANDS {
        if BRIDGE_COMMANDS.iter().any(|bridge| *bridge == active) {
            return false;
        }
    }

    // 2) every declared command must exist in resolver prefix table
    for active in ACTIVE_COMMANDS {
        if !COMMAND_PREFIXES.iter().any(|(_, cmd)| *cmd == active) {
            return false;
        }
    }
    for bridge in BRIDGE_COMMANDS {
        if !COMMAND_PREFIXES.iter().any(|(_, cmd)| *cmd == bridge) {
            return false;
        }
    }

    // 3) resolver table entries must map to known policy commands
    for (_, cmd) in COMMAND_PREFIXES {
        let is_active = ACTIVE_COMMANDS.iter().any(|name| name == cmd);
        let is_bridge = BRIDGE_COMMANDS.iter().any(|name| name == cmd);
        if !is_active && !is_bridge {
            return false;
        }
    }

    true
}

fn print_policy_self_check(platform: &dyn hal::Platform) {
    platform.puts("[POLICY] ");
    platform.puts(POLICY_SELF_CHECK_TAG);
    platform.puts(": ");

    if policy_tables_are_consistent() {
        platform.puts("PASS\r\n");
    } else {
        platform.puts("FAIL\r\n");
        crate::enterprise::audit::log_security(
            crate::enterprise::audit::AuditSeverity::Critical,
            POLICY_SELF_CHECK_TAG,
            "Shell command policy table integrity check failed",
        );
    }
}

fn execute_command(platform: &dyn hal::Platform, cmd: &str) -> CommandExec {
    match classify_command_policy(cmd) {
        CommandPolicy::Active => match cmd {
            "help" => {
                print_help(platform);
                CommandExec::Handled
            }
            "calc" => {
                print_calc(platform);
                CommandExec::Handled
            }
            "clear" => {
                soft_clear(platform);
                CommandExec::Handled
            }
            "meshstatus" => {
                use crate::mesh::GLOBAL_MESH;
                GLOBAL_MESH.lock().debug_print_status();
                CommandExec::Handled
            }
            "exit" => CommandExec::Exit,
            _ => CommandExec::Unknown,
        },
        CommandPolicy::BridgeDenied => {
            bridge_disabled(platform, cmd);
            CommandExec::Handled
        }
        CommandPolicy::Unknown => {
            maybe_audit_unknown_command(platform, cmd);
            CommandExec::Unknown
        }
    }
}

fn read_line(platform: &dyn hal::Platform) -> LineInput {
    let mut line = LineInput::new();

    loop {
        let raw = platform.get_char();
        if raw == 0 || raw == 0xFF {
            continue;
        }

        let c = raw & 0x7F;

        if c == b'\r' || c == b'\n' {
            platform.puts("\r\n");
            break;
        }

        if c == 8 || c == 127 {
            if line.len > 0 {
                line.len -= 1;
                platform.puts("\x08 \x08");
            }
            continue;
        }

        // Izinkan semua karakter ASCII printable (0x20..=0x7E) dan simbol umum
        if c < 0x20 || c > 0x7E {
            continue;
        }

        if line.len < INPUT_MAX {
            line.buf[line.len] = c;
            line.len += 1;
            platform.put_char(c);
        }
    }

    line
}

fn resolve_core_command(input: &[u8]) -> &'static str {
    // Skip leading whitespace
    let mut start = 0;
    while start < input.len() && (input[start] == b' ' || input[start] == b'\t') {
        start += 1;
    }
    let mut input = &input[start..];

    // Extract first token only (stop at whitespace)
    let mut end = 0;
    while end < input.len() && input[end] != b' ' && input[end] != b'\t' {
        end += 1;
    }
    input = &input[..end];

    // Trim trailing punctuation (common in smoketests, e.g., ';', ':', ',')
    while input.len() > 0 {
        let last = input[input.len() - 1];
        if last == b';' || last == b':' || last == b',' {
            input = &input[..input.len() - 1];
        } else {
            break;
        }
    }

    // Find prefix match
    for &(prefix, cmd) in COMMAND_PREFIXES {
        if input.len() >= prefix.len() && input[..prefix.len()].eq_ignore_ascii_case(prefix) {
            if input.len() == prefix.len() {
                return cmd;
            }
        }
    }

    // Fallback: original logic for shortcuts (1,2,3,0)
    let mut has_alpha = false;
    let mut has_0 = false;
    let mut has_1 = false;
    let mut has_2 = false;
    let mut has_3 = false;
    let mut letters = [0u8; LETTERS_MAX];
    let mut n = 0usize;
    for &b in input {
        let c = (b & 0x7F).to_ascii_lowercase();
        match c {
            b'0' => has_0 = true,
            b'1' => has_1 = true,
            b'2' => has_2 = true,
            b'3' => has_3 = true,
            b'a'..=b'z' => has_alpha = true,
            _ => {}
        }
        if n < LETTERS_MAX && c.is_ascii_alphabetic() {
            letters[n] = c;
            n += 1;
        }
    }

    if !has_alpha {
        if has_0 { return "exit"; }
        if has_3 { return "clear"; }
        if has_2 { return "calc"; }
        if has_1 { return "help"; }
    }

    if n == 0 {
        return "";
    }

    let ls = &letters[..n];

    if starts_with(ls, b"help") || starts_with(ls, b"hlp") || starts_with(ls, b"hep") || contains(ls, b"help") {
        return "help";
    }
    if starts_with(ls, b"exit") || starts_with(ls, b"ex") || contains(ls, b"exit") {
        return "exit";
    }
    if starts_with(ls, b"clear") || starts_with(ls, b"cl") || contains(ls, b"clear") {
        return "clear";
    }
    if starts_with(ls, b"calc") || starts_with(ls, b"ca") || contains(ls, b"calc") {
        return "calc";
    }

    // Last-resort first-letter fallback for noisy keyboard stream
    match ls[0] {
        b'h' => "help",
        b'e' => "exit",
        b'c' => {
            if n >= 2 && ls[1] == b'l' {
                "clear"
            } else {
                "calc"
            }
        }
        _ => "",
    }
}

fn starts_with(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.len() > haystack.len() {
        return false;
    }
    for i in 0..needle.len() {
        if haystack[i] != needle[i] {
            return false;
        }
    }
    true
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() {
        return true;
    }
    if needle.len() > haystack.len() {
        return false;
    }

    let limit = haystack.len() - needle.len();
    for i in 0..=limit {
        let mut ok = true;
        for j in 0..needle.len() {
            if haystack[i + j] != needle[j] {
                ok = false;
                break;
            }
        }
        if ok {
            return true;
        }
    }
    false
}

fn bridge_disabled(platform: &dyn hal::Platform, name: &str) {
    maybe_audit_bridge_denial(platform, name);

    platform.puts("[BRIDGE DISABLED] ");
    platform.puts(name);
    platform.puts(": production shell blocks bridge execution (");
    platform.puts(SHELL_POLICY_STAGE_LABEL);
    platform.puts(").\r\n");
}

fn maybe_audit_bridge_denial(platform: &dyn hal::Platform, name: &str) {
    let now = platform.get_ticks() as usize;
    let last = LAST_BRIDGE_AUDIT_TICK.load(Ordering::Relaxed);
    if now.wrapping_sub(last) >= BRIDGE_AUDIT_THROTTLE_TICKS
        && LAST_BRIDGE_AUDIT_TICK
            .compare_exchange(last, now, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
    {
        crate::enterprise::audit::log_security(
            crate::enterprise::audit::AuditSeverity::Warning,
            name,
            "Shell bridge command denied by stage policy",
        );
    }
}

fn maybe_audit_unknown_command(platform: &dyn hal::Platform, cmd: &str) {
    // Ignore empty resolver outputs to avoid noise from blank input paths.
    if cmd.is_empty() {
        return;
    }

    let now = platform.get_ticks() as usize;
    let last = LAST_UNKNOWN_AUDIT_TICK.load(Ordering::Relaxed);
    if now.wrapping_sub(last) >= UNKNOWN_AUDIT_THROTTLE_TICKS
        && LAST_UNKNOWN_AUDIT_TICK
            .compare_exchange(last, now, Ordering::Relaxed, Ordering::Relaxed)
            .is_ok()
    {
        crate::enterprise::audit::log_security(
            crate::enterprise::audit::AuditSeverity::Warning,
            cmd,
            "Unknown shell command observed",
        );
    }
}

#[cfg(test)]
mod shell_policy_tests {
    use super::{
        classify_command_policy, policy_tables_are_consistent, resolve_core_command,
        CommandPolicy, ACTIVE_COMMANDS, BRIDGE_COMMANDS, COMMAND_PREFIXES,
    };

    #[test]
    fn classify_active_command_policy() {
        assert_eq!(classify_command_policy("help"), CommandPolicy::Active);
        assert_eq!(classify_command_policy("meshstatus"), CommandPolicy::Active);
    }

    #[test]
    fn classify_bridge_denied_policy() {
        assert_eq!(classify_command_policy("omni"), CommandPolicy::BridgeDenied);
        assert_eq!(classify_command_policy("windows"), CommandPolicy::BridgeDenied);
    }

    #[test]
    fn classify_unknown_policy() {
        assert_eq!(classify_command_policy("no-such-cmd"), CommandPolicy::Unknown);
        assert_eq!(classify_command_policy(""), CommandPolicy::Unknown);
    }

    #[test]
    fn command_tables_are_disjoint() {
        for active in ACTIVE_COMMANDS {
            assert!(
                !BRIDGE_COMMANDS.iter().any(|bridge| *bridge == active),
                "command '{}' appears in both active and bridge-denied tables",
                active
            );
        }
    }

    #[test]
    fn command_tables_are_covered_by_prefix_table() {
        for active in ACTIVE_COMMANDS {
            assert!(
                COMMAND_PREFIXES.iter().any(|(_, cmd)| *cmd == active),
                "active command '{}' missing from prefix table",
                active
            );
        }

        for bridge in BRIDGE_COMMANDS {
            assert!(
                COMMAND_PREFIXES.iter().any(|(_, cmd)| *cmd == bridge),
                "bridge command '{}' missing from prefix table",
                bridge
            );
        }
    }

    #[test]
    fn resolver_shortcuts_and_trimming_are_stable() {
        assert_eq!(resolve_core_command(b"1"), "help");
        assert_eq!(resolve_core_command(b"2"), "calc");
        assert_eq!(resolve_core_command(b"3"), "clear");
        assert_eq!(resolve_core_command(b"0"), "exit");

        assert_eq!(resolve_core_command(b"   help"), "help");
        assert_eq!(resolve_core_command(b"calc;"), "calc");
        assert_eq!(resolve_core_command(b"exit:"), "exit");
        assert_eq!(resolve_core_command(b"clear,"), "clear");
    }

    #[test]
    fn resolver_covers_all_declared_commands() {
        for (_, cmd) in COMMAND_PREFIXES {
            assert_eq!(resolve_core_command(cmd.as_bytes()), *cmd);
        }
    }

    #[test]
    fn resolver_and_policy_stay_consistent() {
        for active in ACTIVE_COMMANDS {
            let resolved = resolve_core_command(active.as_bytes());
            assert_eq!(resolved, active);
            assert_eq!(classify_command_policy(resolved), CommandPolicy::Active);
        }

        for bridge in BRIDGE_COMMANDS {
            let resolved = resolve_core_command(bridge.as_bytes());
            assert_eq!(resolved, bridge);
            assert_eq!(classify_command_policy(resolved), CommandPolicy::BridgeDenied);
        }
    }

    #[test]
    fn command_tables_have_no_duplicates() {
        // Active and bridge command tables must be unique internally.
        for (i, lhs) in ACTIVE_COMMANDS.iter().enumerate() {
            for rhs in ACTIVE_COMMANDS.iter().skip(i + 1) {
                assert_ne!(lhs, rhs, "duplicate active command '{}'", lhs);
            }
        }

        for (i, lhs) in BRIDGE_COMMANDS.iter().enumerate() {
            for rhs in BRIDGE_COMMANDS.iter().skip(i + 1) {
                assert_ne!(lhs, rhs, "duplicate bridge command '{}'", lhs);
            }
        }

        // Prefix output names must be unique as resolver source of truth.
        for (i, (_, lhs)) in COMMAND_PREFIXES.iter().enumerate() {
            for (_, rhs) in COMMAND_PREFIXES.iter().skip(i + 1) {
                assert_ne!(lhs, rhs, "duplicate resolver command '{}'", lhs);
            }
        }
    }

    #[test]
    fn resolver_prefixes_are_lowercase_ascii() {
        for (prefix, cmd) in COMMAND_PREFIXES {
            for b in *prefix {
                assert!(
                    b.is_ascii_lowercase(),
                    "resolver prefix for '{}' contains non-lowercase byte 0x{:02X}",
                    cmd,
                    b
                );
            }
        }
    }

    #[test]
    fn command_prefix_table_matches_policy_tables_exactly() {
        // Every prefix command must be either active or bridge-denied.
        for (_, cmd) in COMMAND_PREFIXES {
            let is_active = ACTIVE_COMMANDS.iter().any(|name| name == cmd);
            let is_bridge = BRIDGE_COMMANDS.iter().any(|name| name == cmd);
            assert!(
                is_active || is_bridge,
                "prefix command '{}' not represented in policy tables",
                cmd
            );
        }

        // All policy commands must exist in the resolver prefix table.
        for cmd in ACTIVE_COMMANDS {
            assert!(
                COMMAND_PREFIXES.iter().any(|(_, p)| *p == cmd),
                "active command '{}' missing from resolver prefix table",
                cmd
            );
        }
        for cmd in BRIDGE_COMMANDS {
            assert!(
                COMMAND_PREFIXES.iter().any(|(_, p)| *p == cmd),
                "bridge command '{}' missing from resolver prefix table",
                cmd
            );
        }
    }

    #[test]
    fn policy_tables_self_check_passes() {
        assert!(policy_tables_are_consistent());
    }
}

fn print_help(platform: &dyn hal::Platform) {
    platform.puts("\r\n=== AetherShell Capability Profile ===\r\n");
    platform.puts("  Stage policy: production bridge lockdown active (");
    platform.puts(SHELL_POLICY_STAGE_LABEL);
    platform.puts(").\r\n");
    platform.puts("\r\nActive commands:\r\n");
    for cmd in ACTIVE_COMMANDS {
        platform.puts("  [active] ");
        platform.puts(cmd);
        platform.puts("\r\n");
    }

    platform.puts("\r\nDisabled bridge commands:\r\n");
    for cmd in BRIDGE_COMMANDS {
        platform.puts("  [disabled] ");
        platform.puts(cmd);
        platform.puts("\r\n");
    }

    platform.puts("\r\nShortcuts: 1=help, 2=calc, 3=clear, 0=exit\r\n");
    platform.puts("Bridge policy: disabled commands return [BRIDGE DISABLED].\r\n");
}

fn print_calc(platform: &dyn hal::Platform) {
    platform.puts("\r\n[Calculator] Mode Active (Press Ctrl+C to exit - simulation)\r\n");
    platform.puts("Calculator demo skipped for shell responsiveness.\r\n");
}

fn soft_clear(platform: &dyn hal::Platform) {
    for _ in 0..SOFT_CLEAR_LINES {
        platform.puts("\r\n");
    }
}
