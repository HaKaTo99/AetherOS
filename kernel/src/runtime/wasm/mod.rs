pub mod executor;
pub mod legacy_runtime;

// Re-export legacy types to maintain compatibility and fix build errors
pub use executor::WasmExecutor;
pub use legacy_runtime::{WasmRuntime, WasmModule, WasmValue, WasmType, WasmInstr, WasmFunc, WasmInterpreter, WASM_STORE, WasmAppStore};
