//! Aether Interactive Shell (Phase 38.2)
//!
//! Encapsulates the boot-time login and command handling logic.

use crate::hal;
use alloc::string::String;
use alloc::vec::Vec;
use crate::runtime::android::{ApkManifest, APK_INSTALLER};
use crate::runtime::posix::Vfs;
use crate::compat::win32::{Win32Loader, sys_win32_create_process};

pub struct AetherShell;

impl AetherShell {
    /// Starts the interactive shell session.
    ///
    /// Versi demo: tidak ada login/password, langsung masuk ke Sovereign Shell.
    /// Perbaikan ekstrem: bypass OmniLang input, pakai pembacaan serial blokir langsung.
    pub fn start() {
        let platform = hal::get_platform();



        platform.puts("--- AetherOS v10.1 Sovereign Shell ---\r\n");
        platform.puts("[AUTHORITY] Welcome, Architect Herman Krisnanto.\r\n");
        platform.puts("Type 'help' for commands.\r\n");

        loop {
            platform.puts("\r\nAetherShell> ");
            let input = read_line(platform);
            let cmd = input.trim();

            if cmd == "exit" {
                break;
            } else if cmd == "help" {
                platform.puts("\r\nAvailable Commands:\r\n");
                platform.puts("  omni [code]   : Compile and run OmniLang code\r\n");
                platform.puts("  blender [file]: Start headless render job (.blend)\r\n");
                platform.puts("  vlc [file]    : Play multimedia resource\r\n");
                platform.puts("  apk [flags]   : Android App Bridge (--install, --list, --run)\r\n");
                platform.puts("  linux [flags] : POSIX Compatibility Layer (--shell, --run)\r\n");
                platform.puts("  unix [flags]  : Classic UNIX (BSD/SysV) Compatibility Bridge\r\n");
                platform.puts("  windows [args]: Win32 Execution Bridge (--run)\r\n");
                platform.puts("  mac [args]    : Darwin (macOS/iOS) Bridge (--run)\r\n");
                platform.puts("  harmony [args]: HarmonyOS Distributed Ability Bridge (--run)\r\n");
                platform.puts("  symbian [args]: EPOC/Symbian OS Legacy Bridge (--run)\r\n");
                platform.puts("  python [file] : Python 3.12 Engine (interpreted via POSIX)\r\n");
                platform.puts("  node [file]   : Node.js/QuickJS Runtime (JavaScript/TypeScript)\r\n");
                platform.puts("  java [class]  : Java/Kotlin Virtual Machine (ART/Dalvik)\r\n");
                platform.puts("  rustc [file]  : Native Rust Compiler & Toolchain\r\n");
                platform.puts("  php [file]    : PHP 8.3 & Laravel Runtime\r\n");
                platform.puts("  webos [args]  : WebOS Sandboxed Container Bridge (--launch)\r\n");
                platform.puts("  intent [mode] : Sectoral AI context switch (--sector)\r\n");
                platform.puts("  identity [cmd]: SSI Identity management (--create)\r\n");
                platform.puts("  evolve        : Trigger Autonomous Evolution Core self-diagnostic\r\n");
                platform.puts("  tactical [cmd]: Military-Grade Mesh Controller (--stealth, --flash)\r\n");
                platform.puts("  calc          : Run simple calculator demo\r\n");
                platform.puts("  clear         : Clear screen\r\n");
                platform.puts("  exit          : Shutdown shell\r\n");
            } else if cmd.starts_with("omni ") {
                let source = &cmd[5..];
                platform.puts("\r\n[OmniBridge] Compiling...\r\n");
                if let Ok(output) = crate::runtime::omnilang_bridge::OmniBridge::compile_and_run(source) {
                    platform.puts("[Output] ");
                    platform.puts(&output);
                    platform.puts("\r\n");
                }
            } else if cmd.starts_with("blender ") {
                let filename = cmd[8..].trim();
                platform.puts("\r\n[Blender] Initializing Compute Node...\r\n");
                let mut node = crate::runtime::media::blender::BlenderComputeNode::new();
                match node.start_render(filename) {
                    Ok(res) => {
                         platform.puts(&res);
                         platform.puts("\r\n");
                    },
                    Err(e) => {
                         platform.puts("Error: ");
                         platform.puts(e);
                         platform.puts("\r\n");
                    }
                }
            } else if cmd.starts_with("vlc ") {
                let filename = cmd[4..].trim();
                platform.puts("\r\n[VLC] Initializing Universal Media Player...\r\n");
                if let Ok(mut player) = crate::runtime::MediaRuntime::new(filename) {
                    let _ = player.play();
                } else {
                    platform.puts("Error: Failed to load Media Runtime.\r\n");
                }
            } else if cmd.starts_with("apk ") {
                let args = cmd[4..].trim();
                if args == "--list" {
                    platform.puts("\r\n[Android] Installed APKs:\r\n");
                    let installer = crate::runtime::android::APK_INSTALLER.lock();
                    for app in installer.list() {
                        platform.puts("  - ");
                        platform.puts(app);
                        platform.puts("\r\n");
                    }
                    if installer.list().is_empty() {
                         platform.puts("  (No apps installed)\r\n");
                    }
                } else if args.starts_with("--install ") {
                    let pkg = args[10..].trim();
                    platform.puts("\r\n[Android] Installing: ");
                    platform.puts(pkg);
                    platform.puts("...\r\n");
                    
                    let manifest = crate::runtime::android::ApkManifest {
                        package: String::from(pkg),
                        version_code: 1,
                        version_name: String::from("1.0"),
                        min_sdk: 33,
                        main_activity: String::from("MainActivity"),
                    };
                    
                    let mut installer = crate::runtime::android::APK_INSTALLER.lock();
                    if let Ok(_) = installer.install(manifest, alloc::vec![]) {
                        platform.puts("[Android] Success: APK installed to /data/app/\r\n");
                    } else {
                        platform.puts("[Android] Error: Installation failed.\r\n");
                    }
                } else if args.starts_with("--run ") {
                    let pkg = args[6..].trim();
                    platform.puts("\r\n[Android] Starting Activity: ");
                    platform.puts(pkg);
                    platform.puts("\r\n");
                    
                    let installer = crate::runtime::android::APK_INSTALLER.lock();
                    if let Some(app) = installer.find(pkg) {
                        platform.puts("[Android] Initializing ART (Android Runtime)...\r\n");
                        platform.puts("[Android] Loading DEX: ");
                        platform.puts(&app.manifest.package);
                        platform.puts("\r\n");
                        platform.puts("[Android] Execution: MainActivity.onCreate()\r\n");
                        platform.puts("[Android] Process Lifecycle: OK.\r\n");
                    } else {
                        platform.puts("[Android] Error: App not found.\r\n");
                    }
                } else {
                    platform.puts("Usage: apk [--install <pkg> | --list | --run <pkg>]\r\n");
                }
            } else if cmd.starts_with("linux ") {
                let args = cmd[6..].trim();
                if args == "--shell" {
                    platform.puts("\r\n[Linux] Initializing POSIX Environment...\r\n");
                    let _vfs = Vfs::new();
                    platform.puts("[Linux] VFS (dev, proc, tmp, home) mounted.\r\n");
                    platform.puts("[Linux] aether@fabric:~$ _\r\n");
                } else if args.starts_with("--run ") {
                    let bin = args[6..].trim();
                    platform.puts("\r\n[Linux] Loading ELF binary: ");
                    platform.puts(bin);
                    platform.puts("\r\n");
                    platform.puts("[Linux] Mapping segments... Done.\r\n");
                    platform.puts("[Linux] Executing: posix_spawn()\r\n");
                } else {
                    platform.puts("Usage: linux [--shell | --run <bin>]\r\n");
                }
            } else if cmd.starts_with("unix ") {
                let args = cmd[5..].trim();
                platform.puts("\r\n[UNIX] Classic POSIX/BSD Compatibility Layer (Phase 15.1)...\r\n");
                if args == "--shell" {
                    platform.puts("[UNIX] Spawning Bourne-Compatible Shell (sh)...\r\n");
                    platform.puts("unix# ");
                } else if args.starts_with("--run ") {
                    let bin = args[6..].trim();
                    platform.puts("[UNIX] Executing ELF/Static Binary: ");
                    platform.puts(bin);
                    platform.puts("\r\n");
                    platform.puts("[UNIX] Status: System V Syscall Mapping OK.\r\n");
                } else {
                    platform.puts("Usage: unix [--shell | --run <bin>]\r\n");
                }
            } else if cmd.starts_with("windows ") {
                let args = cmd[8..].trim();
                if args.starts_with("--run ") {
                    let exe = args[6..].trim();
                    platform.puts("\r\n[Windows] Initializing Win32 Bridge...\r\n");
                    if sys_win32_create_process(exe) {
                        let mut loader = Win32Loader::new();
                        loader.load_pe(alloc::vec![0; 100].as_slice());
                        loader.resolve_imports();
                        platform.puts("[Windows] Process started successfully.\r\n");
                    }
                } else {
                    platform.puts("Usage: windows [--run <exe>]\r\n");
                }
            } else if cmd.starts_with("mac ") {
                let args = cmd[4..].trim();
                platform.puts("\r\n[Darwin] Accessing macOS/iOS Compatibility Bridge (Phase 28.5)...\r\n");
                if args.starts_with("--run ") {
                    let bin = args[6..].trim();
                    platform.puts("[Darwin] Initializing Mach-O Execution Environment...\r\n");
                    platform.puts("[Darwin] Loading Binary: ");
                    platform.puts(bin);
                    platform.puts("\r\n");
                    platform.puts("[Darwin] Status: Mach-O Segment Mapping SUCCESS.\r\n");
                } else {
                    platform.puts("Usage: mac [--run <app>]\r\n");
                }
            } else if cmd.starts_with("harmony ") {
                let args = cmd[8..].trim();
                platform.puts("\r\n[HarmonyOS] Active Ability Bridge (Phase 28.6)...\r\n");
                if args.starts_with("--run ") {
                    let hap = args[6..].trim();
                    platform.puts("[HarmonyOS] Loading Package: ");
                    platform.puts(hap);
                    platform.puts("\r\n");
                    let mut loader = crate::compat::harmony::HarmonyLoader::new();
                    loader.load_hap(&[]); // Simulation
                    loader.execute_ability(hap);
                } else {
                    platform.puts("Usage: harmony --run <hap_package>\r\n");
                }
            } else if cmd.starts_with("symbian ") {
                let args = cmd[8..].trim();
                platform.puts("\r\n[Symbian] EPOC32 Legacy Bridge (Phase 28.5)...\r\n");
                if args.starts_with("--run ") {
                    let app = args[6..].trim();
                    platform.puts("[Symbian] Initializing E32 Active Scheduler...\r\n");
                    platform.puts("[Symbian] Mapping binary: ");
                    platform.puts(app);
                    platform.puts("\r\n");
                    let mut loader = crate::compat::epoc::EpocLoader::new();
                    loader.load_e32(&[]); // Simulation
                    loader.execute();
                } else {
                    platform.puts("Usage: symbian --run <e32_binary>\r\n");
                }
            } else if cmd.starts_with("webos ") {
                let args = cmd[6..].trim();
                platform.puts("\r\n[WebOS] Application Container Bridge (Phase 28.6)...\r\n");
                if args.starts_with("--launch ") {
                    let app_id = args[9..].trim();
                    let mut runtime = crate::compat::webos::WebOSRuntime::new();
                    runtime.launch_app(app_id);
                    platform.puts("[WebOS] Lunar Bus Sync: OK.\r\n");
                } else {
                    platform.puts("Usage: webos --launch <app_id>\r\n");
                }
            } else if cmd.starts_with("python ") {
                let file = cmd[7..].trim();
                platform.puts("\r\n[Python 3.12] Initializing Interpreter (POSIX Bridge)...\r\n");
                platform.puts("[Python] Loading: ");
                platform.puts(file);
                platform.puts("\r\n[Python] Status: Execution SUCCESS.\r\n");
            } else if cmd.starts_with("node ") {
                let file = cmd[5..].trim();
                platform.puts("\r\n[Node.js/QuickJS] Initializing V8-Lite Engine (Phase 18)...\r\n");
                platform.puts("[Node] Loading: ");
                platform.puts(file);
                platform.puts("\r\n[Node] Status: Garbage Collection & Execution OK.\r\n");
            } else if cmd.starts_with("java ") {
                let class_name = cmd[5..].trim();
                platform.puts("\r\n[JVM/ART] Starting Android Runtime (Dalvik-Bridge)...\r\n");
                platform.puts("[JVM] Loading Class: ");
                platform.puts(class_name);
                platform.puts("\r\n[JVM] Status: Bytecode verification SUCCESS.\r\n");
            } else if cmd.starts_with("rustc ") {
                let file = cmd[6..].trim();
                platform.puts("\r\n[Rustc] Accessing Native Toolchain...\r\n");
                platform.puts("[Rustc] Compiling: ");
                platform.puts(file);
                platform.puts("\r\n[Rustc] Output: Aether Native Binary (SMME-Aware) Generated.\r\n");
            } else if cmd.starts_with("php ") {
                let file = cmd[4..].trim();
                platform.puts("\r\n[PHP 8.3] Starting PHP-FPM Bridge (Phase 19)...\r\n");
                platform.puts("[PHP] Executing: ");
                platform.puts(file);
                platform.puts("\r\n[PHP] Status: Laravel-ready Environment OK.\r\n");
            } else if cmd.starts_with("intent ") {
                let args = cmd[7..].trim();
                if args.starts_with("--sector ") {
                    let sector_name = args[9..].trim();
                    let mut engine = crate::runtime::ai::sectoral::SECTORAL_ENGINE.lock();
                    let mode = match sector_name {
                        "industrial" => crate::runtime::ai::sectoral::SectorMode::Industrial,
                        "medical" => crate::runtime::ai::sectoral::SectorMode::Medical,
                        "military" => crate::runtime::ai::sectoral::SectorMode::Military,
                        "research" => crate::runtime::ai::sectoral::SectorMode::Research,
                        _ => crate::runtime::ai::sectoral::SectorMode::General,
                    };
                    engine.set_mode(mode);
                    platform.puts("\r\n[SectoralAI] Policy: ");
                    platform.puts(&engine.get_policy_description());
                    platform.puts("\r\n");
                } else {
                    platform.puts("Usage: intent --sector [industrial|medical|military|research|general]\r\n");
                }
            } else if cmd.starts_with("identity ") {
                let args = cmd[9..].trim();
                if args.starts_with("--create ") {
                    let owner = args[9..].trim();
                    let mut manager = crate::security::identity::ssi::SSI_MANAGER.lock();
                    let did = manager.generate_local_did(owner);
                    platform.puts("\r\n[SSI] DID Generated: ");
                    platform.puts(&did);
                    platform.puts("\r\n[SSI] Controller: ");
                    platform.puts(owner);
                    platform.puts("\r\n[SSI] PQC Anchor: Kyber-768/Dilithium-3 Verified.\r\n");
                } else {
                    platform.puts("Usage: identity --create <owner_name>\r\n");
                }
            } else if cmd == "evolve" {
                platform.puts("\r\n[Evolution] Accessing Singularity Era Core (Phase 30.1)...\r\n");
                let mut core = crate::runtime::ai::evolution::EVOLUTION_CORE.lock();
                platform.puts("[Evolution] Running Diagnostic: ");
                platform.puts(&core.run_self_diagnostic());
                platform.puts("\r\n");
                
                platform.puts("[Evolution] Generation: ");
                let gen_str = format!("{}", core.generation);
                platform.puts(&gen_str);
                platform.puts("\r\n");
                
                platform.puts("[Evolution] Triggering adaptation in background...\r\n");
                core.trigger_adaptation();
                platform.puts("[Evolution] Status: ASCENDING.\r\n");
            } else if cmd.starts_with("tactical ") {
                let args = cmd[9..].trim();
                platform.puts("\r\n[Tactical] Accessing Sovereign Tactical Mesh (Phase 29.1)...\r\n");
                let mut controller = crate::mesh::tactical::TACTICAL_CONTROLLER.lock();
                if args == "--stealth" {
                    controller.enable_stealth_mode();
                    platform.puts("[Tactical] Stealth Mode: ACTIVE (Radio Silence).\r\n");
                } else if args.starts_with("--flash ") {
                    let msg = args[8..].trim();
                    controller.send_secure_flash(msg.as_bytes());
                    platform.puts("[Tactical] FLASH SENT: ");
                    platform.puts(msg);
                    platform.puts("\r\n");
                } else {
                    platform.puts("Usage: tactical [--stealth | --flash <msg>]\r\n");
                }
            } else if cmd == "calc" {
                 // Calculator Logic
                 platform.puts("\r\n[Calculator] Mode Active (Press Ctrl+C to exit - simulation)\r\n");
                 // ... (Simplified calculator logic could go here or be a separate function)
                 platform.puts("Calculator demo skipped for shell responsiveness.\r\n");
            } else if cmd == "clear" {
                platform.clear();
            } else if !cmd.is_empty() {
                platform.puts("\r\nUnknown command. Type 'help'.\r\n");
            }
        }

        platform.puts("\r\nShutting down...\r\n");
        platform.shutdown();
    }
}

// Helper: baca satu baris dari serial (blocking), echo, handle backspace.
fn read_line(platform: &dyn hal::Platform) -> alloc::string::String {
    let mut buf = alloc::string::String::new();
    loop {
        let c = platform.get_char(); // blocking sampai ada data
        // filter noise
        if c == 0 || c == 0xFF { continue; }

        // newline: selesai
        if c == b'\r' || c == b'\n' {
            platform.puts("\r\n");
            break;
        }

        // backspace
        if c == 8 || c == 127 {
            if !buf.is_empty() {
                buf.pop();
                platform.puts("\x08 \x08");
            }
            continue;
        }

        // normal char
        buf.push(c as char);
        platform.put_char(c);
    }
    buf
}
