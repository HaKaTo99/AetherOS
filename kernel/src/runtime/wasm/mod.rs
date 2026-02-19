pub mod executor;
pub mod legacy_runtime;

// Re-export legacy types to maintain compatibility and fix build errors
pub use legacy_runtime::{WasmRuntime, WasmModule, WasmValue, WasmInstr, WasmFunc, WASM_STORE, WasmAppStore};
pub use executor::WasmExecutor;
