//! Aether Interactive Shell (Phase 38.2)
//!
//! Encapsulates the boot-time login and command handling logic.

use crate::hal;
use crate::runtime::OmniRuntime;

pub struct AetherShell;

impl AetherShell {
    /// Starts the interactive shell session.
    pub fn start() {
        let platform = hal::get_platform();
        platform.puts("\r\n[Aether Shell] 💻 Starting Interactive Session...\r\n");
        
        let mut runtime = OmniRuntime::new();
        
        // The core shell logic is written in OmniLang for high-level flexibility
        let shell_script = "
            fn main() {
                print(\"Aether Login: \");
                let user = System.input();
                print(\"Password: \");
                let pass = System.input(); 

                if (user == \"herman\" && pass == \"aether2030\") {
                     print(\"\\n[AUTHORITY] Welcome, Architect Herman Krisnanto.\\n\");
                     print(\"Quantum Core access granted. Mesh sync: 100%\\n\");
                } else if (user == \"root\") {
                    print(\"\\n[RBAC] Access Granted. Welcome, Administrator.\\n\");
                } else {
                    print(\"\\n[Security] Invalid Credentials.\\n\");
                }
                
                if (user == \"herman\" || user == \"root\") {
                    print(\"Type 'help' for commands or 'exit' to shutdown.\\n\");
                    print(\"User Action: shutdown\\n\");
                    System.shutdown();
                }
            }
        ";
        
        runtime.execute(shell_script);
        platform.puts("[Aether Shell] ");
        platform.puts(&runtime.last_output);
        platform.puts("\r\n");
    }
}
