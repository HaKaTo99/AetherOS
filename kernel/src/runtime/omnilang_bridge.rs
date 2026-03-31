//! OmniLang Bridge (Phase 3 Integration)
//!
//! Connects the kernel runtime to the external OmniLang compiler source.
//! This acts as the Foreign Function Interface (FFI) to D:\GitHub\OmniLang.

use alloc::string::String;

pub struct OmniBridge;

impl OmniBridge {
    /// invokes the external OmniLang compiler directly
    pub fn compile_and_run(source: &str) -> Result<String, &'static str> {
        // In a real scenario, this would call functions from the linked crate.
        // For v10.1 stabilization, we use the internal runtime as a proxy
        // until the external crate is fully vendored.
        
        crate::println!("[Bridge] Delegating to internal runtime...");
        
        // Create a temporary runtime just for this execution
        let mut runtime = crate::runtime::OmniRuntime::new();
        runtime.execute(source);
        
        Ok(runtime.last_output)
    }
}
